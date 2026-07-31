# Pacta Domain Language

Pacta uses contract and arbitration terminology for public APIs, specs, and
user-facing documentation. These terms replace generic queue vocabulary where
the concept is part of Pacta's public model.

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

- `Registry` - the pure lifecycle state machine that preserves pacts and dockets.
- `Executor` - the public role that executes claimed pacts through middleware.
- `Execution` - a single attempt to handle a claimed pact.
- `Outcome` - the executor's result for an execution.
- `Settlement` - the lifecycle conclusion applied to a claim, currently fulfill
  or breach.
- `Middleware` - Pacta-native execution composition around an executor.
- `Policy` - Pacta-native orchestration rule such as retry, timeout, or rate
  limiting.
- `Driver` - a mechanical runtime-loop term for polling a registry and driving
  execution. Use it when discussing implementation mechanics, not as the main
  public role.

## Engineering Boundary

Public APIs, OpenSpec requirements, examples, and user-facing documentation use
the Pacta domain language. Private implementation may use mechanical terms such
as driver, heartbeat, retry, timeout, scheduler, or middleware when those terms
make behavior clearer.

Tower is an adapter target, not Pacta's core public identity. Tower vocabulary
belongs in adapter-owned crates, not in the first-layer runtime API.

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
