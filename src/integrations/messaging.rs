//! OpenClaw-compatible webhook shape + local messaging simulator.
//!
//! Design: we don't hard-couple to OpenClaw's wire format (it's evolving).
//! Instead we define an internal `InboundMessage` / `OutboundReply` pair, and
//! an adapter trait each provider implements. A local simulator produces
//! synthetic inbound messages so the full loop (webhook → orchestrator →
//! `commit_interaction` → reply) can be exercised in tests without real keys.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundMessage {
    pub provider: String,           // "whatsapp" | "telegram" | "simulator"
    pub external_session_id: String, // chat id / conversation id
    pub external_msg_id: String,
    pub user_handle: Option<String>,
    pub text: String,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundReply {
    pub provider: String,
    pub external_session_id: String,
    pub reply_to_msg_id: Option<String>,
    pub text: String,
}

#[async_trait]
pub trait MessagingAdapter: Send + Sync {
    fn provider(&self) -> &str;
    async fn deliver(&self, reply: &OutboundReply) -> anyhow::Result<()>;
}

/// No-op adapter used during local dev; logs the outbound reply.
pub struct LocalSimulatorAdapter;

#[async_trait]
impl MessagingAdapter for LocalSimulatorAdapter {
    fn provider(&self) -> &str {
        "simulator"
    }
    async fn deliver(&self, reply: &OutboundReply) -> anyhow::Result<()> {
        tracing::info!(
            provider = %reply.provider,
            session = %reply.external_session_id,
            "simulator -> delivered: {}",
            reply.text.chars().take(200).collect::<String>()
        );
        Ok(())
    }
}
