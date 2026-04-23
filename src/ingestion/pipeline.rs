//! Ingestion pipeline. Stages: parse → extract (local LLM) → persist → sidecar.

use std::path::Path;

use anyhow::{bail, Context, Result};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::parsers;
use super::sidecar;
use crate::tasks::{self, TaskUpsert};
use crate::vector::StoredVector;
use crate::AppState;

#[derive(Debug, Default)]
pub struct IngestOptions {
    pub source: Option<String>,
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResult {
    pub ok: bool,
    pub status: String, // succeeded|skipped|failed
    pub job_id: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub entities: Vec<String>,
    pub task_candidates: Vec<String>,
    pub related_notes: Vec<String>,
    pub sidecar_path: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct ExtractorJson {
    #[serde(default)]
    summary: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    entities: Vec<String>,
    #[serde(default)]
    task_candidates: Vec<String>,
    #[serde(default)]
    related_notes: Vec<String>,
}

pub async fn ingest(state: &AppState, path: &Path, opts: IngestOptions) -> Result<IngestResult> {
    if !path.exists() {
        bail!("path does not exist: {path:?}");
    }
    let bytes = std::fs::read(path).with_context(|| format!("reading {path:?}"))?;
    let checksum = blake3::hash(&bytes).to_hex().to_string();

    // Duplicate detection.
    let conn = state.db.conn()?;
    let existing: Option<(String, String, Option<String>)> = conn
        .query_row(
            "SELECT id, status, summary FROM ingestion_jobs WHERE source_path = ?1 AND checksum = ?2 ORDER BY created_at DESC LIMIT 1",
            params![path.to_string_lossy(), checksum],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .ok();
    if let Some((job_id, status, summary)) = &existing {
        if !opts.force && status == "succeeded" {
            return Ok(IngestResult {
                ok: true,
                status: "skipped".into(),
                job_id: job_id.clone(),
                summary: summary.clone().unwrap_or_default(),
                tags: vec![],
                entities: vec![],
                task_candidates: vec![],
                related_notes: vec![],
                sidecar_path: None,
            });
        }
    }

    let job_id = uuid::Uuid::new_v4().to_string();
    let (kind, text) = match parsers::extract_text(path) {
        Ok(v) => v,
        Err(e) => {
            conn.execute(
                "INSERT INTO ingestion_jobs(id, source_path, kind, status, checksum, error) VALUES(?1,?2,?3,'failed',?4,?5)",
                params![job_id, path.to_string_lossy(), "unknown", checksum, e.to_string()],
            )?;
            return Ok(IngestResult {
                ok: false,
                status: "failed".into(),
                job_id,
                summary: String::new(),
                tags: vec![],
                entities: vec![],
                task_candidates: vec![],
                related_notes: vec![],
                sidecar_path: None,
            });
        }
    };
    let kind_label = parsers::kind_label(kind).to_string();

    conn.execute(
        "INSERT INTO ingestion_jobs(id, source_path, kind, status, checksum) VALUES(?1,?2,?3,'running',?4)",
        params![job_id, path.to_string_lossy(), kind_label, checksum],
    )?;
    drop(conn);

    let extracted = run_local_extractor(state, &text).await;

    let conn = state.db.conn()?;
    let (summary, tags, entities, candidates, related) = match extracted {
        Ok(ex) => (ex.summary, ex.tags, ex.entities, ex.task_candidates, ex.related_notes),
        Err(e) => {
            // Fallback: lightweight local-only summary.
            tracing::warn!(error=%e, "extractor unavailable; using fallback");
            let s: String = text.lines().filter(|l| !l.trim().is_empty()).take(5).collect::<Vec<_>>().join(" ");
            let s: String = s.chars().take(400).collect();
            (
                s,
                crate::vault::chunker::extract_tags(&text),
                vec![],
                crate::vault::chunker::extract_todos(&text).into_iter().map(|(_, t)| t).collect(),
                vec![],
            )
        }
    };

    // Persist task candidates.
    for (i, title) in candidates.iter().enumerate() {
        let dedupe = format!("ingest:{checksum}:{i}");
        tasks::upsert(
            &conn,
            &TaskUpsert {
                title: title.clone(),
                details: None,
                status: "pending".into(),
                priority: None,
                due_at: None,
                source_kind: "ingestion".into(),
                source_ref: Some(job_id.clone()),
                dedupe_key: dedupe,
            },
        )?;
    }

    // Persist relationships: ingested doc → related note hint.
    for rel in &related {
        let rel_id = uuid::Uuid::new_v4().to_string();
        let _ = conn.execute(
            "INSERT OR IGNORE INTO relationships(id, from_kind, from_id, to_kind, to_id, kind, weight)
             VALUES(?1, 'ingestion', ?2, 'note', ?3, 'related', 0.5)",
            params![rel_id, job_id, rel],
        );
    }

    // Sidecar.
    let sidecar_path = sidecar::sidecar_path(
        &state.config.vault_path,
        &state.config.sidecar_dir,
        path,
        &checksum,
    );
    let s = sidecar::build(
        path,
        &checksum,
        &kind_label,
        state.ollama.extract_model(),
        summary.clone(),
        tags.clone(),
        entities.clone(),
        candidates.clone(),
        related.clone(),
    );
    sidecar::write(&sidecar_path, &s)?;

    // Update job row with cloud-safe outputs + sidecar.
    conn.execute(
        "UPDATE ingestion_jobs SET status='succeeded', summary=?1, tags_json=?2, entities_json=?3, sidecar_path=?4, finished_at=datetime('now') WHERE id=?5",
        params![
            summary,
            serde_json::to_string(&tags)?,
            serde_json::to_string(&entities)?,
            sidecar_path.to_string_lossy(),
            job_id
        ],
    )?;
    drop(conn);

    // Index the summary into the vector index so search_vault can surface it.
    if let Ok(vec) = state.ollama.embed(&summary).await {
        let stored = StoredVector {
            id: format!("ing:{job_id}"),
            kind: "summary".into(),
            ref_id: job_id.clone(),
            payload: summary.chars().take(240).collect(),
            vector: vec.clone(),
        };
        let _ = state.vector.upsert(vec![stored]).await;
    }

    Ok(IngestResult {
        ok: true,
        status: "succeeded".into(),
        job_id,
        summary,
        tags,
        entities,
        task_candidates: candidates,
        related_notes: related,
        sidecar_path: Some(sidecar_path.to_string_lossy().to_string()),
    })
}

async fn run_local_extractor(state: &AppState, text: &str) -> Result<ExtractorJson> {
    // Cap the text to keep prompts bounded.
    let capped: String = text.chars().take(12_000).collect();
    let prompt = format!(
        "You are a local ingestion worker. The user's private file is below. Respond with STRICT JSON matching:\n\
         {{\"summary\": string, \"tags\": string[], \"entities\": string[], \"task_candidates\": string[], \"related_notes\": string[]}}\n\
         Only include information present in the text. No prose outside JSON.\n\n\
         --- FILE START ---\n{capped}\n--- FILE END ---\n"
    );
    let resp = state.ollama.generate(&prompt).await?;
    let json_str = isolate_json(&resp).ok_or_else(|| anyhow::anyhow!("no JSON object in response"))?;
    let parsed: ExtractorJson = serde_json::from_str(&json_str)
        .with_context(|| format!("parsing extractor JSON: {json_str}"))?;
    Ok(parsed)
}

/// Strips everything before the first `{` and after the last `}`.
fn isolate_json(s: &str) -> Option<String> {
    let start = s.find('{')?;
    let end = s.rfind('}')?;
    if end > start {
        Some(s[start..=end].to_string())
    } else {
        None
    }
}
