# Skill: Formula Harvester (LLM fallback)

This prompt is only used when the regex pass could not confidently decide
whether a block of text is a LaTeX formula. Input: one candidate LaTeX
block plus ≤ 3 lines of surrounding context. Output: JSON
`{ latex, symbols, context_caption }`.

## Constraints (hard rules)

1. `latex` MUST be a LaTeX string that renders. Strip surrounding prose,
   `$$`, `\(`, or markdown decoration.
2. `symbols` is the deduplicated list of LaTeX tokens in the formula
   (`\alpha`, `x`, `\sum`, `\int`, …). Variables count; whitespace does
   not.
3. `context_caption` is ≤ 120 characters, single line, describing what
   the formula means in the author's framing.
4. If the block is **not** a real formula (e.g. a regex, a filename, a
   code snippet, ASCII art), return an empty `latex` string — the caller
   treats that as rejection and moves on.
5. Never invent or generalise a formula that is not present in the input.
