# Information Lab

Information Lab is an edge-native autonomous pipeline that converts PDFs into an Obsidian-ready knowledge graph and runs continuous multi-agent research over the generated notes.

## What it does

- Watches a folder for new PDFs.
- Ingests and chunks content into SQLite.
- Runs extraction + research agents across light/heavy model tiers.
- Writes structured Obsidian notes with source and topic indexing.
- Produces syntheses, bridges, theorem notes, derivation chains, and daily reports.

## Quick Start

```bash
cargo run
```

Then drop PDFs into your configured `WATCH_DIR` and inspect output in `VAULT_DIR`.

## High-level Architecture

```mermaid
flowchart LR
    PDF[Watched PDFs] --> INGEST[Ingest + Chunk]
    INGEST --> DB[(SQLite)]
    DB --> ORCH[Orchestrator + Scheduler]
    ORCH --> AGENTS[Agent System]
    AGENTS --> VAULT[Vault Writer]
    VAULT --> OBS[Obsidian Notes + Indexes]
```

For detailed architecture and workflow diagrams, see the docs below.

## Documentation

- [Docs Home](docs/README.md)
- [User Guide](docs/user-guide/README.md)
- [Developer Guide](docs/developer-guide/README.md)
- [Architecture](docs/architecture/README.md)
- [Research Loop](docs/research-loop/README.md)

## Key Concepts

- **Dual model tiers**: light and heavy roles have separate limiter budgeting.
- **Research loop**: scheduled parallel tasks plus iterative bridge refinement.
- **Knowledge vault**: two-axis index by source and topic.

## Repository Layout

- `src/` core runtime and agents
- `skills/` prompt and behavior specs
- `migrations/` SQLite schema evolution
- `systemd/` service unit
- `docs/` user and developer documentation

## Contributing

If you add a new agent or runtime behavior, update the relevant developer docs:

- `docs/developer-guide/README.md`
- `docs/developer-guide/documentation-standards.md`
- `docs/architecture/README.md` (if control flow changes)

