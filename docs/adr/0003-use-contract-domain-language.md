# ADR 0003: Use Contract Domain Language

## Status

Accepted

## Context

Pacta already established its foundation as a Pacta-native,
middleware-oriented runtime with a pure lifecycle storage contract and
middleware-owned execution. The initial public API still used generic queue and
lease terms such as `Store`, `Reservation`, `reserve`, `ack`, `nack`, `lane`,
and `payload`.

Those names are mechanically clear, but they weaken Pacta's contract-oriented
model and keep the API close to the broker-centric vocabulary the project is
trying to leave behind.

## Decision

Use contract and arbitration terminology for public Pacta concepts.

The public contract crate, OpenSpec requirements, examples, and user-facing
documentation use the canonical glossary in `docs/domain-language.md`. The
central migration is:

- `Store` becomes `Registry`
- `lane` becomes `docket`
- `payload` becomes `clause`
- `Reservation` becomes `Claim`
- `ReservationReceipt` becomes `Retainer`
- `reserve`, `ack`, and `nack` become `claim`, `fulfill`, and `breach`
- public handler language becomes `Executor`
- terminal exhausted-pact handling is named `Tribunal`

Private implementation may use mechanical terms such as driver, heartbeat,
retry, timeout, scheduler, or middleware when those names describe mechanics
more clearly than the domain metaphor.

## Consequences

- Pacta's public API reads as a coherent contract lifecycle rather than a generic
  queue wrapper.
- The storage-purity axiom is unchanged: the registry still does not own retry,
  backoff, routing, priority, or clause inspection.
- This is a pre-release breaking rename. No compatibility aliases are retained.
- Future public terminology changes must go through OpenSpec.
