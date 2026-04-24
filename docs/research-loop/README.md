# Research Loop

This document describes the lifecycle and control logic for continuous research generation.

## 1) Loop objective

Transform extracted knowledge into higher-order artifacts (syntheses, bridges, theorems, derivations, reports) on a recurring schedule.

## 2) Workflow primitives

The runtime uses three primitives:

- **Sequential**: deterministic stage chaining.
- **Parallel**: concurrent drains for independent work classes.
- **Loop**: bounded iterative refinement with explicit stop conditions.

## 3) Research tick sequence

```mermaid
sequenceDiagram
    participant S as Scheduler
    participant O as Orchestrator
    participant C as Curator
    participant B as BridgeFinder
    participant D as DB
    participant V as Vault

    S->>O: tick()
    par Curate lane
      O->>D: claim_curate_task
      D-->>O: task
      O->>C: run
      C->>V: write synthesis
      O->>D: mark done/error
    and Bridge lane
      O->>D: claim_bridge_task
      D-->>O: task
      O->>B: run iterative bridge loop
      B->>V: write bridge
      O->>D: mark done/error
    end
```

## 4) Bridge iterative loop (bounded)

```mermaid
flowchart LR
    P[Propose Link] --> R[Refine w/ Search]
    R --> C[Critique Confidence]
    C -->|confidence >= threshold| E[Emit Artifact]
    C -->|else and iter < max| P
    C -->|iter == max| X[Stop Without Emit]
```

## 5) State model

```mermaid
stateDiagram-v2
    [*] --> pending
    pending --> claimed
    claimed --> done
    claimed --> error
    error --> pending: retrier backoff
    error --> failed_final: max retries exceeded
    done --> [*]
    failed_final --> [*]
```

## 6) Guardrails and SLO-oriented controls

- Per-role limiter admission controls throughput.
- Separate light/heavy tiers prevent starvation.
- Confidence thresholds gate publication of speculative outputs.
- Retries are bounded to prevent infinite churn.

## 7) Debugging checklist

- Verify ticks are firing at expected intervals.
- Verify claim functions return eligible tasks.
- Verify limiter is admitting target roles.
- Inspect error-to-retry transitions for stuck records.
