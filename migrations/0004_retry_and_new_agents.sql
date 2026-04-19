-- Retry tracking + new agent-task kinds.
--
-- Two things this migration does:
--   1. Adds `retry_count` / `last_retry_at` to `chunks`, and extends
--      `chunks.state` with `'failed_final'` (terminal after N retries).
--   2. Extends `agent_tasks.kind` with the new research + recovery agents
--      (`Theorem`, `Derivation`, `Report`, `FormulaExtract`, `ErrorRetry`).
--
-- SQLite cannot rewrite a CHECK constraint with ALTER, so both tables are
-- rebuilt. Data is copied verbatim; the new columns default for existing
-- rows.

PRAGMA foreign_keys = OFF;

-- ---- chunks rebuild -------------------------------------------------------
CREATE TABLE chunks_new (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    doc_hash       TEXT NOT NULL REFERENCES documents(hash) ON DELETE CASCADE,
    page_start     INTEGER NOT NULL,
    page_end       INTEGER NOT NULL,
    content        TEXT NOT NULL,
    token_estimate INTEGER NOT NULL,
    state          TEXT NOT NULL
                   CHECK (state IN ('pending','batched','done','error','failed_final')),
    batch_id       TEXT,
    last_error     TEXT,
    retry_count    INTEGER NOT NULL DEFAULT 0,
    last_retry_at  TEXT,
    created_at     TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO chunks_new (
    id, doc_hash, page_start, page_end, content, token_estimate,
    state, batch_id, last_error, retry_count, last_retry_at,
    created_at, updated_at
)
SELECT
    id, doc_hash, page_start, page_end, content, token_estimate,
    state, batch_id, last_error, 0, NULL,
    created_at, updated_at
FROM chunks;

DROP TABLE chunks;
ALTER TABLE chunks_new RENAME TO chunks;

CREATE INDEX IF NOT EXISTS idx_chunks_state ON chunks(state);
CREATE INDEX IF NOT EXISTS idx_chunks_doc   ON chunks(doc_hash);
CREATE INDEX IF NOT EXISTS idx_chunks_retry ON chunks(state, last_retry_at);

-- ---- agent_tasks rebuild --------------------------------------------------
CREATE TABLE agent_tasks_new (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    kind         TEXT NOT NULL CHECK (kind IN (
                    'Extract','Curate','Bridge','Harvest',
                    'Theorem','Derivation','Report','FormulaExtract','ErrorRetry'
                 )),
    payload      TEXT NOT NULL,
    state        TEXT NOT NULL DEFAULT 'pending'
                 CHECK (state IN ('pending','running','done','error')),
    batch_id     TEXT,
    last_error   TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT
);

INSERT INTO agent_tasks_new (
    id, kind, payload, state, batch_id, last_error, created_at, completed_at
)
SELECT
    id, kind, payload, state, batch_id, last_error, created_at, completed_at
FROM agent_tasks;

DROP TABLE agent_tasks;
ALTER TABLE agent_tasks_new RENAME TO agent_tasks;

CREATE INDEX IF NOT EXISTS idx_agent_tasks_state ON agent_tasks(state, kind);

PRAGMA foreign_keys = ON;
