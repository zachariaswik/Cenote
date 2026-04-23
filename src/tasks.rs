//! Task persistence. `dedupe_key` gives us idempotent upserts so repeated
//! scans / re-ingestions don't keep creating new rows for the same TODO.

use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub details: Option<String>,
    pub status: String,
    pub priority: Option<String>,
    pub due_at: Option<String>,
    pub source_kind: String,
    pub source_ref: Option<String>,
    pub dedupe_key: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct TaskUpsert {
    pub title: String,
    pub details: Option<String>,
    pub status: String,
    pub priority: Option<String>,
    pub due_at: Option<String>,
    pub source_kind: String,
    pub source_ref: Option<String>,
    pub dedupe_key: String,
}

pub fn upsert(conn: &Connection, t: &TaskUpsert) -> Result<String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM tasks WHERE dedupe_key = ?1",
            [&t.dedupe_key],
            |r| r.get(0),
        )
        .ok();
    if let Some(id) = existing {
        conn.execute(
            "UPDATE tasks SET title=?1, details=?2, priority=?3, due_at=?4, source_kind=?5, source_ref=?6, updated_at=datetime('now') WHERE id=?7",
            params![
                t.title,
                t.details,
                t.priority,
                t.due_at,
                t.source_kind,
                t.source_ref,
                id
            ],
        )?;
        return Ok(id);
    }
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO tasks(id, title, details, status, priority, due_at, source_kind, source_ref, dedupe_key)
         VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        params![
            id,
            t.title,
            t.details,
            t.status,
            t.priority,
            t.due_at,
            t.source_kind,
            t.source_ref,
            t.dedupe_key
        ],
    )?;
    Ok(id)
}

pub fn list(
    db: &Database,
    status: Option<&str>,
    due_before: Option<&str>,
    source: Option<&str>,
    limit: usize,
) -> Result<Vec<Task>> {
    let conn = db.conn()?;
    let mut sql = String::from(
        "SELECT id, title, details, status, priority, due_at, source_kind, source_ref, dedupe_key, created_at, updated_at FROM tasks WHERE 1=1",
    );
    let mut args: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    match status {
        Some("any") => {}
        Some(s) => {
            sql.push_str(" AND status = ?");
            args.push(Box::new(s.to_string()));
        }
        None => {
            sql.push_str(" AND status IN ('pending','in_progress')");
        }
    }
    if let Some(db_val) = due_before {
        sql.push_str(" AND due_at <= ?");
        args.push(Box::new(db_val.to_string()));
    }
    if let Some(src) = source {
        sql.push_str(" AND source_kind = ?");
        args.push(Box::new(src.to_string()));
    }
    sql.push_str(" ORDER BY CASE WHEN due_at IS NULL THEN 1 ELSE 0 END, due_at, created_at DESC LIMIT ?");
    args.push(Box::new(limit as i64));

    let mut stmt = conn.prepare(&sql)?;
    let params_iter = rusqlite::params_from_iter(args.iter().map(|b| b.as_ref()));
    let rows = stmt
        .query_map(params_iter, |r| {
            Ok(Task {
                id: r.get(0)?,
                title: r.get(1)?,
                details: r.get(2)?,
                status: r.get(3)?,
                priority: r.get(4)?,
                due_at: r.get(5)?,
                source_kind: r.get(6)?,
                source_ref: r.get(7)?,
                dedupe_key: r.get(8)?,
                created_at: r.get(9)?,
                updated_at: r.get(10)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(rows)
}
