//! End-to-end conversation loop. Used by web and messaging adapters.
//!
//! Flow:
//!   1. Recall relevant memories.
//!   2. Optionally fetch vault hits.
//!   3. Call the cloud reasoner with cloud-safe context only.
//!   4. Persist the turn via `commit_interaction`.
//!   5. Return the assistant text to the caller for delivery.

use anyhow::Result;

use crate::integrations::cloud::{self, ReasonInput};
use crate::memory::{commit, recall};
use crate::AppState;

pub struct TurnResult {
    pub session_id: String,
    pub assistant_text: String,
    pub memories_used: usize,
    pub search_hits: usize,
}

pub async fn turn(
    state: &AppState,
    channel: &str,
    external_session_id: Option<&str>,
    user_handle: Option<&str>,
    user_message: &str,
    external_msg_id: Option<&str>,
) -> Result<TurnResult> {
    // Memory recall.
    let memories = recall::recall(state, user_message, 4, Some(channel), None, None)
        .await
        .unwrap_or_default();

    // Vault search (cloud-safe excerpts).
    let hits = crate::vault::search::hybrid(state, user_message, 4, None)
        .await
        .unwrap_or_default();

    let mut context = Vec::new();
    for m in &memories {
        context.push(format!("[memory {}] {}", m.created_at, m.excerpt));
    }
    for h in &hits {
        context.push(format!(
            "[vault {}{}] {}",
            h.title.clone().unwrap_or_else(|| h.path.clone()),
            h.section
                .as_ref()
                .map(|s| format!(" • {s}"))
                .unwrap_or_default(),
            h.excerpt
        ));
    }

    let reasoner = cloud::build(
        state.config.cloud_provider,
        state.config.cloud_url.clone(),
        state.config.cloud_api_key.clone(),
        state.config.cloud_model.clone(),
    );
    let input = ReasonInput {
        system: load_system_prompt(state),
        user_message: user_message.to_string(),
        tool_context: context,
    };
    let output = reasoner.reason(&input).await?;

    // Persist the interaction (idempotent on external id).
    let commit_result = commit::commit(
        state,
        commit::CommitInput {
            channel: channel.to_string(),
            external_session_id: external_session_id.map(String::from),
            user_handle: user_handle.map(String::from),
            user: commit::CommitTurn {
                content: user_message.to_string(),
                external_id: external_msg_id.map(String::from),
                tool_metadata: None,
                attachments: None,
            },
            assistant: commit::CommitTurn {
                content: output.text.clone(),
                external_id: external_msg_id.map(|id| format!("{id}:assistant")),
                tool_metadata: Some(serde_json::json!({
                    "provider": output.used_provider,
                    "model": output.used_model,
                    "memories_used": memories.len(),
                    "search_hits": hits.len()
                })),
                attachments: None,
            },
        },
    )
    .await?;

    Ok(TurnResult {
        session_id: commit_result.session_id,
        assistant_text: output.text,
        memories_used: memories.len(),
        search_hits: hits.len(),
    })
}

fn load_system_prompt(state: &AppState) -> String {
    // Prefer a project-local override if present.
    let override_path = state
        .config
        .vault_path
        .join(&state.config.sidecar_dir)
        .join("system_prompt.md");
    std::fs::read_to_string(override_path).unwrap_or_else(|_| cloud::DEFAULT_SYSTEM_PROMPT.to_string())
}
