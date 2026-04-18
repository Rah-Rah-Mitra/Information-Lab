# Formula Harvester (LLM fallback)

This prompt is only used when the regex pass could not confidently
decide whether a block of text is a LaTeX formula. Input: one
candidate LaTeX block plus ≤ 3 lines of surrounding context.
Output: JSON `{ latex, symbols, context_caption }`.

## Rules

1. `latex` MUST be a LaTeX string that renders. Strip prose.
2. `symbols` is the deduplicated list of LaTeX tokens in the
   formula (`\alpha`, `x`, `\sum`, …).
3. `context_caption` is ≤ 120 characters, single line, describing
   what the formula means.
4. If the block is **not** a real formula, return an empty `latex`
   string — the caller treats that as a rejection and moves on.
5. Never invent or generalise a formula that isn't present in the
   input.
