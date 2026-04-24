# Architecture

This document explains the runtime architecture at two levels: conceptual and detailed.

## 1) Conceptual overview

```mermaid
flowchart LR
    INPUT[PDF Inputs] --> INGEST[Ingest Pipeline]
    INGEST --> STORE[(SQLite State)]
    STORE --> CONTROL[Scheduler + Orchestrator]
    CONTROL --> AGENTS[Agent Fleet]
    AGENTS --> OUTPUT[Vault Writer]
    OUTPUT --> OBSIDIAN[Obsidian Knowledge Graph]
```

## 2) Runtime topology (detailed)

```mermaid
flowchart TB
    subgraph Host
      WATCH[Watcher]
      ING[ingest.rs]
      DB[(SQLite WAL)]
      ORCH[orchestrator.rs]
      SCHED[scheduler.rs]
      VAULT[vault.rs]
      AG1[Extractor]
      AG2[Curator]
      AG3[BridgeFinder]
      AG4[TheoremProver]
      AG5[DerivationChain]
      AG6[ReportWriter]
      AG7[FormulaExtractor]
      AG8[ErrorRetrier]
    end

    subgraph External
      L1[Light Model Tier]
      L2[Heavy Model Tier]
      SEARCH[Tavily Search]
      OTEL[OTLP/Jaeger]
    end

    WATCH --> ING --> DB
    SCHED --> ORCH
    ORCH --> AG1 --> L1
    ORCH --> AG2 --> L2
    ORCH --> AG3 --> L2
    AG3 --> SEARCH
    ORCH --> AG4 --> L2
    ORCH --> AG5 --> L2
    ORCH --> AG6 --> L2
    ORCH --> AG7 --> L1
    ORCH --> AG8 --> DB
    AG1 --> VAULT
    AG2 --> VAULT
    AG3 --> VAULT
    AG4 --> VAULT
    AG5 --> VAULT
    AG6 --> VAULT
    VAULT --> DB
    ORCH --> OTEL
```

## 3) Control boundaries

- **Ingest boundary**: file detection, hashing, chunking, and DB insertion.
- **Reasoning boundary**: orchestrator claims work and dispatches agents.
- **Persistence boundary**: DB task state and usage/event logging.
- **Presentation boundary**: vault writing and index regeneration.

## 4) Design principles

- Eventual progress over strict real-time output.
- Rate-limit aware scheduling per model tier.
- Guarded emission for confidence-sensitive outputs.
- Observable async control flow via tracing spans/events.
