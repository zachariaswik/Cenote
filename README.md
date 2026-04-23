# Cenote

A local-first hybrid life OS: a Rust daemon that owns your vault, runs private
files through a local model hive, and exposes a small, well-defined MCP tool
surface to cloud reasoning and messaging/web frontends.

## Architecture

- **Cloud Cerebrum** — a high-reasoning cloud model (Claude / Gemini / GPT).
  Reasons over tool outputs only; never reads raw files.
- **Local Nervous System** — this Rust daemon. Owns storage, search,
  ingestion, memory, and policy.
- **Local Hive** — small local models served through Ollama. Parse private
  content before anything leaves the machine.
- **Primary UI** — WhatsApp & Telegram via OpenClaw (webhook shape included).
- **Secondary UI** — browser chat via the `/chat` HTTP endpoint (LobeChat /
  Open WebUI compatible).

### Canonical MCP tools

| tool | purpose |
|---|---|
| `search_vault`     | hybrid (FTS + semantic) retrieval over local notes |
| `list_tasks`       | structured task records mined from notes and chat |
| `ingest_new_file`  | parse + local-hive extract + sidecar; returns cloud-safe outputs |
| `commit_interaction` | persist every user/assistant turn; idempotent on external msg id |
| `recall_memories`  | ranked episodic recall (semantic + keyword) |

Helper tools: `health_check`.

### Storage

- **Structured**: SQLite (LibSQL-compatible) — schema in `migrations/`.
- **Semantic**: `VectorIndex` trait with a JSON-file fallback; swap in LanceDB
  via a Cargo feature later without touching callers.
- **Human-readable**: Markdown sidecars under `<vault>/.cenote/sidecars/`.

## Quick start

```sh
# 1. Dependencies
rustc --version     # 1.70+
ollama serve &      # keep the local hive running

# 2. Configure
cp .env.example .env

# 3. Initialize
cargo run -- migrate
cargo run -- doctor

# 4. Scan the vault & search
cargo run -- reindex
cargo run -- search "ollama"

# 5. Ingest a private file
cargo run -- ingest fixtures/vault/project-notes.md

# 6. Serve (HTTP chat + webhook + MCP stdio)
cargo run -- serve
```

## Configuration

All via environment (see `.env.example`). Required paths are validated on
boot and parents are created automatically.

## HTTP surface

`cargo run -- serve` binds `CENOTE_HTTP_BIND` (default `127.0.0.1:8787`):

- `GET /healthz` — readiness + metrics snapshot
- `POST /chat` — `{ "message": "...", "session_id": "optional" }` → reply
- `POST /webhooks/openclaw` — OpenClaw-shaped inbound message

The MCP stdio transport runs on the same process. Point an MCP client at
`cenote serve`.

## Privacy boundary

- `ingest_new_file` parses and extracts **locally**. Its tool response returns
  only `{ summary, tags, entities, task_candidates, related_notes }` — never
  raw bytes. Enforced by an integration test
  (`ingest_tool_never_returns_raw_bytes`).
- Sidecar files live under the vault itself so they are portable with the
  user's data.
- The cloud reasoner (`src/integrations/cloud.rs`) is handed only the system
  instruction and cloud-safe tool outputs. It does not have a database or
  filesystem handle.

## Testing

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Integration tests run with Ollama unreachable on purpose; all ingestion and
memory flows degrade gracefully without it.

## Demo

```sh
./scripts/demo.sh
```

Proves: migrations → vault index → search → task list → ingest → chat turn
with `commit_interaction` → recall.

## Troubleshooting

- **"ollama embed status 404"** — install the embedding model:
  `ollama pull nomic-embed-text`.
- **"unknown model llama3.2:3b"** — `ollama pull llama3.2:3b` or point
  `CENOTE_OLLAMA_EXTRACT_MODEL` at a model you already have.
- **DB locked** — another process has the file open; check `ps | grep
  cenote`.
- **Webhook 500s** — verify `CENOTE_OPENCLAW_SHARED_SECRET` matches the
  provider config.

## Repo layout

```
src/
  config.rs           env-driven runtime config
  db/                 SQLite pool + embedded migrations
  mcp/                MCP stdio transport + tool registry
  tools/              canonical tools (search_vault, list_tasks, ingest_new_file,
                      commit_interaction, recall_memories, health_check)
  vault/              scanner, watcher, chunker, search
  vector.rs           VectorIndex trait + JSON-file fallback
  ingestion/          parsers, pipeline, sidecars
  memory/             commit, recall, rolling summaries
  tasks.rs            task upsert/list
  integrations/       ollama client, cloud reasoner, messaging, web chat,
                      orchestrator
  telemetry.rs        tracing + metrics
migrations/           numbered SQL files, idempotent
fixtures/vault/       sample markdown for tests & demo
assets/               editable system prompt
scripts/demo.sh       end-to-end walkthrough
tests/integration.rs  tests for migrations, scan, commit, ingest
```

## Roadmap gaps

- LanceDB backend behind the `VectorIndex` trait (currently JSON-file).
- Real OpenClaw/Anthropic/OpenAI cloud adapters (simulator ships by default).
- Image OCR ingestion (returns an explicit unsupported error today).
