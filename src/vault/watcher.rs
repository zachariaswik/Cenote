//! File watcher — batches filesystem events and re-indexes affected files.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult};
use tokio::sync::mpsc;

use super::scanner;
use crate::db::Database;

pub enum WatcherEvent {
    Changed(PathBuf),
    Removed(PathBuf),
    Error(String),
}

pub fn spawn(db: Database, root: PathBuf) -> Result<mpsc::Receiver<WatcherEvent>> {
    let (tx, rx) = mpsc::channel::<WatcherEvent>(256);
    let tx_inner = tx.clone();
    std::thread::spawn(move || {
        let runtime = match tokio::runtime::Handle::try_current() {
            Ok(h) => Some(h),
            Err(_) => None,
        };
        let tx_for_events = tx_inner.clone();
        let mut debouncer = match new_debouncer(
            Duration::from_millis(400),
            None,
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    for ev in events {
                        for path in &ev.event.paths {
                            let p = path.clone();
                            let msg = if p.exists() {
                                WatcherEvent::Changed(p)
                            } else {
                                WatcherEvent::Removed(p)
                            };
                            let _ = tx_for_events.blocking_send(msg);
                        }
                    }
                }
                Err(errs) => {
                    for e in errs {
                        let _ = tx_for_events.blocking_send(WatcherEvent::Error(e.to_string()));
                    }
                }
            },
        ) {
            Ok(d) => d,
            Err(e) => {
                let _ = tx_inner.blocking_send(WatcherEvent::Error(e.to_string()));
                return;
            }
        };
        if let Err(e) = debouncer.watcher().watch(&root, RecursiveMode::Recursive) {
            let _ = tx_inner.blocking_send(WatcherEvent::Error(e.to_string()));
            return;
        }
        // Keep the debouncer alive for as long as this thread runs.
        // Park forever — runtime drop will clean up via process exit.
        let _ = runtime; // suppress unused warning in non-async contexts
        loop {
            std::thread::park();
        }
    });

    // Re-index events into the DB on a background task.
    let db_clone = db.clone();
    let (out_tx, out_rx) = mpsc::channel::<WatcherEvent>(256);
    let mut rx = rx;
    tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            match &ev {
                WatcherEvent::Changed(p) => {
                    if matches!(
                        p.extension().and_then(|s| s.to_str()).unwrap_or(""),
                        "md" | "markdown"
                    ) {
                        if let Err(e) = scanner::index_file(&db_clone, p) {
                            tracing::warn!(path=%p.display(), error=%e, "watcher reindex failed");
                        }
                    }
                }
                WatcherEvent::Removed(p) => {
                    if let Err(e) = scanner::remove_file(&db_clone, p) {
                        tracing::warn!(path=%p.display(), error=%e, "watcher delete failed");
                    }
                }
                WatcherEvent::Error(e) => {
                    tracing::warn!(%e, "watcher error");
                }
            }
            let _ = out_tx.send(ev).await;
        }
    });
    Ok(out_rx)
}
