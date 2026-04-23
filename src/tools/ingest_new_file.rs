//! ingest_new_file: parse a file locally, run the local hive, persist the
//! cloud-safe outputs. The tool **never** returns raw file content.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::ingestion::pipeline::IngestOptions;
use crate::mcp::{McpError, McpTool};
use crate::AppState;

pub struct IngestNewFileTool;

#[derive(Debug, Deserialize)]
struct Args {
    path: String,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    force: bool,
}

#[async_trait]
impl McpTool for IngestNewFileTool {
    fn name(&self) -> &'static str {
        "ingest_new_file"
    }
    fn description(&self) -> &'static str {
        "Ingest a local private file (md/txt/pdf). Processes locally, returns summary/tags/tasks. Never returns raw contents."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "source": {"type": "string"},
                "force": {"type": "boolean", "description": "Re-run even if checksum matches"}
            },
            "required": ["path"],
            "additionalProperties": false
        })
    }

    async fn call(&self, state: &AppState, args: Value) -> Result<Value, McpError> {
        let args: Args = serde_json::from_value(args)
            .map_err(|e| McpError::InvalidArgs(e.to_string()))?;
        let t0 = std::time::Instant::now();
        let result = crate::ingestion::pipeline::ingest(
            state,
            std::path::Path::new(&args.path),
            IngestOptions {
                source: args.source,
                force: args.force,
            },
        )
        .await
        .map_err(McpError::Internal)?;
        state.metrics.incr(&state.metrics.ingestion_jobs);
        if !result.ok {
            state.metrics.incr(&state.metrics.ingestion_failures);
        }
        let ms = t0.elapsed().as_millis() as u64;
        Ok(json!({
            "ok": result.ok,
            "status": result.status,
            "job_id": result.job_id,
            "summary": result.summary,           // cloud-safe
            "tags": result.tags,                  // cloud-safe
            "entities": result.entities,          // cloud-safe
            "task_candidates": result.task_candidates,
            "related_notes": result.related_notes,
            "sidecar_path": result.sidecar_path,
            "latency_ms": ms,
            // Intentionally NOT returned: raw_text, raw_bytes
        }))
    }
}
