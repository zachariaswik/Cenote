//! Cenote CLI entrypoint.

use std::sync::Arc;

use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use clap::{Parser, Subcommand};
use tower_http::trace::TraceLayer;

use cenote::config::Config;
use cenote::mcp::ToolRegistry;
use cenote::vault;
use cenote::AppState;

#[derive(Parser, Debug)]
#[command(name = "cenote", version, about = "Local-first hybrid life OS daemon")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Apply migrations (idempotent).
    Migrate,
    /// Rescan the vault and rebuild indexes.
    Reindex {
        #[arg(long, default_value_t = false)]
        with_embeddings: bool,
    },
    /// Diagnostics: config, db, ollama, vault, vector index.
    Doctor,
    /// Run the daemon: MCP server + HTTP chat + webhook.
    Serve,
    /// One-off: ingest a file via the local hive and print results.
    Ingest {
        path: String,
        #[arg(long)]
        force: bool,
    },
    /// One-off: search the vault and print results.
    Search {
        query: String,
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;
    cenote::telemetry::init(&config.log_level)?;
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Migrate => {
            let state = AppState::bootstrap(config).await?;
            state.db.run_migrations()?;
            println!("migrations applied");
        }
        Cmd::Reindex { with_embeddings } => {
            let state = AppState::bootstrap(config).await?;
            state.db.run_migrations()?;
            let report = vault::scanner::scan(&state.db, &state.config.vault_path, &state.config.sidecar_dir)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            if with_embeddings {
                reindex_embeddings(&state).await?;
            }
        }
        Cmd::Doctor => {
            let state = AppState::bootstrap(config).await?;
            state.db.run_migrations()?;
            let ollama_ok = state.ollama.health().await.is_ok();
            let vault_exists = state.config.vault_path.exists();
            let vectors = state.vector.len().await.unwrap_or(0);
            let tools = ToolRegistry::new()
                .list()
                .into_iter()
                .map(|v| v["name"].as_str().unwrap_or("").to_string())
                .collect::<Vec<_>>();
            let report = serde_json::json!({
                "db_path": state.config.db_path.display().to_string(),
                "vault_path": state.config.vault_path.display().to_string(),
                "vault_exists": vault_exists,
                "ollama_url": state.ollama.base_url(),
                "ollama_ok": ollama_ok,
                "vectors": vectors,
                "tools": tools,
                "cloud_provider": format!("{:?}", state.config.cloud_provider),
                "mcp_transport": format!("{:?}", state.config.mcp_transport),
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Cmd::Serve => {
            let state = AppState::bootstrap(config).await?;
            state.db.run_migrations()?;
            // Prime the vault.
            let _ = vault::scanner::scan(&state.db, &state.config.vault_path, &state.config.sidecar_dir);

            let registry = Arc::new(ToolRegistry::new());

            // MCP stdio transport on a separate task. We keep stdin/stdout for MCP
            // and bind HTTP for chat and webhooks in parallel.
            let mcp_state = state.clone();
            let mcp_registry = registry.clone();
            tokio::spawn(async move {
                if let Err(e) = cenote::mcp::stdio::run(mcp_state, mcp_registry).await {
                    tracing::warn!(error=%e, "mcp stdio loop exited");
                }
            });

            // Watcher.
            let _watch_rx = vault::watcher::spawn(state.db.clone(), state.config.vault_path.clone())?;

            // HTTP surfaces.
            let http_bind = state.config.http_bind.clone();
            let app = Router::new()
                .route("/healthz", get(cenote::integrations::web_chat::health))
                .route("/chat", post(cenote::integrations::web_chat::chat))
                .route(
                    "/webhooks/openclaw",
                    post(cenote::integrations::web_chat::webhook),
                )
                .layer(TraceLayer::new_for_http())
                .with_state(state.clone());
            tracing::info!(bind=%http_bind, "serving http");
            let listener = tokio::net::TcpListener::bind(&http_bind).await?;
            axum::serve(listener, app).await?;
        }
        Cmd::Ingest { path, force } => {
            let state = AppState::bootstrap(config).await?;
            state.db.run_migrations()?;
            let opts = cenote::ingestion::pipeline::IngestOptions {
                source: None,
                force,
            };
            let res = cenote::ingestion::pipeline::ingest(&state, std::path::Path::new(&path), opts).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Cmd::Search { query, limit } => {
            let state = AppState::bootstrap(config).await?;
            state.db.run_migrations()?;
            let hits = cenote::vault::search::hybrid(&state, &query, limit, None).await?;
            println!("{}", serde_json::to_string_pretty(&hits)?);
        }
    }
    Ok(())
}

async fn reindex_embeddings(state: &AppState) -> Result<()> {
    let conn = state.db.conn()?;
    let mut stmt = conn.prepare("SELECT id, text FROM note_chunks")?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    drop(stmt);
    for (chunk_id, text) in rows {
        if let Ok(vec) = state.ollama.embed(&text).await {
            let stored = cenote::vector::StoredVector {
                id: format!("chunk:{chunk_id}"),
                kind: "note_chunk".into(),
                ref_id: chunk_id,
                payload: text.chars().take(240).collect(),
                vector: vec,
            };
            let _ = state.vector.upsert(vec![stored]).await;
        }
    }
    Ok(())
}
