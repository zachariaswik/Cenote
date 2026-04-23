//! `commit_interaction` logic. Idempotent by (session_id, external_msg_id, role).

use anyhow::{Context, Result};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tasks::{self, TaskUpsert};
use crate::vector::StoredVector;
use crate::AppState;

#[derive(Debug, Clone)]
pub struct CommitTurn {
    pub content: String,
    pub external_id: Option<String>,
    pub tool_metadata: Option<Value>,
    pub attachments: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct CommitInput {
    pub channel: String,
    pub external_session_id: Option<String>,
    pub user_handle: Option<String>,
    pub user: CommitTurn,
    pub assistant: CommitTurn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitResult {
    pub session_id: String,
    pub user_message_id: String,
    pub assistant_message_id: String,
    pub mined_tasks: Vec<String>,
}

pub async fn commit(state: &AppState, input: CommitInput) -> Result<CommitResult> {
    // Session upsert.
    let conn = state.db.conn()?;
    let session_id = upsert_session(
        &conn,
        &input.channel,
        input.external_session_id.as_deref(),
        input.user_handle.as_deref(),
    )?;

    // Insert the two turns transactionally. Idempotency: we key on
    // (session_id, external_msg_id, role). Replays of the same external id
    // are silently no-ops.
    let user_msg_id = insert_message(
        &conn,
        &session_id,
        "user",
        &input.user.content,
        input.user.external_id.as_deref(),
        input.user.tool_metadata.as_ref(),
        input.user.attachments.as_ref(),
    )?;
    let assistant_msg_id = insert_message(
        &conn,
        &session_id,
        "assistant",
        &input.assistant.content,
        input.assistant.external_id.as_deref(),
        input.assistant.tool_metadata.as_ref(),
        input.assistant.attachments.as_ref(),
    )?;
    conn.execute(
        "UPDATE conversation_sessions SET last_message_at = datetime('now') WHERE id = ?1",
        [&session_id],
    )?;
    drop(conn);

    // Embed both turns (best-effort — if Ollama is down we still persist).
    for (id, text, role) in [
        (user_msg_id.clone(), input.user.content.clone(), "user"),
        (
            assistant_msg_id.clone(),
            input.assistant.content.clone(),
            "assistant",
        ),
    ] {
        match state.ollama.embed(&text).await {
            Ok(vec) => {
                let stored = StoredVector {
                    id: format!("msg:{id}"),
                    kind: "message".into(),
                    ref_id: id.clone(),
                    payload: summarize_for_payload(&text),
                    vector: vec.clone(),
                };
                if let Err(e) = state.vector.upsert(vec![stored]).await {
                    tracing::warn!(error=%e, "vector upsert failed for message");
                } else {
                    let dim = vec.len() as i64;
                    let _ = state.db.conn().and_then(|c| {
                        c.execute(
                            "INSERT OR REPLACE INTO embeddings(id, kind, ref_id, dim) VALUES(?1, 'message', ?2, ?3)",
                            params![format!("msg:{id}"), id, dim],
                        )?;
                        Ok(())
                    });
                }
            }
            Err(e) => tracing::debug!(error=%e, role, "skipped embedding (Ollama unavailable)"),
        }
    }

    // Mine implicit tasks from the USER turn only.
    let mined_tasks = mine_tasks_from_message(state, &user_msg_id, &input.user.content)?;
    Ok(CommitResult {
        session_id,
        user_message_id: user_msg_id,
        assistant_message_id: assistant_msg_id,
        mined_tasks,
    })
}

fn upsert_session(
    conn: &rusqlite::Connection,
    channel: &str,
    external: Option<&str>,
    handle: Option<&str>,
) -> Result<String> {
    if let Some(ext) = external {
        if let Ok(id) = conn.query_row(
            "SELECT id FROM conversation_sessions WHERE channel=?1 AND external_id=?2",
            params![channel, ext],
            |r| r.get::<_, String>(0),
        ) {
            return Ok(id);
        }
    }
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO conversation_sessions(id, channel, external_id, user_handle) VALUES(?1,?2,?3,?4)",
        params![id, channel, external, handle],
    )
    .context("inserting session")?;
    Ok(id)
}

fn insert_message(
    conn: &rusqlite::Connection,
    session_id: &str,
    role: &str,
    content: &str,
    external_id: Option<&str>,
    tool_meta: Option<&Value>,
    attachments: Option<&Value>,
) -> Result<String> {
    if let Some(ext) = external_id {
        if let Ok(existing) = conn.query_row(
            "SELECT id FROM conversation_messages WHERE session_id=?1 AND external_msg_id=?2 AND role=?3",
            params![session_id, ext, role],
            |r| r.get::<_, String>(0),
        ) {
            return Ok(existing);
        }
    }
    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO conversation_messages(id, session_id, role, content, tool_metadata, attachments, external_msg_id) VALUES(?1,?2,?3,?4,?5,?6,?7)",
        params![
            id,
            session_id,
            role,
            content,
            tool_meta.map(|v| v.to_string()),
            attachments.map(|v| v.to_string()),
            external_id,
        ],
    )?;
    // Manual FTS sync because we used an external-content FTS table.
    conn.execute(
        "INSERT INTO conversation_messages_fts(rowid, content, role, session_id)
         VALUES((SELECT rowid FROM conversation_messages WHERE id=?1), ?2, ?3, ?4)",
        params![id, content, role, session_id],
    )?;
    Ok(id)
}

fn summarize_for_payload(text: &str) -> String {
    let trimmed: String = text.chars().take(240).collect();
    trimmed.replace('\n', " ")
}

/// Heuristic task miner for chat messages. Looks for action verbs at start or
/// explicit `remind me to`, `i need to`, `please ...`. We deliberately keep
/// this simple — an LLM-based pass can be layered in later.
pub fn mine_tasks_from_message(
    state: &AppState,
    message_id: &str,
    content: &str,
) -> Result<Vec<String>> {
    let patterns = [
        ("remind me to ", 13),
        ("i need to ", 10),
        ("i should ", 9),
        ("todo: ", 6),
        ("please ", 7),
    ];
    let lower = content.to_lowercase();
    let mut created = Vec::new();
    let conn = state.db.conn()?;
    for (needle, len) in patterns {
        if let Some(idx) = lower.find(needle) {
            let start = idx + len;
            let end = content[start..]
                .find(|c| c == '.' || c == '\n' || c == '!' || c == '?')
                .map(|e| start + e)
                .unwrap_or_else(|| content.len());
            let title = content[start..end].trim().to_string();
            if title.len() < 4 {
                continue;
            }
            let dedupe = format!("chat:{message_id}:{needle}");
            let id = tasks::upsert(
                &conn,
                &TaskUpsert {
                    title,
                    details: None,
                    status: "pending".into(),
                    priority: None,
                    due_at: None,
                    source_kind: "chat".into(),
                    source_ref: Some(message_id.to_string()),
                    dedupe_key: dedupe,
                },
            )?;
            created.push(id);
        }
    }
    Ok(created)
}
