# Pacta Product And Architecture Blueprint

Pacta is a thin, elegant durable contract fabric and governed pattern framework
for Rust user-defined obligations.

The blueprint exists to set boundaries. It is not a phase roadmap.

## Core Pattern

```text
Signal -> Pact -> Claim -> Execution -> Settlement
```

- `Signal` is short-lived external intent.
- `Pact` is the durable unit of obligation.
- `Claim` grants temporary authority to execute.
- `Execution` is one attempt to handle the obligation.
- `Settlement` records the lifecycle conclusion.

The kernel owns only the durable contract lifecycle. User meaning, execution
policy, persistence choices, and framework integration attach around it.

## Extension Surfaces

### User-Defined Obligation

Users decide what an obligation means, how clauses are interpreted, and what
successful fulfillment means for their domain.

Candidate patterns: typed clauses, validation, obligation families, operator
review semantics.

### Execution Composition

Execution behavior composes around `Executor` without expanding the lifecycle
kernel.

Candidate patterns: retry, timeout, rate limit, tracing, concurrency limits,
settlement policy.

### Lifecycle Persistence

Registries preserve durable pact lifecycle authority without owning business
policy.

Candidate patterns: conformance suites, in-memory registries, durable registry
backends, lifecycle recovery.

### Integration Boundary

Adapters connect Pacta to frameworks, transports, runtimes, and observability
systems without defining core APIs.

Candidate patterns: ingress adapters, framework adapters, telemetry exporters,
operator tools.

## Growth Rule

Future growth enters Pacta as a governed design pattern:

1. Name the extension surface.
2. Show that the pattern does not expand the lifecycle kernel.
3. Keep adapter or benchmark vocabulary outside the first-layer public API.
4. Add executable governance when prose claims a boundary.

## Benchmark Stance

Worklane is origin context and a warning against returning to heavy queue
behavior. Apalis, Tower, and lightweight background-job systems are calibration
points. They help Pacta see the ecosystem gap; they do not define Pacta's core.

## Non-Commitment Rule

Examples in this blueprint are possible directions, not required phases. A
candidate becomes real only through an OpenSpec change, adversarial review,
implementation, verification, sync, and archive.
