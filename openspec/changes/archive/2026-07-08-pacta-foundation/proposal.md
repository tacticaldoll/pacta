# Proposal: Pacta Foundation (A Tower-Native Task Runtime)

## Why

Pacta was born from a fundamental critique of traditional, broker-centric job queues (such as `worklane`, Celery, or Oban). Traditional queues suffer from "Semantic Bloat" — they pull execution policies (retries, backoff, routing, dead-letter storage, priority) into the storage layer and the core job envelope. This forces every storage backend (Postgres, Redis, etc.) to implement complex business logic, making the system heavy, hard to extend, and prone to breaking.

Pacta introduces a paradigm shift:
- **Ingress is composition.**
- **Store is lifecycle.**
- **Driver is runtime.**
- **Handler is Service.**

By completely delegating execution orchestration (retries, timeouts, tracing, rate limiting) to the `Middleware` ecosystem, the storage layer (`Store`) degrades into a pure, minimal state machine. The Store only tracks three lifecycle transitions: `reserve` (lease), `ack` (success), and `nack` (failure/dead).

## What Changes

This initial change establishes the **Axiomatic Design** of Pacta. It prevents the repository from accumulating the historical baggage of traditional job queues by cementing strict architectural boundaries from Day 1.

1. **The Zero-Dependency Contract**: We define `pacta-contract` as the sole source of truth for the system. It will have zero dependencies (save for `serde` and `uuid`) and will house the pure `Store` trait and `Pact` envelope.
2. **The "Store is Lifecycle" Axiom**: We explicitly strip features like `delay`, `attempts`, `max_attempts`, `priority`, and `trace_context` from the core job envelope. 
3. **Executable Governance**: We introduce `pacta-governance` using `tianheng` to physically prevent `pacta-contract` from importing business logic, and to ensure that Store implementations only depend on the contract.

## Philosophical Departures from Worklane

To ensure the knowledge of our exploration session is durably captured:

1. **No Built-in Retry/Timeout**: A Store must never compute exponential backoff. Retries are handled by a `RetryLayer` wrapping the Handler.
2. **No Built-in Priority/Cron**: Priority is a routing concern (handled by polling different "Lanes" at the Ingress/Driver layer). Cron is an external Ingress that injects `Signal`s into `Pact`s.
3. **Event Replay via Middleware**: By removing network-bound `&mut self` and `poll_ready` friction from the core task handler, Pacta natively supports massive parallel event replay. Replaying an event is simply pushing historical data through the exact same Middleware stack, potentially injecting a `SuppressSideEffectsLayer`.
4. **Natural Backpressure**: The `Driver` seamlessly translates backpressure by checking `poll_ready()` on the inner Handler stack before fetching new contracts from the Store.

## Scope

This proposal covers the foundation:
- Rewriting `AGENTS.md` and `PROJECT.md` to define the new axioms.
- Setting up the Cargo workspace.
- Creating the `pacta-contract` crate.
- Creating the `pacta-governance` crate to lock the boundaries.
- No actual `Store` implementations (e.g., Memory, SQLite) are built in this change. They will follow in subsequent proposals.
