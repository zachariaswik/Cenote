//! Recursive scan + checksum + DB upsert. Idempotent: unchanged files are
//! skipped by checksum match.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use super::chunker;
use crate::db::Database;
use crate::tasks;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanReport {
    pub scanned: usize,
    pub indexed: usize,
    pub skipped: usize,
    pub removed: usize,
}

/// Scan the given vault path, upsert notes and chunks, and reconcile deletes.
pub fn scan(db: &Database, vault: &Path, sidecar_dir: &str) -> Result<ScanReport> {
    let mut report = ScanReport::default();
    let mut seen = Vec::<String>::new();

    for entry in WalkDir::new(vault)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        // Skip sidecar files.
        if path
            .components()
            .any(|c| c.as_os_str() == sidecar_dir)
        {
            continue;
        }
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown") {
            continue;
        }
        report.scanned += 1;
        let path_str = path.to_string_lossy().to_string();
        seen.push(path_str.clone());
        let changed = upsert_note(db, path, &path_str)?;
        if changed {
            report.indexed += 1;
        } else {
            report.skipped += 1;
        }
    }

    report.removed = reconcile_deletes(db, vault, &seen)?;
    Ok(report)
}

/// Index a single file (used by the watcher). Returns true if contents changed.
pub fn index_file(db: &Database, path: &Path) -> Result<bool> {
    let path_str = path.to_string_lossy().to_string();
    upsert_note(db, path, &path_str)
}

pub fn remove_file(db: &Database, path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();
    let conn = db.conn()?;
    let id: Option<String> = conn
        .query_row(
            "SELECT id FROM notes WHERE path = ?1",
            [&path_str],
            |r| r.get(0),
        )
        .ok();
    if let Some(id) = id {
        delete_chunks_fts(&conn, &id)?;
        conn.execute("DELETE FROM notes WHERE id = ?1", [&id])?;
    }
    Ok(())
}

fn reconcile_deletes(db: &Database, vault: &Path, seen: &[String]) -> Result<usize> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare("SELECT id, path FROM notes")?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    drop(stmt);
    let mut removed = 0;
    for (id, path) in rows {
        let keep_if_outside_vault = !PathBuf::from(&path).starts_with(vault);
        if keep_if_outside_vault {
            continue;
        }
        if !seen.iter().any(|s| s == &path) {
            delete_chunks_fts(&conn, &id)?;
            conn.execute("DELETE FROM notes WHERE id = ?1", [&id])?;
            removed += 1;
        }
    }
    Ok(removed)
}

fn delete_chunks_fts(conn: &rusqlite::Connection, note_id: &str) -> Result<()> {
    // Rebuild FTS entries via the chunks deletion cascade; we also prune FTS manually.
    conn.execute(
        "DELETE FROM note_chunks_fts WHERE note_id = ?1",
        [note_id],
    )?;
    conn.execute("DELETE FROM note_chunks WHERE note_id = ?1", [note_id])?;
    Ok(())
}

fn upsert_note(db: &Database, path: &Path, path_str: &str) -> Result<bool> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {path:?}"))?;
    let checksum = blake3::hash(&bytes).to_hex().to_string();
    let modified = std::fs::metadata(path)?
        .modified()
        .ok()
        .and_then(|m| m.elapsed().ok())
        .map(|e| Utc::now() - chrono::Duration::from_std(e).unwrap_or_default())
        .unwrap_or_else(Utc::now)
        .to_rfc3339();

    let conn = db.conn()?;
    let existing: Option<(String, String)> = conn
        .query_row(
            "SELECT id, checksum FROM notes WHERE path = ?1",
            [path_str],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .ok();

    if let Some((_id, old_checksum)) = &existing {
        if old_checksum == &checksum {
            return Ok(false);
        }
    }

    let note_id = existing
        .as_ref()
        .map(|(id, _)| id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let text = String::from_utf8_lossy(&bytes).to_string();
    let title = chunker::extract_title(&text);
    let tags = chunker::extract_tags(&text);
    let tags_json = serde_json::to_string(&tags).unwrap_or_else(|_| "[]".into());
    let byte_size = bytes.len() as i64;

    conn.execute(
        "INSERT INTO notes(id, path, title, checksum, modified_at, byte_size, tags_json, status)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, 'indexed')
         ON CONFLICT(path) DO UPDATE SET
             title=excluded.title,
             checksum=excluded.checksum,
             modified_at=excluded.modified_at,
             byte_size=excluded.byte_size,
             tags_json=excluded.tags_json,
             status='indexed',
             updated_at=datetime('now')",
        params![
            note_id,
            path_str,
            title,
            checksum,
            modified,
            byte_size,
            tags_json
        ],
    )?;

    // Replace chunks + FTS entries for this note.
    delete_chunks_fts(&conn, &note_id)?;
    let chunks = chunker::chunk_markdown(&note_id, &text);
    for c in &chunks {
        conn.execute(
            "INSERT INTO note_chunks(id, note_id, ord, section, byte_start, byte_end, text)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![c.id, c.note_id, c.ord, c.section, c.byte_start, c.byte_end, c.text],
        )?;
        conn.execute(
            "INSERT INTO note_chunks_fts(rowid, text, section, note_id)
             VALUES((SELECT rowid FROM note_chunks WHERE id = ?1), ?2, ?3, ?4)",
            params![c.id, c.text, c.section, c.note_id],
        )?;
    }

    // Mine TODOs from the markdown as tasks with a stable dedupe key.
    let todos = chunker::extract_todos(&text);
    for (line, title) in todos {
        let dedupe_key = format!("note:{note_id}:{line}");
        tasks::upsert(
            &conn,
            &tasks::TaskUpsert {
                title,
                details: None,
                status: "pending".into(),
                priority: None,
                due_at: None,
                source_kind: "note".into(),
                source_ref: Some(note_id.clone()),
                dedupe_key,
            },
        )?;
    }

    Ok(true)
}
