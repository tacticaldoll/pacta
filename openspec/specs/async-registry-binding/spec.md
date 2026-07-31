# async-registry-binding Specification

## Purpose

An asynchronous binding of the frozen `Registry` contract, for durable backends that do async
I/O and cannot implement the synchronous trait. It is a *second binding* of the same five
operations, not new semantics: the lifecycle semantics are single-sourced in
`pacta_contract::lifecycle`, which both bindings compose over, so they cannot drift. It lives in
a separate `pacta-contract-async` crate so the frozen sync contract is undisturbed and sync-only
consumers never pull the async dependency.

## Requirements

### Requirement: An Async Binding Of The Frozen Registry Contract
Pacta SHALL provide an `AsyncRegistry` trait, in a separate `pacta-contract-async` crate, that is
a faithful asynchronous binding of the frozen five-op `Registry` contract — the same operations
(claim, heartbeat, fulfill, breach, release) with the same semantics, made asynchronous. It SHALL
add no operation the sync contract lacks and SHALL NOT change the sync `Registry`. Because the
claim authority behaves identically, this is a second binding of the frozen contract, not new
semantics.

#### Scenario: The async trait mirrors the five ops
- **WHEN** a caller uses `AsyncRegistry`
- **THEN** it exposes claim, heartbeat, fulfill, breach, and release with the same meaning as the sync `Registry`, and adds no other operation

#### Scenario: The sync contract is untouched
- **WHEN** the async binding is added
- **THEN** the synchronous `Registry` trait, `Outcome`, and the value types are unchanged

### Requirement: Backends Implement Primitives; Semantics Stay Single-Sourced
The `AsyncRegistry` trait SHALL require a backend to implement only a small set of primitives — a
native selection (`claim`) and a transition port (`load` the held state, then `cas` set-if-
unchanged) — and SHALL provide the transition operations (heartbeat, fulfill, breach, release) as
default methods that compose over the shared `pacta_contract::lifecycle` kernel. The lifecycle
semantics (eligibility, transitions, the lapsed check, the authority check) SHALL NOT be
re-implemented in the async binding; they come from the shared kernel, so the sync and async
bindings cannot drift. Only the native selection re-expresses the eligibility invariant per
backend.

#### Scenario: A transition composes over the shared kernel
- **WHEN** a default transition method runs
- **THEN** it loads the held state, computes the next state via the shared `lifecycle` kernel, and atomically applies it — not re-deciding the transition itself

#### Scenario: A backend implements only the primitives
- **WHEN** a backend binds to `AsyncRegistry`
- **THEN** it implements the selection and the load/cas transition port, and inherits the five ops through the default methods

### Requirement: A Lost Authority Resolves To Not-Current-Holder
A transition whose atomic set-if-unchanged fails because the pact's state changed SHALL be
retried against the reloaded state, and if the retainer no longer holds the pact (a lapse and
reclaim rotated authority away, or it was settled), the transition SHALL resolve to a
not-current-holder error. A backend's error type SHALL be able to represent that outcome (the
shared kernel's `NotCurrentHolder` converts into it).

#### Scenario: A reclaimed pact's stale transition is rejected
- **WHEN** a holder's lease lapsed and the pact was reclaimed, then the prior holder attempts a transition
- **THEN** the atomic set fails, the reload finds the retainer no longer holds the pact, and the transition resolves to a not-current-holder error
