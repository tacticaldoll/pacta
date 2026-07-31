# Pacta Domain Language

Pacta uses contract and arbitration terminology for public APIs, specs, and
user-facing documentation. The vocabulary is governance, not decoration: it
keeps the product centered on durable user-defined obligations instead of
generic queue-runtime behavior.

## Core Entities

- `Signal` - a short-lived external trigger, such as an HTTP request or cron
  tick, that may be turned into a pact.
- `Pact` - the durable unit of obligation accepted by Pacta.
- `Docket` - the public grouping from which pacts are selected for execution.
- `Clause` - the business data carried by a pact.
- `Brief` - non-business operational context attached to a pact.

## Lifecycle

- `Claim` - acquire short-term processing authority for a pact.
- `Retainer` - the opaque token proving authority to settle a claim.
- `Fulfill` - report that a claimed pact completed successfully.
- `Breach` - report that a claimed pact failed.
- `Lapse` - recover a pact whose retainer expired without settlement.
- `Tribunal` - terminal review for exhausted pacts that should no longer be
  handled automatically.

## Architecture

- `Registry` - the durable lifecycle-authority port that preserves pacts and dockets
  and decides claim, lease, and settlement authority. It is the I/O-owning port, not
  the pure machine itself: the pure, colorless state machine is `lifecycle`, which
  every `Registry` backend composes over.
- `Executor` - the public role that executes claimed pacts through middleware.
- `Execution` - a single attempt to handle a claimed pact.
- `Outcome` - the executor's result for an execution.
- `Settlement` - the lifecycle conclusion applied to a claim, currently fulfill
  or breach.
- `Middleware` - Pacta-native execution composition around an executor. It acts as a decorator without framework concepts like readiness or generic requests.
- `lifecycle` - the pure, colorless, sans-I/O state machine that owns the lifecycle
  semantics (eligibility, transitions, the holder authority check, and lease
  arithmetic); every `Registry` backend composes over it so the semantics cannot drift.
- `Driver` - a mechanical runtime-loop term for polling a registry and driving
  execution. Use it when discussing implementation mechanics, not as the main
  public role.

## Product Boundary

Pacta's names express a thin, elegant contract/arbitration worldview:

- obligations are durable pacts, not generic jobs
- groupings are dockets, not broker topics
- processing authority is claimed and retained, not treated as an opaque queue
  pop
- failed or exhausted obligations go to review, not into a storage-owned policy
  sink

New vocabulary must preserve this restraint. It may use ordinary engineering
terms when they make mechanics clearer, but it must not let adapter, benchmark,
or queue terminology become the governing public shape.

## Engineering Boundary

Public APIs, OpenSpec requirements, examples, and user-facing documentation use
the Pacta domain language. Private implementation may use mechanical terms such
as driver, heartbeat, retry, timeout, scheduler, or middleware when those terms
make behavior clearer.

Framework integration is adapter scope, not Pacta's core public identity.
External framework vocabulary belongs in adapter-owned crates, not in the
first-layer runtime API.

## Legacy Mapping

| Previous public term | Pacta term |
|---|---|
| `Store` | `Registry` |
| lane / queue / topic | `Docket` |
| payload / body | `Clause` |
| metadata / headers | `Brief` |
| `reserve` | `claim` |
| `Reservation` | `Claim` |
| `ReservationReceipt` / lease token | `Retainer` |
| `ack` | `fulfill` |
| `nack` | `breach` |
| dead-letter / terminal failure area | `Tribunal` |
