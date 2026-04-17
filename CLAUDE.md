# CLAUDE.md — guidance for Claude Code working on this repo

This file is loaded automatically by Claude Code. It captures the
project-specific conventions that save you from re-deriving them every
session.

## What this project is

`edge-kg-agent` is a Rust daemon that watches a folder for PDFs and
writes an Obsidian knowledge graph. It runs on a Raspberry Pi on the
free tier of Google AI Studio. Anything you do has to respect those
constraints:

- **RAM is tight.** Prefer streaming over buffering. Chunks are
  processed sequentially, not in parallel.
- **Quota is 15 RPM.** `governor` gates all LLM calls. Don't add
  parallel fan-out to the reasoner without a matching limiter.
- **SQLite is the system of record.** If work gets lost on crash,
  `requeue_orphans()` has to recover it. Don't introduce in-memory-only
  state for anything durable.

## Repo layout (quick)

```
src/            Rust source. Each module has a doc comment explaining its role.
skills/         Markdown instruction files for the LLM agents.
migrations/     SQLx migrations, applied on startup.
systemd/        Production deployment unit.
```

## Before you ship changes

1. **`cargo check`** must pass. The CI-equivalent gate for this repo.
2. If you touched an agent or skill: re-read both `skills/kg_extractor.md`
   and `skills/obsidian_writer.md`. The JSON schema in `agents.rs::kg_schema()`
   and the skill text must agree on field names and constraints.
3. If you touched the vault writer: `INDEX.md` entries must remain
   parseable by `append_index`'s `contains(&marker)` dedup check. The
   marker is `({rel_path})` — keep that format stable.
4. `.env` is local-only and gitignored. Never commit it. `.env.example`
   is the documented surface.

## Conventions

### Agent prompts live in `skills/`, not in Rust

If you need the KG agent to emit a new field or follow a new rule,
edit the relevant `.md` in `skills/` AND update `kg_schema()` plus the
`KgOutput` struct. The skill file is compiled in via `include_str!` —
rebuild to pick up changes. Do NOT put multi-paragraph system prompts
inline in `agents.rs`; the skill files are the supported surface.

### Tracing, not println

Use `tracing::{info,warn,error,debug}!` everywhere. Instrument async
functions with `#[tracing::instrument(level = "info", skip(...))]`.
Avoid `.entered()` span guards in `async` code — they aren't `Send`
and they break `tokio::spawn`. Use `.instrument(span)` on a future
instead, or just let the inner `#[instrument]` attrs carry the span
tree.

### Errors

All fallible functions return `AppResult<T>`. New error variants go in
`src/error.rs`. Prefer `AppError::other(msg)` over adding a variant
for one-off cases.

### Windows vs Unix

This code runs on Linux in production but is developed on Windows. The
watcher and signal handling already handle both. When writing paths in
tests or tools, use `PathBuf::join`, not string concatenation.

### New dependencies

`autoagents`, `opentelemetry-*`, and `sqlx` are version-pinned and
coupled. Don't bump a single crate in the OTel family without bumping
the whole set — API breaks cascade. `tracing-opentelemetry 0.27` pairs
with `opentelemetry 0.26`.

## Obsidian vault conventions

- Generated notes live under `$VAULT_DIR/Generated/{source}/`.
- Filenames are `{slug-of-title}-{yyyymmdd-hhmmss}.md`. Slugs are
  lowercase-kebab, ASCII only, capped at 80 chars.
- Every write updates two indexes: root `INDEX.md` and the source
  folder's `_folder_index.md`. Both are append-only (deduped by rel
  path).
- Frontmatter YAML uses double-quoted scalars with escaped `\\` and `\"`.
- Body always has an H1 title + italic one-line summary + the
  `markdown_snippet`, which itself uses `##` headings only.

## OpenTelemetry

OTel is opt-in via `OTEL_EXPORTER_OTLP_ENDPOINT`. When adding new
async functions on hot paths, add `#[tracing::instrument]` so the
trace tree stays useful. Current first-class spans:

- `ingest` (per PDF, via watcher → orchestrator)
- `extract` (per batch, KG agent call)
- `enrich` (parent for research calls)
- `research` (per concept)
- `write_note` (per generated note)

## Don't

- Don't bypass `governor` with a direct LLM client.
- Don't use `std::fs` in async paths — use `tokio::fs`.
- Don't commit `.env`, `target/`, `logs/`, `state.db`, or the vault.
- Don't add `println!` — it bypasses the file/OTel layers.
- Don't introduce cross-platform-sensitive paths with hardcoded `/` or
  `\`. Use `PathBuf`.
- Don't set the git remote to anything other than
  `https://github.com/Rah-Rah-Mitra/Information-Lab.git` without asking.

## Useful commands

```bash
cargo check                  # must pass before committing
cargo run --release          # local dev run
cargo fmt --check            # formatting (add to CI if not there)
cargo clippy -- -D warnings  # lint gate (optional but recommended)
```

## Known dead code

Three `dead_code` warnings are expected and intentional placeholders —
do not remove without re-checking:

- `KnowledgeGraphAgent::model` — exposed for future status reporting.
- `Config::api_base` — held for an upcoming custom-endpoint path.
- `AppError::Api` — reserved for when we surface upstream status codes.

If you touch those modules for a real reason, wire the fields in
rather than deleting them.
