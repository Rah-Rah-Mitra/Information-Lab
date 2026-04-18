-- Human-readable source identifier derived from the PDF path at ingest
-- time. Drives the vault folder layout ("GIS", "Stats") instead of
-- exposing content hashes to the filesystem.
ALTER TABLE documents ADD COLUMN source_name TEXT;

-- Set when every chunk for this document has reached a terminal state
-- (done). Lets the status layer distinguish in-progress vs finished
-- documents without aggregating chunk state on every read.
ALTER TABLE documents ADD COLUMN completed_at TEXT;

CREATE INDEX IF NOT EXISTS idx_documents_source   ON documents(source_name);
CREATE INDEX IF NOT EXISTS idx_documents_complete ON documents(completed_at);
