//! Embedded migrations. Each entry is `(id, sql)`; `id` is the unique,
//! monotonically-increasing migration name. Applied migrations are tracked
//! in `_cenote_migrations`.

use anyhow::{Context, Result};
use rusqlite::Connection;

const MIGRATIONS: &[(&str, &str)] = &[
    ("0001_init", include_str!("../../migrations/0001_init.sql")),
    (
        "0002_conversations",
        include_str!("../../migrations/0002_conversations.sql"),
    ),
    (
        "0003_ingestion",
        include_str!("../../migrations/0003_ingestion.sql"),
    ),
];

pub fn run(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _cenote_migrations (
            id TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )
    .context("creating migration ledger")?;

    for (id, sql) in MIGRATIONS {
        let already: bool = conn
            .query_row(
                "SELECT 1 FROM _cenote_migrations WHERE id = ?1",
                [*id],
                |_| Ok(true),
            )
            .unwrap_or(false);
        if already {
            tracing::debug!(migration = id, "skipped (already applied)");
            continue;
        }
        let tx = conn.transaction()?;
        tx.execute_batch(sql)
            .with_context(|| format!("applying migration {id}"))?;
        tx.execute(
            "INSERT INTO _cenote_migrations(id) VALUES(?1)",
            [*id],
        )?;
        tx.commit()?;
        tracing::info!(migration = id, "applied");
    }
    Ok(())
}
