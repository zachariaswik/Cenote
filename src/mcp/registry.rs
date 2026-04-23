//! Central tool registry. The daemon constructs one of these at startup and
//! hands it to each transport (stdio, HTTP). The canonical tool set is wired
//! here — helper/utility tools may be registered alongside.

use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::tools::{
    commit_interaction::CommitInteractionTool, health_check::HealthCheckTool,
    ingest_new_file::IngestNewFileTool, list_tasks::ListTasksTool,
    recall_memories::RecallMemoriesTool, search_vault::SearchVaultTool,
};

use super::{McpError, McpTool};

pub struct ToolRegistry {
    tools: BTreeMap<&'static str, Arc<dyn McpTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            tools: BTreeMap::new(),
        };
        reg.register(Arc::new(SearchVaultTool));
        reg.register(Arc::new(ListTasksTool));
        reg.register(Arc::new(IngestNewFileTool));
        reg.register(Arc::new(CommitInteractionTool));
        reg.register(Arc::new(RecallMemoriesTool));
        reg.register(Arc::new(HealthCheckTool));
        reg
    }

    pub fn register(&mut self, tool: Arc<dyn McpTool>) {
        self.tools.insert(tool.name(), tool);
    }

    pub fn list(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|t| {
                json!({
                    "name": t.name(),
                    "description": t.description(),
                    "inputSchema": t.input_schema(),
                })
            })
            .collect()
    }

    pub fn get(&self, name: &str) -> Result<Arc<dyn McpTool>, McpError> {
        self.tools
            .get(name)
            .cloned()
            .ok_or_else(|| McpError::NotFound(name.to_string()))
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
