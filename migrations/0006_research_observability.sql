-- Research observability expansion for event timelines and API querying.

ALTER TABLE agent_events ADD COLUMN research_request_id TEXT;
ALTER TABLE agent_events ADD COLUMN step_index          INTEGER;
ALTER TABLE agent_events ADD COLUMN phase               TEXT;
ALTER TABLE agent_events ADD COLUMN tool_name           TEXT;
ALTER TABLE agent_events ADD COLUMN model_name          TEXT;
ALTER TABLE agent_events ADD COLUMN prompt_hash         TEXT;
ALTER TABLE agent_events ADD COLUMN response_hash       TEXT;
ALTER TABLE agent_events ADD COLUMN artifact_path       TEXT;

CREATE INDEX IF NOT EXISTS idx_agent_events_request ON agent_events(research_request_id, id);
CREATE INDEX IF NOT EXISTS idx_agent_events_phase   ON agent_events(phase, ts);
