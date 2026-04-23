//! Thin HTTP client for a locally-hosted Ollama server.
//!
//! We stay in HTTP land (no SDK) because Ollama's API is simple and we want
//! to avoid pulling a whole LLM-tools dependency. Retries are applied only
//! to transport errors — a 400-class failure from Ollama (unknown model,
//! etc.) is surfaced straight to the caller.

use std::time::Duration;

use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: GenerateOptions,
}
#[derive(Debug, Serialize, Default)]
struct GenerateOptions {
    temperature: f32,
    num_ctx: u32,
}
#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Debug, Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}
#[derive(Debug, Deserialize)]
struct EmbedResponse {
    embedding: Vec<f32>,
}

pub struct OllamaClient {
    http: Client,
    base_url: String,
    extract_model: String,
    embed_model: String,
}

impl OllamaClient {
    pub fn new(
        base_url: String,
        extract_model: String,
        embed_model: String,
        timeout_ms: u64,
    ) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .context("building reqwest client")?;
        Ok(Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
            extract_model,
            embed_model,
        })
    }

    pub fn extract_model(&self) -> &str {
        &self.extract_model
    }
    pub fn embed_model(&self) -> &str {
        &self.embed_model
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Ping `/api/tags` — cheap liveness check used by `doctor`.
    pub async fn health(&self) -> Result<()> {
        let url = format!("{}/api/tags", self.base_url);
        let res = self.http.get(&url).send().await.context("GET /api/tags")?;
        if !res.status().is_success() {
            bail!("ollama /api/tags status {}", res.status());
        }
        Ok(())
    }

    /// Non-streaming generate. Retries twice on transport errors.
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        let body = GenerateRequest {
            model: &self.extract_model,
            prompt,
            stream: false,
            options: GenerateOptions {
                temperature: 0.2,
                num_ctx: 4096,
            },
        };
        let mut last_err = None;
        for attempt in 0..3 {
            let res = self.http.post(&url).json(&body).send().await;
            match res {
                Ok(r) if r.status().is_success() => {
                    let parsed: GenerateResponse = r.json().await.context("parsing generate")?;
                    return Ok(parsed.response);
                }
                Ok(r) => bail!("ollama generate status {}: {}", r.status(), r.text().await.unwrap_or_default()),
                Err(e) => {
                    tracing::warn!(error = %e, attempt, "ollama generate transport error");
                    last_err = Some(e);
                    tokio::time::sleep(Duration::from_millis(250 * (attempt + 1) as u64)).await;
                }
            }
        }
        Err(anyhow::anyhow!(
            "ollama generate failed after retries: {:?}",
            last_err
        ))
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.base_url);
        let body = EmbedRequest {
            model: &self.embed_model,
            prompt: text,
        };
        let res = self.http.post(&url).json(&body).send().await?;
        if !res.status().is_success() {
            bail!("ollama embed status {}", res.status());
        }
        let parsed: EmbedResponse = res.json().await?;
        Ok(parsed.embedding)
    }
}
