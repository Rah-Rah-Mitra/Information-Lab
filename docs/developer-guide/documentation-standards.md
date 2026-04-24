# Documentation Standards

Use these standards for all new or updated docs.

## Required sections

Each technical doc should include:

1. Purpose
2. Scope
3. Inputs/Outputs
4. Control flow or lifecycle
5. Failure modes and guardrails
6. Operational/observability signals

## Diagram standards (Mermaid)

- Prefer multiple small diagrams over one dense diagram.
- Keep diagrams under ~10 nodes when possible.
- Use directional consistency (left-to-right or top-to-bottom).
- Avoid crossing arrows where possible.
- Label external dependencies clearly.

## Naming standards

- Use exact module/agent names from code.
- Avoid aliasing that differs from source paths.
- Use stable headings to support deep links.

## Update policy

When behavior changes, update docs in the same PR:

- `docs/architecture/README.md` for control-flow/topology changes.
- `docs/research-loop/README.md` for lifecycle/state changes.
- `docs/user-guide/README.md` for user-facing behavior changes.

## Quality checklist

- [ ] Audience is explicit (user/developer/operator).
- [ ] Examples are runnable or clearly marked pseudo.
- [ ] Error and retry behavior is documented.
- [ ] Links to related docs are present.
