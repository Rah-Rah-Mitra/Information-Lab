-- Multi-agent research architecture.
--
-- Adds:
--   * A generic agent-task queue (Curate / Bridge / Harvest).
--   * A bridges table: recorded cross-source hypotheses so we never
--     re-propose the same pair.
--   * Topic snapshots: per-topic counters that let the idle scheduler
--     tell whether a topic has grown enough to be worth curating.
--   * Formula corpus: regex + LLM-normalised LaTeX blocks harvested from
--     Generated/**.md, deduped by normalised symbol set.
--   * Monthly Tavily usage counter (1000 req/mo is the tightest quota
--     in the whole system).
--   * Per-role call counters on api_usage so the status page can see how
--     the daily Gemma budget got spent.

CREATE TABLE IF NOT EXISTS agent_tasks (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    kind         TEXT NOT NULL CHECK (kind IN ('Extract','Curate','Bridge','Harvest')),
    payload      TEXT NOT NULL,                   -- json, agent-specific
    state        TEXT NOT NULL DEFAULT 'pending'
                 CHECK (state IN ('pending','running','done','error')),
    batch_id     TEXT,
    last_error   TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_state ON agent_tasks(state, kind);

CREATE TABLE IF NOT EXISTS bridges (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    topic_a       TEXT NOT NULL,
    topic_b       TEXT NOT NULL,
    source_a      TEXT NOT NULL,
    source_b      TEXT NOT NULL,
    confidence    REAL NOT NULL,
    iterations    INTEGER NOT NULL,
    note_rel_path TEXT NOT NULL,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(topic_a, topic_b, source_a, source_b)
);

CREATE TABLE IF NOT EXISTS topic_snapshots (
    topic            TEXT PRIMARY KEY,
    entry_count      INTEGER NOT NULL,
    last_curated_at  TEXT,
    last_snapshot_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS formulas (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    latex_norm    TEXT NOT NULL UNIQUE,
    latex         TEXT NOT NULL,
    symbols       TEXT,
    context       TEXT,
    note_rel_path TEXT NOT NULL,
    first_seen_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_formulas_note ON formulas(note_rel_path);

-- Tavily monthly budget tracking. Row per YYYY-MM.
CREATE TABLE IF NOT EXISTS search_usage (
    month        TEXT PRIMARY KEY,
    calls        INTEGER NOT NULL DEFAULT 0,
    last_call_at TEXT
);

-- Extend api_usage with per-role Gemma call counters. SQLite's ALTER only
-- supports single-column adds; each column is idempotent under IF NOT
-- EXISTS-less ALTER, so we guard with migration ordering instead.
ALTER TABLE api_usage ADD COLUMN curator_calls   INTEGER NOT NULL DEFAULT 0;
ALTER TABLE api_usage ADD COLUMN bridge_calls    INTEGER NOT NULL DEFAULT 0;
ALTER TABLE api_usage ADD COLUMN harvester_calls INTEGER NOT NULL DEFAULT 0;
ALTER TABLE api_usage ADD COLUMN search_calls    INTEGER NOT NULL DEFAULT 0;
