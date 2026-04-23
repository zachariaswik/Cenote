//! Provider-neutral cloud reasoner abstraction.
//!
//! The cloud layer has ONLY two allowed inputs:
//! 1. The system instruction (editable asset on disk or env)
//! 2. Tool outputs and user message text passed into `reason()`
//!
//! It MUST NOT read files, the vector index, or the sqlite db directly.
//! Enforced by keeping this module ignorant of `AppState`'s raw accessors.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::config::CloudProvider;

pub const DEFAULT_SYSTEM_PROMPT: &str = include_str!("../../assets/system_prompt.md");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonInput {
    pub system: String,
    pub user_message: String,
    /// Summaries from `recall_memories`, `search_vault`, `list_tasks` — all cloud-safe.
    pub tool_context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonOutput {
    pub text: String,
    pub used_provider: String,
    pub used_model: Option<String>,
}

#[async_trait]
pub trait CloudReasoner: Send + Sync {
    async fn reason(&self, input: &ReasonInput) -> Result<ReasonOutput>;
}

/// Deterministic simulator. Stands in for Claude / Gemini / GPT during
/// local development and tests. Returns a structured echo that threads the
/// context forward without actually calling a remote service.
pub struct SimulatedReasoner;

#[async_trait]
impl CloudReasoner for SimulatedReasoner {
    async fn reason(&self, input: &ReasonInput) -> Result<ReasonOutput> {
        let mut out = String::new();
        out.push_str("[Cenote • simulated reasoner]\n");
        if !input.tool_context.is_empty() {
            out.push_str("Context used:\n");
            for c in &input.tool_context {
                out.push_str(&format!("- {}\n", c.chars().take(180).collect::<String>()));
            }
        }
        out.push_str("\nYou said: ");
        out.push_str(&input.user_message);
        out.push_str("\n\nNext steps I can take: search_vault, ingest_new_file, list_tasks.");
        Ok(ReasonOutput {
            text: out,
            used_provider: "simulator".into(),
            used_model: None,
        })
    }
}

pub fn build(provider: CloudProvider, _url: Option<String>, _key: Option<String>, _model: Option<String>) -> Box<dyn CloudReasoner> {
    match provider {
        // Real providers can be added here — the simulator is the default so
        // the daemon boots and runs end-to-end with zero external secrets.
        _ => Box::new(SimulatedReasoner),
    }
}
