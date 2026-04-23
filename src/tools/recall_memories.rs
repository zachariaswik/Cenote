use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::mcp::{McpError, McpTool};
use crate::AppState;

pub struct RecallMemoriesTool;

#[derive(Debug, Deserialize)]
struct Args {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    channel: Option<String>,
    #[serde(default)]
    since: Option<String>,
    #[serde(default)]
    until: Option<String>,
}
fn default_limit() -> usize {
    6
}

#[async_trait]
impl McpTool for RecallMemoriesTool {
    fn name(&self) -> &'static str {
        "recall_memories"
    }
    fn description(&self) -> &'static str {
        "Recall ranked conversation memories (semantic + keyword). Returns concise excerpts, not full transcripts."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string"},
                "limit": {"type": "integer", "minimum": 1, "maximum": 50},
                "channel": {"type": "string"},
                "since": {"type": "string"},
                "until": {"type": "string"}
            },
            "required": ["query"],
            "additionalProperties": false
        })
    }

    async fn call(&self, state: &AppState, args: Value) -> Result<Value, McpError> {
        let args: Args = serde_json::from_value(args)
            .map_err(|e| McpError::InvalidArgs(e.to_string()))?;
        let t0 = std::time::Instant::now();
        let memories = crate::memory::recall::recall(
            state,
            &args.query,
            args.limit,
            args.channel.as_deref(),
            args.since.as_deref(),
            args.until.as_deref(),
        )
        .await
        .map_err(McpError::Internal)?;
        let ms = t0.elapsed().as_millis() as u64;
        state.metrics.incr(&state.metrics.recall_calls);
        state
            .metrics
            .add(&state.metrics.recall_latency_ms_total, ms);
        Ok(json!({ "memories": memories, "latency_ms": ms }))
    }
}
