//! Vector index trait plus a JSON-file fallback.
//!
//! Why not LanceDB by default: the Rust LanceDB binding has moved quickly
//! and historically blocked Rust toolchain upgrades. Keeping the trait lets
//! us swap in LanceDB without changing callers — per Prompt 2's guidance.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredVector {
    pub id: String,
    pub kind: String,    // note_chunk|message|summary
    pub ref_id: String,  // chunk id, message id, etc.
    pub payload: String, // cloud-safe snippet for reranking/display
    pub vector: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorHit {
    pub id: String,
    pub kind: String,
    pub ref_id: String,
    pub payload: String,
    pub score: f32,
}

#[async_trait]
pub trait VectorIndex: Send + Sync {
    async fn upsert(&self, vectors: Vec<StoredVector>) -> Result<()>;
    async fn delete(&self, ids: &[String]) -> Result<()>;
    async fn query(&self, vector: &[f32], kind: Option<&str>, limit: usize) -> Result<Vec<VectorHit>>;
    async fn len(&self) -> Result<usize>;
}

/// Minimal cosine-similarity index persisted as a single JSON file. This is
/// the "fallback" path from Prompt 2. It is intentionally dumb — fine for
/// thousands of vectors, swap to LanceDB at scale.
pub struct FileVectorIndex {
    path: PathBuf,
    inner: Mutex<Vec<StoredVector>>,
}

impl FileVectorIndex {
    pub fn open(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        fs::create_dir_all(&dir).with_context(|| format!("creating {dir:?}"))?;
        let path = dir.join("vectors.json");
        let vectors: Vec<StoredVector> = if path.exists() {
            let bytes = fs::read(&path)?;
            if bytes.is_empty() {
                Vec::new()
            } else {
                serde_json::from_slice(&bytes).context("reading vectors.json")?
            }
        } else {
            Vec::new()
        };
        Ok(Self {
            path,
            inner: Mutex::new(vectors),
        })
    }

    fn flush(&self, guard: &Vec<StoredVector>) -> Result<()> {
        let tmp = self.path.with_extension("json.tmp");
        let data = serde_json::to_vec(guard)?;
        fs::write(&tmp, data)?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}

#[async_trait]
impl VectorIndex for FileVectorIndex {
    async fn upsert(&self, vectors: Vec<StoredVector>) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        for v in vectors {
            if let Some(idx) = guard.iter().position(|x| x.id == v.id) {
                guard[idx] = v;
            } else {
                guard.push(v);
            }
        }
        self.flush(&guard)
    }

    async fn delete(&self, ids: &[String]) -> Result<()> {
        let mut guard = self.inner.lock().unwrap();
        guard.retain(|v| !ids.contains(&v.id));
        self.flush(&guard)
    }

    async fn query(
        &self,
        vector: &[f32],
        kind: Option<&str>,
        limit: usize,
    ) -> Result<Vec<VectorHit>> {
        let guard = self.inner.lock().unwrap();
        let mut hits: Vec<VectorHit> = guard
            .iter()
            .filter(|v| kind.map_or(true, |k| v.kind == k))
            .filter_map(|v| {
                let s = cosine(&v.vector, vector)?;
                Some(VectorHit {
                    id: v.id.clone(),
                    kind: v.kind.clone(),
                    ref_id: v.ref_id.clone(),
                    payload: v.payload.clone(),
                    score: s,
                })
            })
            .collect();
        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        hits.truncate(limit);
        Ok(hits)
    }

    async fn len(&self) -> Result<usize> {
        Ok(self.inner.lock().unwrap().len())
    }
}

fn cosine(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    let mut dot = 0f32;
    let mut na = 0f32;
    let mut nb = 0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 {
        return None;
    }
    Some(dot / (na.sqrt() * nb.sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn upsert_query_roundtrip() {
        let dir = TempDir::new().unwrap();
        let idx = FileVectorIndex::open(dir.path()).unwrap();
        idx.upsert(vec![
            StoredVector {
                id: "a".into(),
                kind: "note_chunk".into(),
                ref_id: "chunk-a".into(),
                payload: "apples".into(),
                vector: vec![1.0, 0.0, 0.0],
            },
            StoredVector {
                id: "b".into(),
                kind: "note_chunk".into(),
                ref_id: "chunk-b".into(),
                payload: "berries".into(),
                vector: vec![0.0, 1.0, 0.0],
            },
        ])
        .await
        .unwrap();
        let hits = idx.query(&[0.9, 0.1, 0.0], None, 2).await.unwrap();
        assert_eq!(hits[0].ref_id, "chunk-a");
    }
}
