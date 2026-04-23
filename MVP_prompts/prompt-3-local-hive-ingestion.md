# Prompt 3: Local Hive And Privacy-First Ingestion

Run this after `prompt-2-vault-and-search.md`.

## Prompt
Continue implementing Cenote in the same repository. Inspect the current code and keep Prompt 1 and Prompt 2 behavior working.

## Goal
Implement the local ingestion pipeline that reads private files, sends them to Ollama-hosted local models, and stores only processed outputs for cloud-facing use.

## Deliverables
Implement the following:

1. **Ollama integration**
- Add a client for Ollama with configurable base URL, model names, timeout, and retry policy.
- Support at minimum:
  - one local reasoning/extraction model such as Llama 3.x
  - one local embedding model if needed for memory and vault indexing
- Centralize prompts/templates for ingestion and extraction so they are easy to revise later.

2. **Supported ingestion inputs**
- Support at minimum:
  - Markdown
  - plain text
  - PDF
- If image OCR is practical in the local environment, support images too. If not, define the parser interface now and return a clear unsupported error until OCR is added.

3. **`ingest_new_file` tool**
- Accept a file path and optional source metadata.
- Read the file locally.
- Parse or extract text locally.
- Send the extracted content to the local model pipeline.
- Produce structured outputs:
  - concise summary
  - tags or topics
  - task candidates
  - key entities
  - related-note hints
- Never expose raw file content to the cloud layer through this tool.

4. **Sidecar summaries**
- Persist a human-readable hidden sidecar summary for each ingested document.
- Use a deterministic path scheme such as adjacent hidden files or a dedicated hidden subdirectory.
- Include provenance metadata in the sidecar so it can be regenerated safely.

5. **Database and relationship updates**
- Store ingestion metadata and extracted entities.
- Upsert task candidates into the `tasks` table with source attribution.
- Persist note-to-note relationships suggested by the local model or lexical overlap.
- Reindex summaries and extracted chunks into the search layer.

6. **Privacy boundaries**
- Create a clear separation between:
  - raw local content
  - local model outputs
  - cloud-safe summaries and search snippets
- Enforce that cloud-facing code paths consume only the latter two.

7. **Tests and fixtures**
- Add fixture documents for Markdown, text, and PDF.
- Test success, malformed input, duplicate ingestion, and forced reingestion.
- Add at least one test or guard proving that raw file content is not returned from `ingest_new_file`.

## Constraints
- Keep the ingestion pipeline modular: parser, extractor, summarizer, task miner, relationship linker.
- If a specific parser crate is unstable, choose a reliable alternative and document the tradeoff.
- The daemon must degrade gracefully when Ollama is unavailable.

## Acceptance Criteria
- `ingest_new_file` works end to end for at least Markdown, text, and PDF.
- Sidecar summaries are written deterministically.
- Tasks and relationships are persisted.
- Search can find ingested summaries and extracted metadata.
- Privacy rules are explicit in code structure and tests.

## Final Response Requirements
When you finish, report:
- supported file types
- local model usage
- sidecar file format and location
- commands executed
- test results
- any remaining gaps before conversation features
