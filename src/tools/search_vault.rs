//! search_vault: keyword + semantic + hybrid search over note chunks.
//!
//! Hybrid ranking = normalized FTS score blended with cosine-vector score.
//! We ask for `limit * 4` candidates from each side so reranking has room.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::mcp::{McpError, McpTool};
use crate::AppState;

pub struct SearchVaultTool;

#[derive(Debug, Deserialize)]
struct Args {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    mode: Option<String>, // "keyword" | "semantic" | "hybrid" (default)
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    path_prefix: Option<String>,
}
fn default_limit() -> usize {
    8
}

#[async_trait]
impl McpTool for SearchVaultTool {
    fn name(&self) -> &'static str {
        "search_vault"
    }
    fn description(&self) -> &'static str {
        "Search local vault notes (FTS + semantic). Returns excerpts with path, title, score."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string"},
                "limit": {"type": "integer", "minimum": 1, "maximum": 50},
                "mode": {"type": "string", "enum": ["keyword", "semantic", "hybrid"]},
                "tags": {"type": "array", "items": {"type": "string"}},
                "path_prefix": {"type": "string"}
            },
            "required": ["query"],
            "additionalProperties": false
        })
    }

    async fn call(&self, state: &AppState, args: Value) -> Result<Value, McpError> {
        let args: Args = serde_json::from_value(args)
            .map_err(|e| McpError::InvalidArgs(e.to_string()))?;
        let mode = args.mode.as_deref().unwrap_or("hybrid");
        let t0 = std::time::Instant::now();

        let hits = match mode {
            "keyword" => crate::vault::search::keyword(
                &state.db,
                &args.query,
                args.limit,
                args.path_prefix.as_deref(),
            )?,
            "semantic" => {
                crate::vault::search::semantic(state, &args.query, args.limit, args.path_prefix.as_deref())
                    .await?
            }
            _ => {
                crate::vault::search::hybrid(
                    state,
                    &args.query,
                    args.limit,
                    args.path_prefix.as_deref(),
                )
                .await?
            }
        };

        let ms = t0.elapsed().as_millis() as u64;
        state.metrics.incr(&state.metrics.search_calls);
        state
            .metrics
            .add(&state.metrics.search_latency_ms_total, ms);

        Ok(json!({
            "mode": mode,
            "query": args.query,
            "results": hits,
            "latency_ms": ms
        }))
    }
}
