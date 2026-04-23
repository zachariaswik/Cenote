//! Deterministic hidden sidecar summaries. One JSON file per ingested source,
//! placed in `<vault>/.cenote/sidecars/<hash>.json`. Regenerating is safe —
//! it overwrites using the same path.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SidecarSummary {
    pub schema_version: u32,
    pub source_path: String,
    pub checksum: String,
    pub kind: String,
    pub model: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub entities: Vec<String>,
    pub task_candidates: Vec<String>,
    pub related_notes: Vec<String>,
    pub ingested_at: String,
}

pub fn sidecar_path(vault: &Path, sidecar_dir: &str, source: &Path, checksum: &str) -> PathBuf {
    let mut p = vault.join(sidecar_dir).join("sidecars");
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("doc");
    p.push(format!("{stem}.{}.json", &checksum[..12.min(checksum.len())]));
    p
}

pub fn write(path: &Path, summary: &SidecarSummary) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating sidecar dir {parent:?}"))?;
    }
    let data = serde_json::to_vec_pretty(summary)?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

pub fn build(
    source_path: &Path,
    checksum: &str,
    kind: &str,
    model: &str,
    summary: String,
    tags: Vec<String>,
    entities: Vec<String>,
    task_candidates: Vec<String>,
    related_notes: Vec<String>,
) -> SidecarSummary {
    SidecarSummary {
        schema_version: 1,
        source_path: source_path.to_string_lossy().to_string(),
        checksum: checksum.to_string(),
        kind: kind.to_string(),
        model: model.to_string(),
        summary,
        tags,
        entities,
        task_candidates,
        related_notes,
        ingested_at: Utc::now().to_rfc3339(),
    }
}
