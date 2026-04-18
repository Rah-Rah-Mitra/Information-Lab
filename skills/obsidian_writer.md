<start_of_turn>user
# Skill: Obsidian Markdown Conventions

Rules for the `markdown_snippet` field and any vault content.

## Note types

Every file the agent produces falls into one of two types, declared in YAML frontmatter by the harness:

- `type: content` — a generated concept note under `Generated/<source>/`. Has a title, summary, entities, relationships, body.
- `type: index` — a navigation file under `Index.md` or `Index/**/*.md`. Lists child notes and sub-indexes as wikilinks with one-line summaries.

This distinction is the navigation signal other agents use to decide whether to open a file for browsing (index) vs reading in full (content). Never mix the two in the same file.

## Body structure (content notes)

- Begin with one sentence restating the concept in your own words — anchors the note in the Obsidian graph preview.
- Use `##` and `###` for headings. Do NOT use `#` (reserved for the auto-generated title).
- End with a `## Relationships` section whose bullets mirror the structured `relationships` array.

## Wikilinks and cross-textbook linking

The library holds multiple graduate-level textbooks (GIS, math, stats, CV, …). A single concept — "Principal Component Analysis", "Kernel Density Estimation", "Gradient Descent" — often appears in several of them. Wikilinks are how those appearances get unified into one graph node.

- Every entity in the `entities` array MUST appear as `[[Canonical Name]]` on its FIRST mention in the body.
- Always use the CANONICAL full name, not an abbreviation, so the same concept in different textbooks resolves to the same note (`[[Principal Component Analysis]]`, not `[[PCA]]`).
- Never nest wikilinks (`[[foo [[bar]]]]`).
- Never use pipe aliases (`[[Foo|bar]]`) — keep the graph canonical.
- Never put wikilinks inside code fences.
- Subsequent mentions may be plain text.

## Index files

Index entries are single lines of the form:

```
- [[Canonical Note Title]] — one-sentence summary ≤ 160 chars. (relative/path.md)
```

The `(relative/path.md)` marker is required — the harness uses it to dedupe. Do not remove it. Index files never contain body prose beyond a short header paragraph.

## Tags

- Tags live in YAML frontmatter (written by the harness, not you).
- Do NOT write `#tag` inline in the body.

## Links and citations

- External URLs: `[label](https://…)`.
- Citations go at the end of the paragraph, not inline.

## What to avoid

- No emoji, no "In this note we will…".
- No code fences unless the note is genuinely about code.
- No horizontal rules (`---`) in the body — YAML frontmatter already uses them.
- No trailing whitespace, no multiple blank lines.
<end_of_turn>
<start_of_turn>model
