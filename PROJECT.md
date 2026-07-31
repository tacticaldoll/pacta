# Project Contract

## Purpose

Pacta is a Pacta-native, middleware-oriented contract runtime for Rust. It is
designed to act as an ultra-minimalist durable pact system that delegates all
execution orchestration (retries, timeouts, rate limiting, routing) to
Pacta-native middleware and policy composition.

Tower may be supported through adapter-owned crates, but Tower vocabulary must
not define Pacta's core public API.

By stripping business logic from the registry layer, Pacta enables backend
substitutability, massive concurrency through executor implementations, and
out-of-the-box event replay capabilities.

## Core Contract

The behavior that must be protected at all costs:
- **Registry Purity**: The `Registry` is a pure lifecycle state machine
  (`claim`, `fulfill`, `breach`, `heartbeat`). It never computes exponential
  backoff, never manages visibility delays, and never inspects clauses.
- **Executor Isolation**: Execution is isolated into Pacta-native middleware
  stacks.

## Terminology

- **Signal**: A short-lived external trigger (e.g., an HTTP request or a Cron tick).
- **Pact**: A durable command or contract, generated from a Signal, ready to be executed.
- **Docket**: The public grouping from which Pacts are selected.
- **Clause**: The business data carried by a Pact.
- **Brief**: Non-business operational context attached to a Pact.
- **Registry**: The pure lifecycle state machine tracking Pacts and Dockets.
- **Executor**: The public role responsible for executing a claimed Pact through Pacta-native middleware.
- **Middleware**: Pacta-native execution composition around an executor, acting as a decorator.
- **Policy**: A minimal value naming an orchestration intent (like retry or timeout) evaluated by middleware.
- **Claim**: A Pact plus the authority needed to process it.
- **Retainer**: The opaque token proving authority to fulfill or breach a Claim.
- **Tribunal**: Terminal review for exhausted Pacts that should no longer be handled automatically.

See `docs/domain-language.md` for the canonical glossary and legacy mapping.

## Change Prioritization

When comparing possible changes, prefer the one that protects the core contract earliest. Future phases are defined in `BACKLOG.md`, but the prioritization rule is always:

1. Correctness, data integrity, and strict adherence to the Three Axioms.
2. Specified feature completeness for Pacta-native middleware and policies.
3. Operator observability (tracing, tribunal review).
4. Scale-out and new `Registry` backend integrations.
