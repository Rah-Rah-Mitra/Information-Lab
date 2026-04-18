# CLAUDE.md — guidance for Claude Code working on this repo

This file is loaded automatically by Claude Code. It captures the
project-specific conventions that save you from re-deriving them every
session.

## What this project is

`edge-kg-agent` is a Rust daemon that watches a folder for PDFs and
writes an Obsidian knowledge graph, then runs a research layer on top
(Curator, BridgeFinder, FormulaHarvester, LiteratureSearch) that
synthesises and bridges topics across textbooks. It runs on a Raspberry
Pi on the free tier of Google AI Studio. Anything you do has to respect
those constraints:

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
deploy/otel/    Local Jaeger all-in-one compose for OTel development.
public/         Default watch + vault directory (sample GIS textbook included).
.data/          Runtime state (SQLite DB). Gitignored.
```

Default paths: `WATCH_DIR=./public`, `VAULT_DIR=./public`, `DB_PATH=./.data/state.db`.

## Before you ship changes

1. **`cargo check`** must pass. The CI-equivalent gate for this repo.
2. If you touched an agent or skill: re-read both `skills/kg_extractor.md`
   and `skills/obsidian_writer.md`. The JSON schema in `agents.rs::kg_schema()`
   and the skill text must agree on field names and constraints.
3. If you touched the vault writer: every index entry must end with the
   trailing `({rel_path})` marker — `upsert_index_entry`'s dedup and
   `split_index`'s bucket re-bucketing both rely on that format. Don't
   change the wikilink-then-marker shape of `render_entry_line`.
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

Two-axis hierarchical index. Generated content lives alongside
human-readable index files that distinguish `type: index` from
`type: content` in frontmatter — that's the navigation signal future
agents use to decide what to load.

```
{vault}/
  Index.md                     type: index (root) — lists Sources + Topics
  Sources/{source}.md          type: index, index_kind: source
  Topics/{tag}.md              type: index, index_kind: topic   (cross-source)
  Generated/{source}/{slug}-{yyyymmdd-hhmmss}.md   type: content
```

- Filenames are `{slug-of-title}-{yyyymmdd-hhmmss}.md`. Slugs are
  lowercase-kebab, ASCII only, capped at 80 chars.
- Every write updates three axes: the source index, every topic index
  referenced by the note's tags (cross-textbook aggregation), and the
  root `Index.md` (one bullet per source + one per topic).
- Index entries are append-only and deduped by the `({rel_path})`
  marker at the end of each bullet.
- When an index exceeds `INDEX_ENTRY_CAP` (default 20), it splits into
  alphabetical bucket children `a-e / f-j / k-o / p-t / u-z / other`
  under a same-named sub-directory, and the original file becomes a
  split-parent pointer (`split: true` in frontmatter). Buckets are
  stable — adding entries never renames an existing bucket file.
- Frontmatter YAML uses double-quoted scalars with escaped `\\` and `\"`.
- Content body has an H1 title + italic one-line summary + the
  `markdown_snippet`, which itself uses `##` headings only.
- Entities and wikilinks inside `markdown_snippet` should use canonical
  long-form names (e.g. `[[Principal Component Analysis]]`, not
  `[[PCA]]`) so cross-textbook Topic aggregation actually lines up.

## OpenTelemetry

OTel is opt-in via `OTEL_EXPORTER_OTLP_ENDPOINT`. A local Jaeger
all-in-one backend is provided under `deploy/otel/docker-compose.yml`
(OTLP on :4317/:4318, UI at http://localhost:16686). When adding new
async functions on hot paths, add `#[tracing::instrument]` so the
trace tree stays useful. Current first-class spans:

- `ingest` (per PDF, via watcher → orchestrator)
- `extract` (per batch, KG agent call)
- `write_note` (per generated note)
- `curate` / `bridge.iterate` / `bridge.propose` / `bridge.critique`
  (research layer)
- `harvest` / `scheduler.tick` / `limiter.admit` / `search.tavily`

## Multi-agent research layer

Five agents, all gated through a single shared `Limiter` (global
governor + per-role semaphore + per-role daily counter shaped against
`RPD_LIMIT`, default 1500):

- **ExtractorAgent** (`src/agents/extractor.rs`) — PDF chunks → KG notes.
  Still owns its own governor for now; all other agents use the shared
  `Limiter`.
- **TopicCuratorAgent** (`src/agents/curator.rs`) — on topic growth ≥
  `CURATE_DELTA_K`, synthesises a cross-source note into
  `Generated/_Syntheses/`.
- **BridgeFinderAgent** (`src/agents/bridge.rs`) — 3-iteration loop
  (propose → Tavily-refine → critique); emits to `Generated/_Bridges/`
  when `confidence ≥ BRIDGE_CONFIDENCE_TAU` (0.72).
- **LiteratureSearchAgent** (`src/agents/search.rs`) — Tavily client
  restricted to six academic domains; monthly cap via `search_usage`.
  Only called from Bridge iter 2. Do NOT resurrect the old ReAct/Tavily
  orchestrator paths.
- **FormulaHarvesterAgent** (`src/agents/harvester.rs`) — regex-first
  scan of `Generated/**.md`; LLM fallback only for ambiguous blocks.
  Rewrites `Formulas.md` at vault root.

Daily role shares default to 60/20/15/5 (Extractor/Curator/Bridge/
Harvester). Mid-band bridge selection lives in
`scheduler.rs::sample_bridge_candidates` — cross-source only, entity
overlap in `[BRIDGE_MIN_OVERLAP, BRIDGE_MAX_OVERLAP]`, jaccard ≤ τ.

Skills: `skills/topic_curator.md`, `skills/bridge_finder.md`,
`skills/bridge_search_refine.md`, `skills/formula_harvester.md`. All
compiled in via `include_str!`.

## Don't

- Don't bypass `governor` with a direct LLM client.
- Don't use `std::fs` in async paths — use `tokio::fs`.
- Don't commit `.env`, `target/`, `logs/`, or `.data/` (SQLite lives
  there). `public/` *is* committed as a sample vault with the GIS
  textbook fixture — that's intentional.
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

These `dead_code` warnings are expected and intentional placeholders —
do not remove without re-checking:

- `KnowledgeGraphAgent::model` — exposed for future status reporting.
- `Config::api_base` — held for an upcoming custom-endpoint path.
- `Config::toc_page_budget` — reserved for the TocExtractor vision pass.
- `AppError::Api` / `AppError::Actor` — reserved for surfacing upstream
  status codes and actor-framework errors respectively.
- `DocumentProgress`'s `path` / `byte_size` / `discovered_at` fields —
  populated by sqlx for status page expansion.

If you touch those modules for a real reason, wire the fields in
rather than deleting them.
