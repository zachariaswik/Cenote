//! Stdio JSON-RPC transport for the MCP server.
//!
//! Protocol (simplified): each inbound line is a JSON object like
//!   { "id": "...", "method": "tools/list" | "tools/call", "params": { ... } }
//! Replies are `{ "id": "...", "result": ... }` or `{ "id": "...", "error": { "message": ... } }`.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

use super::registry::ToolRegistry;
use super::McpError;
use crate::AppState;

#[derive(Debug, Deserialize)]
struct McpRequest {
    #[serde(default)]
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum McpResponseBody {
    Ok { id: Value, result: Value },
    Err { id: Value, error: Value },
}

pub async fn run(state: AppState, registry: Arc<ToolRegistry>) -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = Arc::new(Mutex::new(tokio::io::stdout()));
    let mut reader = BufReader::new(stdin).lines();
    while let Some(line) = reader.next_line().await? {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        let req: McpRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(error = %e, "invalid MCP json");
                continue;
            }
        };
        let id = req.id.unwrap_or(Value::Null);
        let reg = registry.clone();
        let state = state.clone();
        let stdout = stdout.clone();
        tokio::spawn(async move {
            let body = handle(&state, reg, req.method, req.params, id.clone()).await;
            let s = serde_json::to_string(&body).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
            let mut guard = stdout.lock().await;
            let _ = guard.write_all(s.as_bytes()).await;
            let _ = guard.write_all(b"\n").await;
            let _ = guard.flush().await;
        });
    }
    Ok(())
}

async fn handle(
    state: &AppState,
    registry: Arc<ToolRegistry>,
    method: String,
    params: Value,
    id: Value,
) -> McpResponseBody {
    match method.as_str() {
        "tools/list" => McpResponseBody::Ok {
            id,
            result: json!({ "tools": registry.list() }),
        },
        "tools/call" => {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);
            match registry.get(name) {
                Ok(tool) => match tool.call(state, arguments).await {
                    Ok(result) => McpResponseBody::Ok { id, result },
                    Err(e) => McpResponseBody::Err {
                        id,
                        error: json!({ "message": e.to_string() }),
                    },
                },
                Err(e) => McpResponseBody::Err {
                    id,
                    error: json!({ "message": e.to_string() }),
                },
            }
        }
        _ => McpResponseBody::Err {
            id,
            error: json!({
                "message": format!("unknown method {method}")
            }),
        },
    }
}

#[allow(dead_code)]
pub fn format_tool_error(e: McpError) -> Value {
    json!({ "message": e.to_string() })
}
