-- Extend agent_tasks.kind with ad-hoc API research requests.
--
-- SQLite cannot alter CHECK constraints in place, so rebuild table.

PRAGMA foreign_keys = OFF;

CREATE TABLE agent_tasks_new (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    kind         TEXT NOT NULL CHECK (kind IN (
                    'Extract','Curate','Bridge','Harvest',
                    'Theorem','Derivation','Report','FormulaExtract','ErrorRetry',
                    'ResearchRequest'
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
