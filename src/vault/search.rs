//! Search over vault notes: keyword (FTS5), semantic (vector), and hybrid.

use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub note_id: String,
    pub path: String,
    pub title: Option<String>,
    pub section: Option<String>,
    pub chunk_id: String,
    pub excerpt: String,
    pub score: f32,
}

pub fn keyword(
    db: &Database,
    query: &str,
    limit: usize,
    path_prefix: Option<&str>,
) -> Result<Vec<SearchHit>> {
    let conn = db.conn()?;
    let fts_query = sanitize_fts(query);
    let mut sql = String::from(
        "SELECT n.id, n.path, n.title, c.id, c.section, c.text, bm25(note_chunks_fts) AS rank
         FROM note_chunks_fts
         JOIN note_chunks c ON c.rowid = note_chunks_fts.rowid
         JOIN notes n ON n.id = c.note_id
         WHERE note_chunks_fts MATCH ?1",
    );
    if path_prefix.is_some() {
        sql.push_str(" AND n.path LIKE ?2");
    }
    sql.push_str(" ORDER BY rank ASC LIMIT ?");
    sql.push_str(&((if path_prefix.is_some() { 3 } else { 2 })).to_string());

    let mut stmt = conn.prepare(&sql)?;
    let mut rows = if let Some(prefix) = path_prefix {
        let like = format!("{prefix}%");
        stmt.query(params![fts_query, like, limit as i64])?
    } else {
        stmt.query(params![fts_query, limit as i64])?
    };
    let mut hits = Vec::new();
    while let Some(r) = rows.next()? {
        let rank: f64 = r.get(6)?;
        // bm25 lower is better; invert + normalize roughly to 0..1.
        let score = (1.0 / (1.0 + rank.max(0.0))) as f32;
        let text: String = r.get(5)?;
        hits.push(SearchHit {
            note_id: r.get(0)?,
            path: r.get(1)?,
            title: r.get(2)?,
            section: r.get(4)?,
            chunk_id: r.get(3)?,
            excerpt: excerpt(&text, query, 240),
            score,
        });
    }
    Ok(hits)
}

pub async fn semantic(
    state: &AppState,
    query: &str,
    limit: usize,
    path_prefix: Option<&str>,
) -> Result<Vec<SearchHit>> {
    let vec = match state.ollama.embed(query).await {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(error=%e, "embedding unavailable; falling back to keyword");
            return keyword(&state.db, query, limit, path_prefix);
        }
    };
    let hits = state
        .vector
        .query(&vec, Some("note_chunk"), limit * 2)
        .await?;
    let conn = state.db.conn()?;
    let mut out = Vec::new();
    for h in hits {
        let row: Option<(String, String, Option<String>, String, Option<String>, String)> = conn
            .query_row(
                "SELECT n.id, n.path, n.title, c.id, c.section, c.text
                 FROM note_chunks c JOIN notes n ON n.id = c.note_id
                 WHERE c.id = ?1",
                [&h.ref_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
            )
            .ok();
        if let Some((note_id, path, title, chunk_id, section, text)) = row {
            if let Some(pfx) = path_prefix {
                if !path.starts_with(pfx) {
                    continue;
                }
            }
            out.push(SearchHit {
                note_id,
                path,
                title,
                section,
                chunk_id,
                excerpt: excerpt(&text, query, 240),
                score: h.score,
            });
            if out.len() >= limit {
                break;
            }
        }
    }
    Ok(out)
}

pub async fn hybrid(
    state: &AppState,
    query: &str,
    limit: usize,
    path_prefix: Option<&str>,
) -> Result<Vec<SearchHit>> {
    let kw = keyword(&state.db, query, limit * 2, path_prefix).unwrap_or_default();
    let sem = semantic(state, query, limit * 2, path_prefix)
        .await
        .unwrap_or_default();
    // Merge by chunk_id, summing normalized scores.
    let mut by_chunk: std::collections::HashMap<String, SearchHit> = Default::default();
    for h in kw {
        let s = h.score * 0.5;
        by_chunk
            .entry(h.chunk_id.clone())
            .and_modify(|existing| existing.score += s)
            .or_insert_with(|| SearchHit { score: s, ..h });
    }
    for h in sem {
        let s = h.score * 0.5;
        by_chunk
            .entry(h.chunk_id.clone())
            .and_modify(|existing| existing.score += s)
            .or_insert_with(|| SearchHit { score: s, ..h });
    }
    let mut merged: Vec<SearchHit> = by_chunk.into_values().collect();
    merged.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    merged.truncate(limit);
    Ok(merged)
}

/// Minimal FTS sanitizer: escape double-quotes, phrase the query. Keeps us
/// from exploding on user input like `foo:bar` or `"unbalanced`.
fn sanitize_fts(query: &str) -> String {
    let cleaned = query.replace('"', " ");
    format!("\"{}\"", cleaned.trim())
}

fn excerpt(text: &str, query: &str, max_len: usize) -> String {
    let lower = text.to_lowercase();
    let q = query.to_lowercase();
    let start = lower.find(&q).map(|i| i.saturating_sub(60)).unwrap_or(0);
    let end = (start + max_len).min(text.len());
    let snippet = &text[start..end];
    let prefix = if start > 0 { "…" } else { "" };
    let suffix = if end < text.len() { "…" } else { "" };
    format!("{prefix}{snippet}{suffix}").replace('\n', " ")
}
