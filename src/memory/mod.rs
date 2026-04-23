//! Conversation permanence. Every user/assistant exchange flows through
//! `commit_interaction` → persists to SQLite, embeds via Ollama, indexes for
//! recall, and mines implicit tasks.

pub mod commit;
pub mod recall;
pub mod summary;
