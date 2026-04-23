# Prompt 1: Core Daemon, Config, And Schema

Run this after `prompt-0-implementation-contract.md`.

## Prompt
Continue implementing Cenote in this repository. Start by reading the existing files and preserving any work already done.

## Goal
Create the first runnable version of the Rust daemon: configuration, CLI entrypoints, MCP server shell, and the initial database schema.

## Deliverables
Implement the following:

1. **Rust application bootstrap**
- Initialize a Rust project or workspace suitable for a long-running daemon.
- Add a binary entrypoint for the daemon and a small internal module structure for config, DB, MCP, and tools.

2. **Configuration system**
- Support environment-based configuration for:
  - vault path
  - local database path
  - vector index path
  - Ollama base URL
  - HTTP bind address
  - MCP transport mode
  - optional Turso URL and auth token
  - log level
- Add `.env.example` or equivalent checked-in sample configuration.
- Validate config on startup and fail with actionable errors.

3. **CLI surface**
- Add commands similar to:
  - `serve`
  - `migrate`
  - `reindex`
  - `doctor`
- `doctor` should verify filesystem paths, DB connectivity, and Ollama reachability if configured.

4. **Database foundation**
- Create migrations for the initial schema. At minimum include tables for:
  - `notes`
  - `note_chunks`
  - `tasks`
  - `relationships`
  - `conversation_sessions`
  - `conversation_messages`
  - `ingestion_jobs`
  - `tool_invocations`
- Add timestamps, status fields, and stable primary keys.
- Enable FTS5 for searchable note and message text.
- Make the schema idempotent and migration-driven.

5. **MCP server skeleton**
- Implement a minimal MCP server with a registry for tools.
- Support `stdio` transport first.
- If practical, also expose HTTP/SSE for local integration clients, but do not block the milestone on that.
- Register placeholder implementations for:
  - `search_vault`
  - `list_tasks`
  - `ingest_new_file`
  - `commit_interaction`
  - `recall_memories`
- Placeholder tools may return `not yet implemented`, but the registry, schemas, and wiring must exist.

6. **Observability basics**
- Add structured logging and tracing spans around startup, config loading, migrations, and tool calls.
- Include a basic `health_check` tool or endpoint.

## Constraints
- Use a local LibSQL-compatible database by default. Do not require remote Turso credentials to boot.
- Keep storage and vector index behind traits or service interfaces so later prompts can extend them without rewiring the whole app.
- Avoid premature integration with messaging providers in this step.

## Acceptance Criteria
- `cargo run -- serve` starts the daemon successfully with local config.
- `cargo run -- migrate` applies migrations to a fresh database.
- `cargo run -- doctor` reports useful diagnostics.
- The MCP server lists the canonical tools.
- The codebase has at least one unit test and one integration-style test proving startup and migration behavior.

## Final Response Requirements
When you finish, report:
- implemented modules
- commands executed
- test results
- any deliberate shortcuts taken that later prompts must complete
