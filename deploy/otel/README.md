# Local OpenTelemetry backend

Single-container Jaeger (all-in-one) with OTLP receivers on the standard
ports. Enough to see every span the agent emits without running a
separate OTel Collector.

## Start

```bash
docker compose -f deploy/otel/docker-compose.yml up -d
```

## Wire the agent

In `.env`:

```
OTEL_EXPORTER_OTLP_ENDPOINT=http://127.0.0.1:4317
OTEL_SERVICE_NAME=edge-kg-agent
OTEL_RESOURCE_ATTRIBUTES=deployment.environment=dev,host.name=pi
```

Then `cargo run --release`. On startup you should see:

```
INFO edge_kg_agent::telemetry: opentelemetry enabled endpoint=http://127.0.0.1:4317
```

## View traces

Open <http://127.0.0.1:16686>. Select service `edge-kg-agent`, click
**Find Traces**. First-class spans: `ingest`, `extract`, `write_note`,
plus whatever the decomposed agents add later.

## Stop

```bash
docker compose -f deploy/otel/docker-compose.yml down
```

Jaeger all-in-one keeps traces in memory only; restarting clears them.
Fine for dev — swap to Tempo + persistent storage when that matters.

## Ports

| Port  | Purpose                               |
| ----- | ------------------------------------- |
| 16686 | Jaeger UI                             |
| 4317  | OTLP/gRPC receiver (agent → Jaeger)   |
| 4318  | OTLP/HTTP receiver (alternative)      |
