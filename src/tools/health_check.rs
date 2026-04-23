use async_trait::async_trait;
use serde_json::{json, Value};

use crate::mcp::{McpError, McpTool};
use crate::AppState;

pub struct HealthCheckTool;

#[async_trait]
impl McpTool for HealthCheckTool {
    fn name(&self) -> &'static str {
        "health_check"
    }
    fn description(&self) -> &'static str {
        "Report daemon health: db connectivity, vault path, ollama reachability, vector index size."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        })
    }
    async fn call(&self, state: &AppState, _args: Value) -> Result<Value, McpError> {
        let db_ok = state.db.conn().and_then(|c| {
            c.query_row("SELECT 1", [], |_| Ok(()))?;
            Ok(())
        }).is_ok();
        let vault_exists = state.config.vault_path.exists();
        let ollama_ok = state.ollama.health().await.is_ok();
        let vectors = state.vector.len().await.unwrap_or(0);
        Ok(json!({
            "ok": db_ok,
            "db": db_ok,
            "vault_path": state.config.vault_path.display().to_string(),
            "vault_exists": vault_exists,
            "ollama_ok": ollama_ok,
            "ollama_url": state.ollama.base_url(),
            "vectors": vectors,
            "metrics": state.metrics.snapshot(),
        }))
    }
}
