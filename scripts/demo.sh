#!/usr/bin/env bash
# End-to-end demo for Cenote. Exercises every prompt's deliverables.
set -euo pipefail

cd "$(dirname "$0")/.."

export CENOTE_VAULT_PATH=${CENOTE_VAULT_PATH:-./fixtures/vault}
export CENOTE_DB_PATH=${CENOTE_DB_PATH:-./data/demo.db}
export CENOTE_VECTOR_PATH=${CENOTE_VECTOR_PATH:-./data/demo-vec}
export RUST_LOG=${RUST_LOG:-cenote=warn}

say() { printf "\n\033[1;36m==> %s\033[0m\n" "$*"; }

say "Building"
cargo build --quiet

BIN=./target/debug/cenote

rm -rf "$CENOTE_DB_PATH" "$CENOTE_VECTOR_PATH"

say "migrate"
"$BIN" migrate

say "doctor"
"$BIN" doctor

say "reindex (vault scan + FTS populate)"
"$BIN" reindex

say "search_vault: 'ollama'"
"$BIN" search "ollama" | head -40

say "list_tasks (via MCP registry stdio)"
echo '{"id":1,"method":"tools/call","params":{"name":"list_tasks","arguments":{}}}' \
  | "$BIN" serve 2>/dev/null &
SERVE_PID=$!
sleep 2
kill $SERVE_PID 2>/dev/null || true

say "ingest_new_file (graceful degradation if Ollama is down)"
"$BIN" ingest fixtures/vault/project-notes.md | head -40

say "chat turn via /chat (commit_interaction + recall)"
"$BIN" serve &
SERVE_PID=$!
trap "kill $SERVE_PID 2>/dev/null || true" EXIT
sleep 2
curl -s -X POST http://127.0.0.1:8787/chat \
  -H 'content-type: application/json' \
  -d '{"message":"remind me to pull nomic-embed-text","session_id":"demo"}' | python3 -m json.tool || true
curl -s -X POST http://127.0.0.1:8787/chat \
  -H 'content-type: application/json' \
  -d '{"message":"what was that about nomic?","session_id":"demo"}' | python3 -m json.tool || true

say "metrics"
curl -s http://127.0.0.1:8787/healthz | python3 -m json.tool || true

say "done"
