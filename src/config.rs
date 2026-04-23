//! Environment-driven configuration.
//!
//! Values are read from process env (loading `.env` if present). Validation
//! rejects missing vault/db/vector paths early so `serve`/`migrate`/`doctor`
//! fail with actionable errors instead of blowing up deep inside a tool call.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum McpTransport {
    Stdio,
    Http,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    Simulator,
    Anthropic,
    Openai,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vault_path: PathBuf,
    pub db_path: PathBuf,
    pub vector_path: PathBuf,
    pub sidecar_dir: String,

    pub ollama_url: String,
    pub ollama_extract_model: String,
    pub ollama_embed_model: String,
    pub ollama_timeout_ms: u64,

    pub http_bind: String,
    pub mcp_transport: McpTransport,

    pub cloud_provider: CloudProvider,
    pub cloud_url: Option<String>,
    pub cloud_api_key: Option<String>,
    pub cloud_model: Option<String>,

    pub openclaw_url: Option<String>,
    pub openclaw_shared_secret: Option<String>,

    pub turso_url: Option<String>,
    pub turso_auth_token: Option<String>,

    pub log_level: String,
    pub debug_raw_logs: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Best-effort: .env is optional.
        let _ = dotenvy::dotenv();

        let vault_path = path_env("CENOTE_VAULT_PATH", "./fixtures/vault");
        let db_path = path_env("CENOTE_DB_PATH", "./data/cenote.db");
        let vector_path = path_env("CENOTE_VECTOR_PATH", "./data/vector");
        let sidecar_dir = str_env("CENOTE_SIDECAR_DIR", ".cenote");

        let ollama_url = str_env("CENOTE_OLLAMA_URL", "http://127.0.0.1:11434");
        let ollama_extract_model = str_env("CENOTE_OLLAMA_EXTRACT_MODEL", "llama3.2:3b");
        let ollama_embed_model = str_env("CENOTE_OLLAMA_EMBED_MODEL", "nomic-embed-text");
        let ollama_timeout_ms = u64_env("CENOTE_OLLAMA_TIMEOUT_MS", 60_000)?;

        let http_bind = str_env("CENOTE_HTTP_BIND", "127.0.0.1:8787");
        let mcp_transport = match str_env("CENOTE_MCP_TRANSPORT", "stdio").as_str() {
            "stdio" => McpTransport::Stdio,
            "http" => McpTransport::Http,
            other => bail!("CENOTE_MCP_TRANSPORT must be stdio|http, got {other}"),
        };

        let cloud_provider = match str_env("CENOTE_CLOUD_PROVIDER", "simulator").as_str() {
            "simulator" => CloudProvider::Simulator,
            "anthropic" => CloudProvider::Anthropic,
            "openai" => CloudProvider::Openai,
            "custom" => CloudProvider::Custom,
            other => bail!("CENOTE_CLOUD_PROVIDER invalid: {other}"),
        };

        let cfg = Config {
            vault_path,
            db_path,
            vector_path,
            sidecar_dir,
            ollama_url,
            ollama_extract_model,
            ollama_embed_model,
            ollama_timeout_ms,
            http_bind,
            mcp_transport,
            cloud_provider,
            cloud_url: opt_env("CENOTE_CLOUD_URL"),
            cloud_api_key: opt_env("CENOTE_CLOUD_API_KEY"),
            cloud_model: opt_env("CENOTE_CLOUD_MODEL"),
            openclaw_url: opt_env("CENOTE_OPENCLAW_URL"),
            openclaw_shared_secret: opt_env("CENOTE_OPENCLAW_SHARED_SECRET"),
            turso_url: opt_env("CENOTE_TURSO_URL"),
            turso_auth_token: opt_env("CENOTE_TURSO_AUTH_TOKEN"),
            log_level: str_env("RUST_LOG", "cenote=info"),
            debug_raw_logs: bool_env("CENOTE_DEBUG_RAW_LOGS", false)?,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        // Parents of db / vector paths must be creatable.
        if let Some(parent) = self.db_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating db parent dir {parent:?}"))?;
            }
        }
        std::fs::create_dir_all(&self.vector_path)
            .with_context(|| format!("creating vector dir {:?}", self.vector_path))?;
        // Vault path existence is advisory — a fresh user may not have one.
        Ok(())
    }
}

fn str_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
fn opt_env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|s| !s.is_empty())
}
fn path_env(key: &str, default: &str) -> PathBuf {
    PathBuf::from(str_env(key, default))
}
fn u64_env(key: &str, default: u64) -> Result<u64> {
    match std::env::var(key) {
        Ok(v) => v.parse::<u64>().with_context(|| format!("parsing {key}")),
        Err(_) => Ok(default),
    }
}
fn bool_env(key: &str, default: bool) -> Result<bool> {
    match std::env::var(key) {
        Ok(v) => match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" | "" => Ok(false),
            other => bail!("{key} must be bool, got {other}"),
        },
        Err(_) => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_validate() {
        // With HOME-relative paths unset, config should still populate defaults.
        let cfg = Config {
            vault_path: PathBuf::from("./fixtures/vault"),
            db_path: PathBuf::from(std::env::temp_dir().join("cenote-test.db")),
            vector_path: std::env::temp_dir().join("cenote-vec-test"),
            sidecar_dir: ".cenote".into(),
            ollama_url: "http://127.0.0.1:11434".into(),
            ollama_extract_model: "llama3.2:3b".into(),
            ollama_embed_model: "nomic-embed-text".into(),
            ollama_timeout_ms: 1000,
            http_bind: "127.0.0.1:8787".into(),
            mcp_transport: McpTransport::Stdio,
            cloud_provider: CloudProvider::Simulator,
            cloud_url: None,
            cloud_api_key: None,
            cloud_model: None,
            openclaw_url: None,
            openclaw_shared_secret: None,
            turso_url: None,
            turso_auth_token: None,
            log_level: "info".into(),
            debug_raw_logs: false,
        };
        cfg.validate().unwrap();
    }
}
