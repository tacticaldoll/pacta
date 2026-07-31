# Backlog & Roadmap

This file records deferred work, future phases, and known architectural gaps. It is intentionally kept separate from the core `openspec/specs/` to ensure the specs represent only shipped, verified truth.

## Roadmap

The overarching journey of Pacta is defined by these phases.

### Phase 1: Foundation (✓ Shipped)
- Core architectural axioms defined (`AGENTS.md`).
- Pure, zero-dependency `Pact` and `Store` trait defined (`pacta-contract`).
- Executable governance via `tianheng` (`pacta-governance`).

### Phase 2: Execution Engine
- Implement `pacta-driver`: The runtime loop that consumes the `Store`.
- Define standard `Middleware` layers for orchestration (Retries, Timeouts, Rate Limiting).
- Wire up `Tower` compatibility.

### Phase 3: Conformance Suite
- Build `pacta-conformance`: A test suite to validate `Store` behavior across different backends.
- Establish the baseline tests that all backends (in-memory, SQLite, Postgres, Redis) must pass.

### Phase 4: Durable Backends
- Implement `pacta-sqlite` for single-node durability.
- Implement `pacta-postgres` for multi-node durability.
- Implement `pacta-redis` for high-throughput, ephemeral-ish workloads.

## Deferred Work (Backlog)

Features or concepts that are explicitly postponed until the core contract is robust:

- **Dashboard / UI**: Operator visibility into the lifecycle is important, but comes after the core engine is proven.
- **Complex Topologies**: Directed Acyclic Graphs (DAGs) and inter-job dependencies.
- **Strict Exactly-Once Delivery**: Pacta guarantees at-least-once. Exactly-once is an application-level concern.
