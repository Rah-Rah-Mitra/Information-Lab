-- Per-day Tavily budget tracking for centralized reservation checks.
CREATE TABLE IF NOT EXISTS search_usage_daily (
    day          TEXT PRIMARY KEY,
    calls        INTEGER NOT NULL DEFAULT 0,
    last_call_at TEXT
);
