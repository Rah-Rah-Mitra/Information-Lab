-- PDFs we've seen. Hash-keyed to dedupe re-syncs.
CREATE TABLE IF NOT EXISTS documents (
    hash        TEXT PRIMARY KEY,
    path        TEXT NOT NULL,
    byte_size   INTEGER NOT NULL,
    discovered_at TEXT NOT NULL DEFAULT (datetime('now')),
    extracted_at  TEXT
);

-- Text chunks awaiting reasoner processing.
CREATE TABLE IF NOT EXISTS chunks (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    doc_hash      TEXT NOT NULL REFERENCES documents(hash) ON DELETE CASCADE,
    page_start    INTEGER NOT NULL,
    page_end      INTEGER NOT NULL,
    content       TEXT NOT NULL,
    token_estimate INTEGER NOT NULL,
    state         TEXT NOT NULL CHECK (state IN ('pending', 'batched', 'done', 'error')),
    batch_id      TEXT,
    last_error    TEXT,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_chunks_state ON chunks(state);
CREATE INDEX IF NOT EXISTS idx_chunks_doc   ON chunks(doc_hash);

-- API usage ledger (per-day).
CREATE TABLE IF NOT EXISTS api_usage (
    day             TEXT PRIMARY KEY,  -- YYYY-MM-DD
    reasoner_calls  INTEGER NOT NULL DEFAULT 0,
    vision_calls    INTEGER NOT NULL DEFAULT 0,
    tokens_sent     INTEGER NOT NULL DEFAULT 0,
    tokens_received INTEGER NOT NULL DEFAULT 0
);
