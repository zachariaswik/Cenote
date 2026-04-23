//! SQLite/LibSQL-compatible storage wrapper.
//!
//! Uses rusqlite with a small r2d2 pool. The schema lives in
//! `migrations/` as plain SQL and is applied via `run_migrations`. Every DDL
//! is idempotent (`IF NOT EXISTS`) so re-running `migrate` is safe.

pub mod migrations;

use std::path::Path;

use anyhow::{Context, Result};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;

pub type Conn = PooledConnection<SqliteConnectionManager>;

#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating db parent {parent:?}"))?;
            }
        }
        let manager = SqliteConnectionManager::file(&path).with_flags(
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI,
        );
        let pool = Pool::builder()
            .max_size(8)
            .build(manager)
            .context("building sqlite pool")?;
        // Pragmas per-connection: WAL + foreign keys.
        let conn = pool.get()?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        Ok(Self { pool })
    }

    pub fn in_memory() -> Result<Self> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder().max_size(1).build(manager)?;
        let conn = pool.get()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        Ok(Self { pool })
    }

    pub fn conn(&self) -> Result<Conn> {
        Ok(self.pool.get()?)
    }

    pub fn run_migrations(&self) -> Result<()> {
        migrations::run(&mut self.conn()?)
    }
}
