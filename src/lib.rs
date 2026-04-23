//! Cenote: local-first hybrid life OS daemon.
//!
//! Layout (see prompt-0-implementation-contract.md):
//! - `config`       — environment-driven runtime configuration
//! - `db`           — SQLite/LibSQL-compatible storage + migrations
//! - `mcp`          — MCP server + tool registry
//! - `tools`        — canonical tool implementations
//! - `ingestion`    — privacy-first local file pipeline
//! - `memory`       — conversation persistence + recall
//! - `tasks`        — task mining + persistence
//! - `integrations` — cloud reasoner, messaging, web chat adapters
//! - `vault`        — scanner, watcher, indexing
//! - `vector`       — VectorIndex trait + implementations
//! - `telemetry`    — metrics and structured logging setup

pub mod config;
pub mod db;
pub mod ingestion;
pub mod integrations;
pub mod mcp;
pub mod memory;
pub mod tasks;
pub mod telemetry;
pub mod tools;
pub mod vault;
pub mod vector;

use std::sync::Arc;

use crate::config::Config;
use crate::db::Database;
use crate::integrations::ollama::OllamaClient;
use crate::vector::VectorIndex;

/// Shared runtime state for the daemon. Cloned cheaply via `Arc` internals.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Database,
    pub ollama: Arc<OllamaClient>,
    pub vector: Arc<dyn VectorIndex>,
    pub metrics: Arc<telemetry::Metrics>,
}

impl AppState {
    pub async fn bootstrap(config: Config) -> anyhow::Result<Self> {
        let config = Arc::new(config);
        let db = Database::open(&config.db_path)?;
        let ollama = Arc::new(OllamaClient::new(
            config.ollama_url.clone(),
            config.ollama_extract_model.clone(),
            config.ollama_embed_model.clone(),
            config.ollama_timeout_ms,
        )?);
        let vector: Arc<dyn VectorIndex> =
            Arc::new(vector::FileVectorIndex::open(&config.vector_path)?);
        let metrics = Arc::new(telemetry::Metrics::default());
        Ok(Self {
            config,
            db,
            ollama,
            vector,
            metrics,
        })
    }
}
