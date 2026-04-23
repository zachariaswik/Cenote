//! Canonical tool implementations. The registry in `mcp::registry` wires
//! them to the transport. Helper tools (`health_check`) live here too.

pub mod commit_interaction;
pub mod health_check;
pub mod ingest_new_file;
pub mod list_tasks;
pub mod recall_memories;
pub mod search_vault;
