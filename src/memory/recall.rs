//! `recall_memories` — hybrid memory retrieval with optional channel/date filters.

use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHit {
    pub message_id: String,
    pub session_id: String,
    pub channel: String,
    pub role: String,
    pub created_at: String,
    pub excerpt: String,
    pub score: f32,
}

pub async fn recall(
    state: &AppState,
    query: &str,
    limit: usize,
    channel: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<Vec<MemoryHit>> {
    let mut by_id: std::collections::HashMap<String, MemoryHit> = Default::default();

    // Semantic leg.
    if let Ok(vec) = state.ollama.embed(query).await {
        if let Ok(hits) = state.vector.query(&vec, Some("message"), limit * 3).await {
            for h in hits {
                if let Some(hit) = hydrate(state, &h.ref_id, h.score * 0.6, channel, since, until)? {
                    merge_hit(&mut by_id, hit);
                }
            }
        }
    }

    // Keyword leg (FTS).
    let conn = state.db.conn()?;
    let fts_query = format!("\"{}\"", query.replace('"', " "));
    let mut sql = String::from(
        "SELECT m.id, m.session_id, s.channel, m.role, m.created_at, m.content, bm25(conversation_messages_fts) AS rank
         FROM conversation_messages_fts
         JOIN conversation_messages m ON m.rowid = conversation_messages_fts.rowid
         JOIN conversation_sessions s ON s.id = m.session_id
         WHERE conversation_messages_fts MATCH ?1",
    );
    let mut args: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(fts_query)];
    if let Some(c) = channel {
        sql.push_str(" AND s.channel = ?");
        args.push(Box::new(c.to_string()));
    }
    if let Some(s) = since {
        sql.push_str(" AND m.created_at >= ?");
        args.push(Box::new(s.to_string()));
    }
    if let Some(u) = until {
        sql.push_str(" AND m.created_at <= ?");
        args.push(Box::new(u.to_string()));
    }
    sql.push_str(" ORDER BY rank ASC LIMIT ?");
    args.push(Box::new((limit * 2) as i64));

    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params_from_iter(args.iter().map(|b| b.as_ref())))?;
    while let Some(r) = rows.next()? {
        let rank: f64 = r.get(6)?;
        let score = (1.0 / (1.0 + rank.max(0.0))) as f32 * 0.4;
        let content: String = r.get(5)?;
        let hit = MemoryHit {
            message_id: r.get(0)?,
            session_id: r.get(1)?,
            channel: r.get(2)?,
            role: r.get(3)?,
            created_at: r.get(4)?,
            excerpt: excerpt(&content, query),
            score,
        };
        merge_hit(&mut by_id, hit);
    }

    let mut out: Vec<MemoryHit> = by_id.into_values().collect();
    out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    out.truncate(limit);
    Ok(out)
}

fn hydrate(
    state: &AppState,
    message_id: &str,
    score: f32,
    channel: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<Option<MemoryHit>> {
    let conn = state.db.conn()?;
    let row: Option<(String, String, String, String, String, String)> = conn
        .query_row(
            "SELECT m.id, m.session_id, s.channel, m.role, m.created_at, m.content
             FROM conversation_messages m JOIN conversation_sessions s ON s.id = m.session_id
             WHERE m.id = ?1",
            params![message_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
        )
        .ok();
    let Some((id, session_id, ch, role, created, content)) = row else {
        return Ok(None);
    };
    if let Some(c) = channel {
        if ch != c {
            return Ok(None);
        }
    }
    if let Some(s) = since {
        if created.as_str() < s {
            return Ok(None);
        }
    }
    if let Some(u) = until {
        if created.as_str() > u {
            return Ok(None);
        }
    }
    Ok(Some(MemoryHit {
        message_id: id,
        session_id,
        channel: ch,
        role,
        created_at: created,
        excerpt: excerpt(&content, ""),
        score,
    }))
}

fn merge_hit(map: &mut std::collections::HashMap<String, MemoryHit>, hit: MemoryHit) {
    map.entry(hit.message_id.clone())
        .and_modify(|existing| existing.score += hit.score)
        .or_insert(hit);
}

fn excerpt(text: &str, query: &str) -> String {
    let max_len = 220;
    let trimmed: String = text.chars().take(max_len * 2).collect();
    if query.is_empty() {
        let chars: String = trimmed.chars().take(max_len).collect();
        return chars.replace('\n', " ");
    }
    let lower = trimmed.to_lowercase();
    let q = query.to_lowercase();
    let start = lower.find(&q).map(|i| i.saturating_sub(60)).unwrap_or(0);
    let end = (start + max_len).min(trimmed.len());
    trimmed[start..end].replace('\n', " ")
}
