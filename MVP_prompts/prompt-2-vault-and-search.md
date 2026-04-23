# Prompt 2: Vault Watching, Indexing, And Retrieval

Run this after `prompt-1-core-daemon-and-schema.md`.

## Prompt
Continue implementing Cenote in the same repository. Read the current code first and extend it without breaking Prompt 1 behavior.

## Goal
Make the daemon useful for local knowledge retrieval by indexing the vault, watching for file changes, and implementing real `search_vault` and `list_tasks` behavior.

## Deliverables
Implement the following:

1. **Vault scanner**
- Recursively scan the configured vault path.
- Track Markdown files first.
- Store file path, checksum, modified time, title, tags if present, and indexing state in the database.
- Make rescans idempotent.

2. **File watcher**
- Use `notify` or a similarly robust Rust crate.
- Debounce rapid changes.
- Queue changed files for indexing instead of doing all work on the watcher thread.
- Handle create, modify, rename, and delete events.

3. **Chunking and FTS indexing**
- Parse Markdown into clean text chunks suitable for both keyword and semantic search.
- Populate `note_chunks` and the FTS tables.
- Preserve references back to source note path and chunk offsets or section identifiers.

4. **Vector indexing abstraction**
- Implement a `VectorIndex` interface.
- Back it with LanceDB if the current Rust integration is practical.
- If LanceDB blocks progress, ship a fallback implementation behind the same trait and document the divergence clearly.
- Do not change public tool contracts if you need a fallback.

5. **`search_vault` tool**
- Accept a query and optional parameters such as limit, mode, tags, and path filters.
- Support:
  - keyword search
  - semantic search
  - hybrid ranking
- Return concise results with:
  - note title
  - note path
  - excerpt or summary
  - relevance score
  - source section or chunk reference
- Do not return the full note by default.

6. **`list_tasks` tool**
- Return pending and in-progress tasks from the database.
- Support filters for status, due date, source, and limit.
- Define a stable JSON schema for the tool response.

7. **Fixtures and tests**
- Add a small fixture vault with Markdown notes and TODOs.
- Test full indexing on a fresh database.
- Test that a file change updates search results.
- Test that search returns relevant snippets instead of whole documents.

## Constraints
- This step is about vault indexing and retrieval, not Ollama-based summarization yet.
- Keep the watcher and indexing queue resilient to malformed files and transient write events.
- Make sure deletes remove or tombstone stale search entries.

## Acceptance Criteria
- The daemon can scan a fixture vault and populate the DB.
- `search_vault` returns useful keyword results.
- Hybrid or semantic search plumbing exists behind the `VectorIndex` interface.
- `list_tasks` returns structured task records from indexed content.
- Tests cover initial scan, update, and delete behavior.

## Final Response Requirements
When you finish, report:
- how indexing works
- how hybrid search is ranked
- commands executed
- test results
- any remaining limitations before Prompt 3
