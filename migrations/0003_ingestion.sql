-- Ingestion jobs + vector embeddings sidecar index + extracted entities.

CREATE TABLE IF NOT EXISTS ingestion_jobs (
    id            TEXT PRIMARY KEY,
    source_path   TEXT NOT NULL,
    kind          TEXT NOT NULL,            -- markdown|text|pdf|image
    status        TEXT NOT NULL,            -- queued|running|succeeded|failed|skipped
    checksum      TEXT,
    error         TEXT,
    summary       TEXT,
    tags_json     TEXT,
    entities_json TEXT,
    sidecar_path  TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    finished_at   TEXT
);
CREATE INDEX IF NOT EXISTS idx_ingestion_status ON ingestion_jobs(status);
CREATE INDEX IF NOT EXISTS idx_ingestion_source ON ingestion_jobs(source_path);

-- Vector index metadata table. Vectors themselves live on disk via the
-- VectorIndex trait, but we keep a row per vector for joins and cleanup.
CREATE TABLE IF NOT EXISTS embeddings (
    id            TEXT PRIMARY KEY,         -- same id used in the on-disk vector store
    kind          TEXT NOT NULL,            -- note_chunk|message|summary
    ref_id        TEXT NOT NULL,            -- FK target (chunk id, message id, etc.)
    dim           INTEGER NOT NULL,
    created_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_embeddings_ref ON embeddings(kind, ref_id);
