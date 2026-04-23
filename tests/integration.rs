//! Integration tests covering the non-Ollama-dependent flows:
//! migrations, vault scan, search, list_tasks, and commit_interaction
//! idempotency. Ollama-dependent flows (ingestion, semantic search, embeddings)
//! gracefully degrade and are exercised where possible.

use std::path::PathBuf;
use std::sync::Arc;

use cenote::config::{CloudProvider, Config, McpTransport};
use cenote::db::Database;
use cenote::integrations::ollama::OllamaClient;
use cenote::memory::commit::{commit, CommitInput, CommitTurn};
use cenote::tasks;
use cenote::telemetry;
use cenote::vault;
use cenote::vector::FileVectorIndex;
use cenote::AppState;

fn fixture_vault() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join("vault")
}

async fn setup() -> AppState {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("cenote-test.db");
    let vec_path = tmp.path().join("vec");
    std::fs::create_dir_all(&vec_path).unwrap();
    let cfg = Config {
        vault_path: fixture_vault(),
        db_path: db_path.clone(),
        vector_path: vec_path.clone(),
        sidecar_dir: ".cenote".into(),
        ollama_url: "http://127.0.0.1:1".into(), // unreachable on purpose
        ollama_extract_model: "llama3.2:3b".into(),
        ollama_embed_model: "nomic-embed-text".into(),
        ollama_timeout_ms: 100,
        http_bind: "127.0.0.1:0".into(),
        mcp_transport: McpTransport::Stdio,
        cloud_provider: CloudProvider::Simulator,
        cloud_url: None,
        cloud_api_key: None,
        cloud_model: None,
        openclaw_url: None,
        openclaw_shared_secret: None,
        turso_url: None,
        turso_auth_token: None,
        log_level: "warn".into(),
        debug_raw_logs: false,
    };
    // Leak the tempdir for the duration of the test.
    std::mem::forget(tmp);
    let _ = telemetry::init("warn");

    let db = Database::open(&db_path).unwrap();
    db.run_migrations().unwrap();
    let ollama = Arc::new(
        OllamaClient::new(
            cfg.ollama_url.clone(),
            cfg.ollama_extract_model.clone(),
            cfg.ollama_embed_model.clone(),
            cfg.ollama_timeout_ms,
        )
        .unwrap(),
    );
    let vector = Arc::new(FileVectorIndex::open(&vec_path).unwrap());
    AppState {
        config: Arc::new(cfg),
        db,
        ollama,
        vector,
        metrics: Arc::new(telemetry::Metrics::default()),
    }
}

#[tokio::test]
async fn migrations_are_idempotent() {
    let state = setup().await;
    state.db.run_migrations().unwrap();
    state.db.run_migrations().unwrap();
}

#[tokio::test]
async fn vault_scan_indexes_notes_and_tasks() {
    let state = setup().await;
    let report = vault::scanner::scan(
        &state.db,
        &state.config.vault_path,
        &state.config.sidecar_dir,
    )
    .unwrap();
    assert!(report.indexed >= 3, "indexed {report:?}");
    let hits = vault::search::keyword(&state.db, "cenote", 10, None).unwrap();
    assert!(!hits.is_empty(), "expected keyword hits for 'cenote'");
    let task_list = tasks::list(&state.db, None, None, None, 50).unwrap();
    assert!(task_list.iter().any(|t| t.title.to_lowercase().contains("ollama")));
}

#[tokio::test]
async fn reindex_is_idempotent_on_unchanged_files() {
    let state = setup().await;
    let r1 = vault::scanner::scan(
        &state.db,
        &state.config.vault_path,
        &state.config.sidecar_dir,
    )
    .unwrap();
    let r2 = vault::scanner::scan(
        &state.db,
        &state.config.vault_path,
        &state.config.sidecar_dir,
    )
    .unwrap();
    assert!(r1.indexed >= 3);
    assert_eq!(r2.indexed, 0, "second pass should skip everything");
    assert!(r2.skipped >= 3);
}

#[tokio::test]
async fn commit_interaction_is_idempotent_on_external_id() {
    let state = setup().await;
    let input = || CommitInput {
        channel: "simulator".into(),
        external_session_id: Some("sess-1".into()),
        user_handle: Some("erik".into()),
        user: CommitTurn {
            content: "remind me to water the plants".into(),
            external_id: Some("ext-user-1".into()),
            tool_metadata: None,
            attachments: None,
        },
        assistant: CommitTurn {
            content: "Got it. Added a reminder.".into(),
            external_id: Some("ext-assistant-1".into()),
            tool_metadata: None,
            attachments: None,
        },
    };
    let first = commit(&state, input()).await.unwrap();
    let second = commit(&state, input()).await.unwrap();
    assert_eq!(first.user_message_id, second.user_message_id);
    assert_eq!(first.assistant_message_id, second.assistant_message_id);

    // Task mining: "remind me to water the plants" should produce one task.
    let task_list = tasks::list(&state.db, None, None, Some("chat"), 50).unwrap();
    assert!(task_list.iter().any(|t| t.title.to_lowercase().contains("water")));
}

#[tokio::test]
async fn ingest_tool_never_returns_raw_bytes() {
    // Even with Ollama unreachable, ingest returns a structured result using
    // the fallback path; it must NEVER include raw file content.
    let state = setup().await;
    let res = cenote::ingestion::pipeline::ingest(
        &state,
        &state.config.vault_path.join("project-notes.md"),
        cenote::ingestion::pipeline::IngestOptions::default(),
    )
    .await
    .unwrap();
    assert!(res.status == "succeeded" || res.status == "skipped");
    let json = serde_json::to_string(&res).unwrap();
    // The raw fixture contains this exact line; the cloud-safe result must not.
    assert!(!json.contains("Raw vault content never leaves this machine"));
}
