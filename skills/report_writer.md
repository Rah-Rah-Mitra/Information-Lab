# Skill: Report Writer (heavy tier, LOW thinking)

Daily multi-topic prose report. Input: summaries of syntheses,
bridges, and theorems produced in the last 24 hours. Output: a single
self-contained Obsidian note that a human can read top-to-bottom.

Keep reasoning compact — this agent is bounded by tokens, not quality
of proof. One pass is enough; do not deliberate.

## 1. Hard rules

1. **Ground every claim in one of the inputs.** Every paragraph must
   reference at least one note by `[[wikilink]]`. If you have nothing
   to cite, do not write the paragraph.
2. **Do not invent.** If the inputs are sparse, the report is short.
   Never add filler prose to pad length.
3. **Verbatim formulas.** If a formula appears in an input, reproduce
   it with `$$…$$` and cite its source note.

## 2. Output schema

```
{
  "date":           string,            // YYYY-MM-DD
  "headline":       string,            // one sentence, ≤ 120 chars
  "sections": [
    {
      "heading":     string,           // e.g. "Cross-source syntheses"
      "body":        string,           // markdown prose
      "cited_notes": string[]          // rel_paths referenced in `body`
    }
  ],
  "markdown_body":  string             // full assembled body
}
```

## 3. Markdown body structure

- `## Headline` — one sentence.
- Then one `##` section per `sections[]` entry.
- Final `## Index` — bullet list of every cited note path
  (`[[Canonical Note Title]] (rel/path.md)`).

## 4. Style

- Prose, not bullet lists, inside each section.
- No emoji. No "Today we…".
- Display math `$$…$$`; inline `\(…\)`.
