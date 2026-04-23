use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::mcp::{McpError, McpTool};
use crate::AppState;

pub struct ListTasksTool;

#[derive(Debug, Deserialize, Default)]
struct Args {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    due_before: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default = "default_limit")]
    limit: usize,
}
fn default_limit() -> usize {
    50
}

#[async_trait]
impl McpTool for ListTasksTool {
    fn name(&self) -> &'static str {
        "list_tasks"
    }
    fn description(&self) -> &'static str {
        "List tasks with optional filters: status, due_before, source. Default returns pending+in_progress."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "status": {"type": "string", "enum": ["pending", "in_progress", "done", "cancelled", "any"]},
                "due_before": {"type": "string", "description": "ISO 8601 timestamp"},
                "source": {"type": "string"},
                "limit": {"type": "integer", "minimum": 1, "maximum": 500}
            },
            "additionalProperties": false
        })
    }

    async fn call(&self, state: &AppState, args: Value) -> Result<Value, McpError> {
        let args: Args = serde_json::from_value(args).unwrap_or_default();
        let tasks = crate::tasks::list(&state.db, args.status.as_deref(), args.due_before.as_deref(), args.source.as_deref(), args.limit)?;
        Ok(json!({ "tasks": tasks, "count": tasks.len() }))
    }
}
