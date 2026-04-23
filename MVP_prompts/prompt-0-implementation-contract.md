# Prompt 0: Implementation Contract

Use this prompt first. It defines the architectural and delivery contract for every later step.

## Prompt
You are implementing **Cenote**, a local-first hybrid life OS, from scratch in this repository.

Before writing code:
- Inspect the repository and preserve any existing documentation or useful scaffolding.
- Do not rewrite completed work unless it blocks the new requirement.
- Prefer incremental, runnable changes over broad speculative refactors.

## Product Definition
Build a system with these roles:
- **Cloud Cerebrum:** a high-reasoning cloud model that handles planning and conversation.
- **Local Nervous System:** a Rust daemon that owns tools, storage, indexing, and policy enforcement.
- **Local Hive:** small local models, served through Ollama, that parse private files and extract structured data before anything reaches the cloud.
- **Primary UI:** WhatsApp and Telegram through OpenClaw.
- **Secondary UI:** a browser-based chat surface such as LobeChat or Open WebUI.

## Non-Negotiable Rules
- The Rust daemon is the source of truth for tools, storage, and policy.
- Raw private file contents must be processed locally first.
- The cloud model must never be given raw vault contents unless a later prompt explicitly authorizes a narrow exception.
- Every user/assistant turn must be persisted locally through `commit_interaction`.
- Semantic recall must be local-first.
- The system must run locally in development without requiring a remote Turso account.
- Use a LibSQL-compatible local database by default, with optional Turso sync or replica support behind configuration.

## Core Storage
- Structured data: LibSQL / SQLite-compatible database.
- Semantic index: LanceDB by default, but hide it behind a `VectorIndex` trait so a fallback can be swapped in if the Rust LanceDB integration blocks progress.
- Human-readable artifacts: Markdown sidecar files and notes in the user vault.

## Canonical MCP Tool Surface
Implement these tool contracts over the course of the prompt sequence:
- `search_vault`
- `list_tasks`
- `ingest_new_file`
- `commit_interaction`
- `recall_memories`

You may add helper tools such as `health_check`, `get_note_context`, or `upsert_task`, but do not remove or rename the canonical tools.

## Required Engineering Standards
- Rust stable toolchain.
- Configuration via environment variables plus a checked-in example env file.
- Database migrations checked into the repo.
- Structured logging and tracing.
- Unit tests for core logic and integration tests for major tool flows.
- CLI commands for running the daemon, running migrations, reindexing data, and validating configuration.
- Clear README or runbook updates for anything a developer must do manually.

## Suggested High-Level Repository Shape
You may refine names, but keep responsibilities clear:
- `src/config`
- `src/db`
- `src/mcp`
- `src/tools`
- `src/ingestion`
- `src/memory`
- `src/integrations`
- `src/tasks`
- `migrations/`
- `tests/`
- `fixtures/`

## Output Contract For Every Implementation Step
At the end of each prompt execution:
- Summarize what was implemented.
- List the main files changed.
- List the commands run.
- Report test status honestly.
- Call out any unresolved risks or follow-up items.

## Definition Of Done For The Full Project
The finished system must be able to:
1. Watch and index a local vault.
2. Ingest new notes and documents locally through Ollama-backed workers.
3. Persist tasks, note metadata, relationships, and chat history.
4. Retrieve note context and episodic memories through MCP tools.
5. Route chat through messaging and web frontends.
6. Enforce privacy boundaries so the cloud layer only sees summaries, search results, and tool outputs.
7. Run locally with documented setup and verification steps.
