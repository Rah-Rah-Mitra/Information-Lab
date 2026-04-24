# Documentation Home

This documentation is split by audience and operational goal.

| Document | Audience | Purpose | Typical Use |
|---|---|---|---|
| [User Guide](user-guide/README.md) | Operators / end users | Install, configure, run, monitor, and use API features | Day-0 setup + daily operations |
| [Developer Guide](developer-guide/README.md) | Contributors | Code map, extension workflow, CI expectations | Implementing or changing features |
| [Architecture](architecture/README.md) | Developers / architects | Runtime topology, control boundaries, data flow | Design reviews and onboarding |
| [Research Loop](research-loop/README.md) | Operators + developers | Scheduler/research lifecycle, state machine, request loop | Debugging research behavior |

## Feature-to-doc map

- **Ingest/extraction pipeline** → User Guide sections 4–6 + Architecture.
- **Research API (`/research/request`, `/research/{id}`)** → User Guide section 7 + Research Loop section 4.
- **Monitoring (`SYSTEM_STATUS.md`, events, OTEL)** → User Guide section 7 + Operations Runbook.
- **Agent development/change workflow** → Developer Guide + Documentation Standards.

## Recommended reading paths

- **New operator**: User Guide → Research Loop (sections 4, 7, 8).
- **New contributor**: Developer Guide → Architecture → Research Loop.
- **On-call debugging**: User Guide (monitoring section) → Operations Runbook.
