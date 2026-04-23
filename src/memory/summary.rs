//! Rolling conversation summaries. Additive — never overwrites messages.

use anyhow::Result;
use rusqlite::params;

use crate::AppState;

/// Produce a summary covering the last `window` messages in a session. Uses
/// Ollama if available; otherwise collapses content locally as a fallback.
pub async fn summarize_session(
    state: &AppState,
    session_id: &str,
    window: usize,
) -> Result<Option<String>> {
    let conn = state.db.conn()?;
    let mut stmt = conn.prepare(
        "SELECT role, content, created_at FROM conversation_messages
         WHERE session_id = ?1 ORDER BY created_at DESC LIMIT ?2",
    )?;
    let rows = stmt
        .query_map(params![session_id, window as i64], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Ok(None);
    }
    let covers_from = rows.last().map(|r| r.2.clone()).unwrap_or_default();
    let covers_to = rows.first().map(|r| r.2.clone()).unwrap_or_default();
    let transcript = rows
        .iter()
        .rev()
        .map(|(role, content, _)| format!("{role}: {content}"))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "Summarize the following conversation in 3-5 bullet points. Keep names, decisions, and open tasks. Do not include raw PII.\n\n{transcript}"
    );
    let summary = match state.ollama.generate(&prompt).await {
        Ok(s) => s.trim().to_string(),
        Err(_) => fallback_summary(&rows),
    };

    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO conversation_summaries(id, session_id, summary, covers_from, covers_to) VALUES(?1,?2,?3,?4,?5)",
        params![id, session_id, summary, covers_from, covers_to],
    )?;
    Ok(Some(summary))
}

fn fallback_summary(rows: &[(String, String, String)]) -> String {
    let mut out = String::new();
    for (role, content, _) in rows.iter().take(5) {
        let snip: String = content.chars().take(120).collect();
        out.push_str(&format!("- [{role}] {snip}\n"));
    }
    out
}
