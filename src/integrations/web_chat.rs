//! HTTP chat surface. Provides a small OpenAI-compatible-ish `/v1/chat`
//! endpoint so LobeChat / Open WebUI can point at the daemon and the user
//! gets the full end-to-end loop, including `commit_interaction`.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::memory::commit::{commit, CommitInput, CommitTurn};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub session_id: Option<String>,
    pub user_handle: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub session_id: String,
    pub reply: String,
    pub memories_used: usize,
    pub search_hits: usize,
}

pub async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    let reply = crate::integrations::orchestrator::turn(
        &state,
        "web",
        req.session_id.as_deref(),
        req.user_handle.as_deref(),
        &req.message,
        None,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ChatResponse {
        session_id: reply.session_id,
        reply: reply.assistant_text,
        memories_used: reply.memories_used,
        search_hits: reply.search_hits,
    }))
}

#[derive(Debug, Deserialize)]
pub struct WebhookRequest {
    pub provider: String,
    pub external_session_id: String,
    pub external_msg_id: String,
    pub user_handle: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub session_id: String,
    pub reply: String,
}

/// OpenClaw-shaped webhook entrypoint. In production, pair with a shared
/// secret header and signature verification (see `config::openclaw_shared_secret`).
pub async fn webhook(
    State(state): State<AppState>,
    Json(req): Json<WebhookRequest>,
) -> Result<Json<WebhookResponse>, (StatusCode, String)> {
    let result = crate::integrations::orchestrator::turn(
        &state,
        &req.provider,
        Some(&req.external_session_id),
        req.user_handle.as_deref(),
        &req.text,
        Some(&req.external_msg_id),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(WebhookResponse {
        session_id: result.session_id,
        reply: result.assistant_text,
    }))
}

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let vault_exists = state.config.vault_path.exists();
    let ollama_ok = state.ollama.health().await.is_ok();
    let vectors = state.vector.len().await.unwrap_or(0);
    Json(serde_json::json!({
        "ok": true,
        "vault_exists": vault_exists,
        "ollama_ok": ollama_ok,
        "vectors": vectors,
        "metrics": state.metrics.snapshot(),
    }))
}

/// Stub to keep imports happy; the commit path lives in `orchestrator::turn`.
#[allow(dead_code)]
async fn _unused_commit_bridge(state: &AppState, input: CommitInput) -> anyhow::Result<()> {
    let _ = commit(state, input).await?;
    Ok(())
}

#[allow(dead_code)]
fn _touch() -> CommitTurn {
    CommitTurn {
        content: String::new(),
        external_id: None,
        tool_metadata: None,
        attachments: None,
    }
}
