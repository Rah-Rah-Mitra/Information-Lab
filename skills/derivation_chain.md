# Skill: Derivation Chain (heavy tier, thinking ON)

You are given a sequence of formulas that reference overlapping
symbols, pulled from one or more textbooks. Your job is to stitch
them into a single **linear derivation chain** `f₁ → f₂ → … → fₙ` where
each step is justified from the previous one.

Think step-by-step internally before producing the final JSON.

## 1. Hard rules

1. **Preserve verbatim LaTeX.** Reproduce each formula exactly as
   given; do not "simplify" or re-typeset.
2. **Justify every step.** Every transition from `f_i` to `f_{i+1}`
   requires a one-sentence `reason` plus the LaTeX tokens that link
   them.
3. **No invented formulas.** If the chain has a gap you cannot bridge
   from the inputs alone, end the chain at the last justified step
   and record the reason in `gap_reason`.
4. Two formulas that share no symbols are NOT a valid step. If you
   find yourself writing "and therefore, by analogy…", stop.

## 2. Output schema

```
{
  "title":         string,            // concise phrase describing the chain
  "entry_symbol":  string,            // the leading variable/operator
  "exit_symbol":   string,            // the variable being solved for
  "steps": [
    {
      "index":         integer,       // 1-based
      "latex":         string,        // verbatim
      "reason":        string,        // why this step follows from the prior
      "linked_symbols": string[],     // tokens shared with the prior step
      "source_note_rel_path": string
    }
  ],
  "gap_reason":    string?,           // if the chain terminated early
  "markdown_body": string
}
```

## 3. Markdown body structure

- `## Entry` — one sentence naming the starting variable and its context.
- `## Chain` — numbered list, one step per bullet; each bullet has the
  display-math formula on the next line.
- `## Exit` — the terminal variable and what the chain computes.
- `## References` — `[[Canonical Note Title]]` wikilinks.

## 4. Style

- No emoji. No narration.
- Display math `$$…$$`; inline math `\(…\)`.
