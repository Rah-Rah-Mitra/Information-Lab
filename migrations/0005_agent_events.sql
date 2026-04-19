-- Agent-interaction event log for discrete-event observability.
--
-- Mirrors ADK-python's agent "event" stream: every LLM call (and a few
-- key lifecycle moments) writes one row so we can reconstruct the
-- back-and-forth between agents after the fact — input summary, output
-- summary, thinking channel, parent/child linkage via OTel trace ids.
--
-- Kept lean: `payload_json` is a freeform blob, but the indexed columns
-- (ts, agent_role, trace_id) give us the discrete-event timeline
-- queries the status page needs without parsing JSON.
--
-- Also extends `api_usage` with per-role counters for the roles added
-- in migration 0004 (Theorem / Derivation / Report / FormulaExtract)
-- that were previously uncounted on the status page.

CREATE TABLE IF NOT EXISTS agent_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    ts              TEXT    NOT NULL DEFAULT (datetime('now')),
    trace_id        TEXT,
    span_id         TEXT,
    parent_span_id  TEXT,
    agent_role      TEXT    NOT NULL,
    event_kind      TEXT    NOT NULL,           -- 'call' | 'thought' | 'result' | 'error'
    input_summary   TEXT,                       -- truncated preview of the input
    output_summary  TEXT,                       -- truncated preview of the output
    thinking        TEXT,                       -- model reasoning, if captured
    payload_json    TEXT,                       -- freeform extra context
    tokens_sent     INTEGER NOT NULL DEFAULT 0,
    tokens_received INTEGER NOT NULL DEFAULT 0,
    duration_ms     INTEGER
);

CREATE INDEX IF NOT EXISTS idx_agent_events_ts       ON agent_events(ts);
CREATE INDEX IF NOT EXISTS idx_agent_events_role     ON agent_events(agent_role, ts);
CREATE INDEX IF NOT EXISTS idx_agent_events_trace    ON agent_events(trace_id);

-- Fill in the per-role counter columns that migration 0003 missed.
ALTER TABLE api_usage ADD COLUMN theorem_calls         INTEGER NOT NULL DEFAULT 0;
ALTER TABLE api_usage ADD COLUMN derivation_calls      INTEGER NOT NULL DEFAULT 0;
ALTER TABLE api_usage ADD COLUMN report_calls          INTEGER NOT NULL DEFAULT 0;
ALTER TABLE api_usage ADD COLUMN formula_extract_calls INTEGER NOT NULL DEFAULT 0;
