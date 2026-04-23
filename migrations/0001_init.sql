-- Cenote initial schema: notes, chunks, tasks, relationships, FTS.

CREATE TABLE IF NOT EXISTS notes (
    id            TEXT PRIMARY KEY,
    path          TEXT NOT NULL UNIQUE,
    title         TEXT,
    checksum      TEXT NOT NULL,
    modified_at   TEXT NOT NULL,
    byte_size     INTEGER NOT NULL,
    tags_json     TEXT NOT NULL DEFAULT '[]',
    status        TEXT NOT NULL DEFAULT 'indexed', -- indexed|pending|stale|deleted
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_notes_status ON notes(status);

CREATE TABLE IF NOT EXISTS note_chunks (
    id           TEXT PRIMARY KEY,
    note_id      TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    ord          INTEGER NOT NULL,
    section      TEXT,
    byte_start   INTEGER NOT NULL,
    byte_end     INTEGER NOT NULL,
    text         TEXT NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_note_chunks_note ON note_chunks(note_id);

CREATE TABLE IF NOT EXISTS tasks (
    id            TEXT PRIMARY KEY,
    title         TEXT NOT NULL,
    details       TEXT,
    status        TEXT NOT NULL DEFAULT 'pending', -- pending|in_progress|done|cancelled
    priority      TEXT,
    due_at        TEXT,
    source_kind   TEXT NOT NULL,                    -- note|chat|ingestion
    source_ref    TEXT,                             -- note id, message id, file path
    dedupe_key    TEXT UNIQUE,                      -- stable key to prevent duplicates
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_due ON tasks(due_at);

CREATE TABLE IF NOT EXISTS relationships (
    id            TEXT PRIMARY KEY,
    from_kind     TEXT NOT NULL, -- note|task|message|entity
    from_id       TEXT NOT NULL,
    to_kind       TEXT NOT NULL,
    to_id         TEXT NOT NULL,
    kind          TEXT NOT NULL, -- mentions|links|derived_from|about
    weight        REAL NOT NULL DEFAULT 1.0,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (from_kind, from_id, to_kind, to_id, kind)
);

-- FTS5 over note chunks; content table pattern keeps storage efficient.
CREATE VIRTUAL TABLE IF NOT EXISTS note_chunks_fts USING fts5(
    text,
    section UNINDEXED,
    note_id UNINDEXED,
    content='note_chunks',
    content_rowid='rowid',
    tokenize='porter unicode61'
);

-- Tool invocation audit log.
CREATE TABLE IF NOT EXISTS tool_invocations (
    id            TEXT PRIMARY KEY,
    tool          TEXT NOT NULL,
    args_json     TEXT NOT NULL,
    result_json   TEXT,
    error         TEXT,
    session_id    TEXT,
    started_at    TEXT NOT NULL DEFAULT (datetime('now')),
    ended_at      TEXT
);
CREATE INDEX IF NOT EXISTS idx_tool_invocations_tool ON tool_invocations(tool);
