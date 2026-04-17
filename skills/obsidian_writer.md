# Skill: Obsidian Markdown Conventions

Rules for the `markdown_snippet` field and any other vault content.

## Structure

- Begin with a single sentence that restates the concept in your own
  words — this anchors the note in the Obsidian graph preview.
- Use `##` and `###` for headings. Do NOT use `#` (reserved for the
  auto-generated title).
- Include a final `## Relationships` section whose bullets mirror the
  structured `relationships` array.

## Wikilinks

- Every named concept that appears in the `entities` array MUST be
  written as `[[Entity Name]]` on its FIRST mention in the body.
- Subsequent mentions may be plain text or wikilinks — your call.
- Never nest wikilinks (`[[foo [[bar]]]]`) or put them inside code
  fences.
- Never rename concepts with pipe aliases (`[[Foo|bar]]`) — keep the
  graph canonical.

## Tags

- Tags live in YAML frontmatter (written by the harness, not you).
- Do NOT write `#tag` inline in the body.

## Links and citations

- External URLs go as `[label](https://…)`.
- When a statement comes from a research brief, place the citation at
  the end of the paragraph, not inline.

## What to avoid

- No emoji, no tables of contents, no "In this note we will…".
- No code fences unless the note is genuinely about code.
- No horizontal rules (`---`) — the YAML block already uses them.
- No trailing whitespace, no multiple blank lines.
