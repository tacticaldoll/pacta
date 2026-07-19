# async-registry-binding Specification

## Purpose

An asynchronous binding of the frozen `Registry` caller contract, for durable backends that do
async I/O and cannot implement the synchronous trait. It is a *second binding* of the same five
operations, not new semantics: the lifecycle semantics are single-sourced in
`pacta_contract::lifecycle`, and both bindings share one implementer-facing transition port
(`apply`), so they cannot drift. It lives behind `pacta-contract`'s `async` feature (a feature-gated
module, not a separate crate) so sync-only consumers that do not enable it compile no async.

## Requirements

### Requirement: An Async Binding Of The Frozen Registry Contract
Pacta SHALL provide an `AsyncRegistry` trait, behind `pacta-contract`'s `async` feature (a
feature-gated module, not a separate crate), that is a faithful asynchronous binding of the frozen
five-op `Registry` caller contract — the same operations (claim, heartbeat, fulfill, breach, release)
with the same semantics, made asynchronous. It SHALL add no operation the sync contract lacks. The
caller-facing five-op contract and its semantics SHALL be identical across the sync and async
bindings, and the two bindings SHALL share one implementer-facing transition port so they cannot
drift. Because the claim authority behaves identically, this is a second binding of the frozen
contract, not new semantics. The async binding SHALL be isolated by the feature gate: a consumer that
does not enable `async` SHALL compile no async binding code, so a sync-only consumer pulls no async
surface.

#### Scenario: The async trait mirrors the five ops
- **WHEN** a caller uses `AsyncRegistry`
- **THEN** it exposes claim, heartbeat, fulfill, breach, and release with the same meaning as the sync `Registry`, and adds no other operation

#### Scenario: The caller-facing five ops are unchanged
- **WHEN** the bindings are unified on the shared transition port
- **THEN** the caller-facing five ops (claim, heartbeat, fulfill, breach, release), `Outcome`, and the value types keep their meaning; only the implementer-facing required-method set changes, and it changes identically for both bindings

#### Scenario: A sync-only consumer compiles no async
- **WHEN** a consumer depends on `pacta-contract` without enabling the `async` feature
- **THEN** the `AsyncRegistry` binding is absent from the build, so the sync-only consumer compiles no async binding code and pulls no async surface

### Requirement: Backends Implement Primitives; Semantics Stay Single-Sourced
The `AsyncRegistry` trait SHALL require a backend to implement only a native selection (`claim`), a
single transition port (`apply`), and a lease accessor, and SHALL provide the transition operations
(heartbeat, fulfill, breach, release) as default methods that call `apply` with the corresponding
`pacta_contract::lifecycle` decision. `apply` SHALL apply a pure kernel transition — a
`Fn(&State) -> Result<State, NotCurrentHolder>` — within the backend's own atomic scope. The
lifecycle semantics (eligibility, transitions, the lapsed check, the authority check) SHALL NOT be
re-implemented in the async binding; they come from the shared kernel decision passed to `apply`, so
the sync and async bindings cannot drift. Only the native selection re-expresses the eligibility
invariant per backend.

#### Scenario: A transition composes over the shared kernel
- **WHEN** a default transition method runs
- **THEN** it applies the shared `lifecycle` decision within the backend's atomic scope, not re-deciding the transition itself

#### Scenario: A backend implements only the primitives
- **WHEN** a backend binds to `AsyncRegistry`
- **THEN** it implements the selection and the `apply` transition port, and inherits the five ops through the default methods

### Requirement: A Lost Authority Resolves To Not-Current-Holder
A transition SHALL resolve to a not-current-holder error when it is applied against a pact the
retainer no longer holds — a lapse and reclaim rotated authority away, or it was settled. Where a
backend's `apply` uses optimistic concurrency, a set-if-unchanged failure SHALL be retried against
the reloaded state; where it uses a lock or transaction, the atomic scope enforces the outcome
directly.
A backend's error type SHALL be able to represent that outcome (the shared kernel's
`NotCurrentHolder` converts into it).

#### Scenario: A reclaimed pact's stale transition is rejected
- **WHEN** a holder's lease lapsed and the pact was reclaimed, then the prior holder attempts a transition
- **THEN** `apply` finds the retainer no longer holds the pact and the transition resolves to a not-current-holder error

### Requirement: The Transition Bound Does Not Color The Apply Future
The `Transition` port type SHALL be `Send + Sync`, and the documentation SHALL state precisely what
that bound does and does not mean. It means the transition **closure** — the pure kernel decision —
can be shared and moved across thread boundaries, which a backend needs to hold it across its own
atomic scope or hand it to a worker. It SHALL NOT be documented as implying that the async binding's
`apply` **future** is `Send`: future coloring stays the consumer's, consistent with the binding's
`Send`-agnostic futures. A backend that requires a `Send` `apply` future satisfies that at its own
concrete call site, not because the `Transition` bound provides it.

#### Scenario: The Transition bound is documented as closure-sharing
- **WHEN** a consumer reads the `Transition` documentation
- **THEN** it states that `Send + Sync` lets the transition closure be shared across thread boundaries, so a backend can hold or hand off the decision

#### Scenario: The Transition bound is not documented as coloring the future
- **WHEN** a consumer reads the `Transition` and async `apply` documentation
- **THEN** it states that the `Transition` bound does not make the async `apply` future `Send`, keeping future coloring the consumer's to compose, consistent with the binding's `Send`-agnostic futures

### Requirement: apply_via_cas Is Unbounded Retry With No Policy
The optional `apply_via_cas` helper SHALL be documented as an unbounded `load → decide →
set-if-unchanged` retry loop: under sustained contention it retries indefinitely and provides no
fairness, timeout, or cancellation guarantee. Termination under pathological contention SHALL be
documented as caller/runtime policy, not a property of the helper. Documenting this SHALL NOT add a
retry policy, a backoff, a timeout, or a runtime to the helper — it remains the minimal
compare-and-set strategy, and any bound on its retrying is composed by the caller outside it.

#### Scenario: The helper documents unbounded retry
- **WHEN** a consumer reads the `apply_via_cas` documentation
- **THEN** it states that under sustained contention the helper retries without bound and offers no fairness, timeout, or cancellation guarantee

#### Scenario: Termination is caller policy
- **WHEN** the documentation describes bounding the helper's retrying under pathological contention
- **THEN** it attributes termination and cancellation to caller or runtime policy composed around the helper, not to the helper itself

#### Scenario: Documenting the contract adds no orchestration
- **WHEN** the `apply_via_cas` contract is clarified
- **THEN** the helper gains no retry policy, backoff, timeout, cancellation, or runtime, remaining the minimal set-if-unchanged strategy
