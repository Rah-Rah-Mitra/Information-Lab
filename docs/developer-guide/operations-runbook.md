# Operations Runbook

## Service model

- Primary runtime is the Rust process (`cargo run` for local execution).
- Production-style host integration can use `systemd/edge-kg-agent.service`.

## Startup checklist

1. Validate environment variables.
2. Confirm DB file path is writable.
3. Confirm watch/vault directories exist.
4. Start service and verify initial scheduler ticks in logs.

## Health signals

- New PDFs are detected and ingested.
- `pending` work drains over time.
- Output appears under `Generated/` and index files update.
- Telemetry spans/events are emitted when tracing is configured.
- Research API accepts queue requests and exposes request timelines.

## Incident patterns

### Stalled queue

- Check API limits and limiter pressure.
- Verify DB is not locked by external process.
- Inspect retrier behavior for repeated terminal errors.

### Missing research outputs

- Confirm scheduler interval settings.
- Confirm task creation preconditions are met.
- Inspect confidence thresholds that gate output emission.
- For ad-hoc requests, inspect `/research/{id}/events` for `solvability_checked` and
  terminal `finalized`/`failed` phases.

### Corrupt or partial vault output

- Validate vault path permissions.
- Check for abrupt restarts during write windows.
- Re-run on affected documents if needed.

## Change management

For migrations or workflow changes:

- Apply SQL migrations before deploying runtime expecting new columns/tables.
- Roll forward with small batches and monitor logs/telemetry.
- Keep docs synchronized in the same change set.

## CI/CD checkpoints

- Pull requests and `main` pushes must pass build + full-target tests in
  `.github/workflows/ci-cd.yml`.
- Release tags (`v*`) trigger binary packaging and release attachment; verify the
  `edge-kg-agent` artifact is present before announcing a release.
