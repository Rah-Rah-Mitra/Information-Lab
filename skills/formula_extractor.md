# Skill: Formula Extractor (light tier, thinking OFF)

You are given a short slice of textbook text that a local heuristic flagged
as **math-dense**. Your job is to recover the equations that pdf_oxide
extraction lost or garbled, and emit them as normalised LaTeX.

This is a *salvage* pass, not a reasoning pass. Do not explain, do not
prove, do not comment on what the formulas mean.

## 1. Hard rules

1. **Only emit formulas that are visibly present in the input.** If the
   input merely talks *about* an equation without quoting it, skip it.
2. **Preserve the math verbatim.** Do not simplify, rename variables, or
   combine two lines of display math into one.
3. **Normalise encoding.** `∑` → `\sum`, `∫` → `\int`, `∂` → `\partial`,
   `α β γ` → `\alpha \beta \gamma`, Greek capitals likewise. Keep the
   result as clean LaTeX that compiles with `amsmath` + `amssymb`.
4. **Preserve matrices and vectors verbatim.** Bold identifiers stay as
   `\mathbf{X}`, `\mathbf{P}`, `\boldsymbol{\theta}`. Stacked matrix /
   vector layouts become `\begin{pmatrix}…\end{pmatrix}` (or `bmatrix`
   if the source uses square brackets). Row separators are `\\`,
   column separators are `&`. Never flatten a matrix into an inline
   product.
5. **No invented context.** If you cannot confidently caption a formula
   in one short phrase from the surrounding sentence, use `""`.
6. **No chat-template tokens, no control characters.** Never emit
   `<start_of_turn>`, `<end_of_turn>`, `<|turn>`, `<|channel>`,
   `<|think|>`, `<|tool>`, NUL bytes (`\u0000`), or any replacement
   characters (`\uFFFD`). These are runtime tokens, not LaTeX.
7. **Double every backslash inside JSON.** The output is a JSON string,
   so `\frac` must be emitted as `"\\frac"`, `\nabla` as `"\\nabla"`,
   `\begin{pmatrix}` as `"\\begin{pmatrix}"`. A single backslash in
   JSON is an escape introducer and will destroy the LaTeX command.
8. If there are zero formulas in the input, return `{"formulas": []}`.

## 2. Output schema

```
{
  "formulas": [
    {
      "latex":            string,   // canonical LaTeX, no $-delimiters
      "symbols":          string[], // identifiers appearing in the formula
      "context_caption":  string    // ≤120 chars, from the surrounding text
    }
  ]
}
```

## 3. Style

- No prose outside JSON. No emoji. No narration.
- Each LaTeX string is a single equation body — do not wrap it in `$…$`
  or `\[…\]`. The downstream writer adds display delimiters.
- `symbols` is for cross-formula linking in the derivation agent; include
  each distinct identifier once.
