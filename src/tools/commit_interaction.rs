use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::mcp::{McpError, McpTool};
use crate::memory::commit::{CommitInput, CommitTurn};
use crate::AppState;

pub struct CommitInteractionTool;

#[derive(Debug, Deserialize)]
struct Args {
    channel: String,
    external_session_id: Option<String>,
    user_handle: Option<String>,

    user_message: String,
    user_external_id: Option<String>,

    assistant_message: String,
    assistant_external_id: Option<String>,

    #[serde(default)]
    tool_invocations: Option<Value>,
    #[serde(default)]
    attachments: Option<Value>,
}

#[async_trait]
impl McpTool for CommitInteractionTool {
    fn name(&self) -> &'static str {
        "commit_interaction"
    }
    fn description(&self) -> &'static str {
        "Persist a user+assistant exchange locally. Idempotent by (session_id, external_msg_id, role). Mines implicit tasks."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "channel": {"type": "string"},
                "external_session_id": {"type": "string"},
                "user_handle": {"type": "string"},
                "user_message": {"type": "string"},
                "user_external_id": {"type": "string"},
                "assistant_message": {"type": "string"},
                "assistant_external_id": {"type": "string"},
                "tool_invocations": {"type": "array"},
                "attachments": {"type": "array"}
            },
            "required": ["channel", "user_message", "assistant_message"],
            "additionalProperties": false
        })
    }

    async fn call(&self, state: &AppState, args: Value) -> Result<Value, McpError> {
        let args: Args = serde_json::from_value(args)
            .map_err(|e| McpError::InvalidArgs(e.to_string()))?;
        let result = crate::memory::commit::commit(
            state,
            CommitInput {
                channel: args.channel,
                external_session_id: args.external_session_id,
                user_handle: args.user_handle,
                user: CommitTurn {
                    content: args.user_message,
                    external_id: args.user_external_id,
                    tool_metadata: None,
                    attachments: args.attachments.clone(),
                },
                assistant: CommitTurn {
                    content: args.assistant_message,
                    external_id: args.assistant_external_id,
                    tool_metadata: args.tool_invocations,
                    attachments: None,
                },
            },
        )
        .await
        .map_err(McpError::Internal)?;
        Ok(json!({
            "session_id": result.session_id,
            "user_message_id": result.user_message_id,
            "assistant_message_id": result.assistant_message_id,
            "task_candidates": result.mined_tasks,
        }))
    }
}
