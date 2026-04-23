-- Conversation permanence: sessions, messages, FTS, rolling summaries.

CREATE TABLE IF NOT EXISTS conversation_sessions (
    id              TEXT PRIMARY KEY,
    channel         TEXT NOT NULL,         -- whatsapp|telegram|web|mcp|cli
    external_id     TEXT,                  -- thread/chat id from provider
    user_handle     TEXT,
    started_at      TEXT NOT NULL DEFAULT (datetime('now')),
    last_message_at TEXT,
    UNIQUE (channel, external_id)
);

CREATE TABLE IF NOT EXISTS conversation_messages (
    id              TEXT PRIMARY KEY,
    session_id      TEXT NOT NULL REFERENCES conversation_sessions(id) ON DELETE CASCADE,
    role            TEXT NOT NULL,          -- user|assistant|system|tool
    content         TEXT NOT NULL,
    tool_metadata   TEXT,                   -- json blob describing tool calls
    attachments     TEXT,                   -- json array of file refs
    external_msg_id TEXT,                   -- id from provider, used for idempotency
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (session_id, external_msg_id, role)
);
CREATE INDEX IF NOT EXISTS idx_messages_session ON conversation_messages(session_id);
CREATE INDEX IF NOT EXISTS idx_messages_created ON conversation_messages(created_at);

CREATE VIRTUAL TABLE IF NOT EXISTS conversation_messages_fts USING fts5(
    content,
    role UNINDEXED,
    session_id UNINDEXED,
    content='conversation_messages',
    content_rowid='rowid',
    tokenize='porter unicode61'
);

CREATE TABLE IF NOT EXISTS conversation_summaries (
    id            TEXT PRIMARY KEY,
    session_id    TEXT NOT NULL REFERENCES conversation_sessions(id) ON DELETE CASCADE,
    summary       TEXT NOT NULL,
    covers_from   TEXT NOT NULL,
    covers_to     TEXT NOT NULL,
    created_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_summaries_session ON conversation_summaries(session_id);
