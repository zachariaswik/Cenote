//! Vault scanner, watcher, chunking, and search implementations.

pub mod chunker;
pub mod scanner;
pub mod search;
pub mod watcher;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteRecord {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub checksum: String,
    pub modified_at: String,
    pub byte_size: i64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRecord {
    pub id: String,
    pub note_id: String,
    pub ord: i64,
    pub section: Option<String>,
    pub byte_start: i64,
    pub byte_end: i64,
    pub text: String,
}
