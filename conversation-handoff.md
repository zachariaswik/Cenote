# Conversation Handoff: Hybrid Life OS Project

## Current State
- We have pivoted the original Deno-based "Life OS" (which was web-first and passive) into a **Rust-based Agentic Daemon**.
- **The Goal:** A system where the primary UI is **WhatsApp/Telegram**, using a cloud model for reasoning and local models for private data ingestion.
- **Key Design Decision:** Use the **Model Context Protocol (MCP)** as the bridge between the high-level Assistant and the local File/Database system.

## Strategic Decisions Made
1. **Language:** Switch to **Rust** for the daemon for performance and MCP safety.
2. **Privacy:** All file ingestion (PDFs, notes, images) is parsed by **local LLMs** (Llama 3.2 via Ollama) before the cloud model ever sees a summary.
3. **Permanence:** All chat I/O must be logged to the local DB via a `commit_interaction` tool.
4. **Interface:** Primary interaction via **OpenClaw** (WhatsApp/Telegram), secondary via **LobeChat** (Web).

## Next Steps for the AI
1. Start with [prompts-index.md](/Users/erikwik/Life/Cenote/prompts-index.md).
2. Apply **Prompt 0** first to establish the architectural contract and delivery rules.
3. Execute **Prompts 1 through 6** in order to build the daemon, indexing, ingestion, memory, frontends, and production hardening.
