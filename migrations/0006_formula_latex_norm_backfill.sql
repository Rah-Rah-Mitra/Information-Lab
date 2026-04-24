-- Backfill formula dedupe key to be LaTeX-based (not symbol-set based).
-- This rebuild keeps one earliest row per normalized `latex_norm`.

DROP TABLE IF EXISTS formulas_new;

CREATE TABLE formulas_new (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    latex_norm    TEXT NOT NULL UNIQUE,
    latex         TEXT NOT NULL,
    symbols       TEXT,
    context       TEXT,
    note_rel_path TEXT NOT NULL,
    first_seen_at TEXT NOT NULL DEFAULT (datetime('now'))
);

WITH raw AS (
    SELECT
        id,
        latex,
        symbols,
        context,
        note_rel_path,
        first_seen_at,
        trim(replace(replace(replace(latex, char(10), ' '), char(13), ' '), char(9), ' ')) AS s0
    FROM formulas
),
collapsed AS (
    SELECT
        id, latex, symbols, context, note_rel_path, first_seen_at,
        trim(
            replace(replace(replace(replace(replace(s0, '  ', ' '), '  ', ' '), '  ', ' '), '  ', ' '), '  ', ' ')
        ) AS s1
    FROM raw
),
unwrapped AS (
    SELECT
        id, latex, symbols, context, note_rel_path, first_seen_at,
        CASE
            WHEN length(s1) >= 4 AND substr(s1, 1, 2) = '$$' AND substr(s1, -2, 2) = '$$'
                THEN trim(substr(s1, 3, length(s1) - 4))
            WHEN length(s1) >= 4 AND substr(s1, 1, 2) = '\\(' AND substr(s1, -2, 2) = '\\)'
                THEN trim(substr(s1, 3, length(s1) - 4))
            WHEN length(s1) >= 4 AND substr(s1, 1, 2) = '\\[' AND substr(s1, -2, 2) = '\\]'
                THEN trim(substr(s1, 3, length(s1) - 4))
            WHEN length(s1) >= 2 AND substr(s1, 1, 1) = '$' AND substr(s1, -1, 1) = '$'
                THEN trim(substr(s1, 2, length(s1) - 2))
            ELSE s1
        END AS latex_norm_new
    FROM collapsed
),
ranked AS (
    SELECT
        *,
        row_number() OVER (
            PARTITION BY latex_norm_new
            ORDER BY first_seen_at ASC, id ASC
        ) AS rn
    FROM unwrapped
    WHERE latex_norm_new <> ''
)
INSERT INTO formulas_new (latex_norm, latex, symbols, context, note_rel_path, first_seen_at)
SELECT latex_norm_new, latex, symbols, context, note_rel_path, first_seen_at
FROM ranked
WHERE rn = 1;

DROP TABLE formulas;
ALTER TABLE formulas_new RENAME TO formulas;
CREATE INDEX IF NOT EXISTS idx_formulas_note ON formulas(note_rel_path);
