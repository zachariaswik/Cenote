//! MCP server surface.
//!
//! We implement a minimal JSON-RPC-ish request/response protocol that matches
//! the Anthropic MCP stdio transport shape: `method` names map to our tool
//! registry entries, and `params` is a JSON object. Full MCP compliance
//! (initialize/shutdown handshakes, etc.) can be added in a later iteration;
//! right now the goal is deliverability, not spec fidelity.

pub mod registry;
pub mod stdio;

use async_trait::async_trait;
use serde_json::Value;

use crate::AppState;

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("tool not found: {0}")]
    NotFound(String),
    #[error("invalid arguments: {0}")]
    InvalidArgs(String),
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[async_trait]
pub trait McpTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn input_schema(&self) -> Value;
    async fn call(&self, state: &AppState, args: Value) -> Result<Value, McpError>;
}

pub use registry::ToolRegistry;
