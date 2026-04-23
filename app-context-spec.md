# Application Specification: Hybrid Life OS

## Overview
A "Hybrid Hive" architecture where a high-reasoning cloud model (Cerebrum) orchestrates a local Rust daemon (Nervous System) and a collection of small local models (The Hive).

## Key Components
- **The Cerebrum (Cloud):** Claude 3.5 / Gemini 1.5. Handles complex reasoning.
- **The Daemon (Local):** Rust-based MCP Server. Manages files, SQLite, and Tooling.
- **The Hive (Local):** Llama 3.2 (1B/3B). Handles private data parsing and task mining.
- **The Primary UI:** WhatsApp/Telegram (via OpenClaw) for ubiquitous, persistent interaction.

## Technical Standards
- **Protocol:** Model Context Protocol (MCP).
- **Storage:** Turso (Metadata/Tasks), LanceDB (Vector Search), Markdown (Human-readable Knowledge).
- **Privacy:** Raw data ingestion is strictly local. The cloud model only interacts with processed summaries or search results.
- **Permanence:** 100% of chat I/O is logged to the local database and indexed for semantic recall.
