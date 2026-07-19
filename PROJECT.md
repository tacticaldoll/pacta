# Project Contract

## Purpose

Pacta is a Tower-native, Middleware-driven task runtime for Rust. It is designed to act as an ultra-minimalist job queue that delegates all execution orchestration (retries, timeouts, rate limiting, routing) to the Middleware ecosystem. 

By stripping business logic from the storage layer, Pacta enables flawless backend substitutability, massive concurrency (via `&self` Handlers), and out-of-the-box event replay capabilities.

## Core Contract

The behavior that must be protected at all costs:
- **Storage Purity**: The `Store` is a pure state machine (`reserve`, `ack`, `nack`, `heartbeat`). It never computes exponential backoff, never manages visibility delays, and never inspects payloads.
- **Handler Isolation**: Execution is entirely isolated into Middleware stacks.

## Terminology

- **Signal**: A short-lived external trigger (e.g., an HTTP request or a Cron tick).
- **Pact**: A durable command or contract, generated from a Signal, ready to be executed.
- **Store**: The pure state machine tracking the lifecycle of Pacts.
- **Driver**: The runtime loop that fetches Pacts from the Store and feeds them into the Handler.
- **Handler**: The Middleware stack responsible for executing the Pact.
- **Reservation**: The lease held by the Driver over a Pact while it is being executed.

## Change Prioritization

When comparing possible changes, prefer the one that protects the core contract earliest. Future phases are defined in `BACKLOG.md`, but the prioritization rule is always:

1. Correctness, data integrity, and strict adherence to the Three Axioms.
2. Specified feature completeness for the Middleware ecosystem.
3. Operator observability (tracing, dead-letter storage).
4. Scale-out and new `Store` backend integrations.
