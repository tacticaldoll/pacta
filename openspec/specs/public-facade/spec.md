# public-facade Specification

## Purpose
Define the curated `pacta` facade as the workspace's single compose-level public entrypoint: a pure re-export crate that excludes the sans-I/O kernel and proves end-to-end lifecycle composition with a crate-root doctest, all enforced by executable governance.
## Requirements
### Requirement: Curated Public Entrypoint
Pacta SHALL provide a single facade crate `pacta` that is the curated public
entrypoint to the compose-level API. The facade SHALL re-export the public items a
downstream consumer needs to compose the lifecycle end to end — implement
`Registry`, implement `Executor` and `Middleware`, compose middleware through the
empty-stack value, the reified stack value, and the blind assembler, and run the
`Driver` — drawing them from `pacta-contract`, `pacta-executor`, and `pacta-driver`.
Because a legal `Registry` backend is written against the transition port, the facade
SHALL also re-export the backend-author lifecycle surface — the colorless
`pacta_contract::lifecycle` module (its `State`, the `on_X` transition decisions, the
`is_claimable` predicate, the lease arithmetic), the `Transition` port type, and the
`Uuid` identifier type the public constructors require (`Pact::new`, `Retainer::new`) —
so a legal backend, which mints Uuid-based fencing tokens, is implementable from `pacta`
alone. The facade SHALL depend only on those three crates and SHALL NOT depend on any
backend crate.

#### Scenario: Facade re-exports the compose-level surface
- **WHEN** a downstream consumer depends only on `pacta`
- **THEN** it can name `Pact`, `Claim`, `Retainer`, `Timestamp`, `Outcome`, `Settlement`, and `Registry`; `Executor`, `Execution`, `Middleware`, `Identity`, `Stack`, and `Composition`; and `Driver`, `Step`, and `DriverError`, without depending on the individual core crates directly

#### Scenario: Facade re-exports the backend-author lifecycle path
- **WHEN** a downstream consumer implements a `Registry` backend using only `pacta`
- **THEN** it can name the `Transition` port, the `pacta_contract::lifecycle` surface (`State`, the `on_X` transition decisions, `is_claimable`, and the lease arithmetic), and `Uuid` through `pacta`, so it can store lifecycle state, select an eligible pact, mint a fresh retainer, and apply a transition without depending on `pacta-contract` directly

#### Scenario: Facade depends on no backend
- **WHEN** `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** the Tianheng constitution reports no violation, because `pacta` depends only on `pacta-contract`, `pacta-executor`, and `pacta-driver`

### Requirement: Facade Excludes The Kernel
The facade's curated surface SHALL exclude the sans-I/O lifecycle **step-driver** kernel. The
step-driver kernel (`Directive`, `Notice`, `Kernel`, `StepResult`, and the `kernel` module)
SHALL remain reachable only through `pacta-contract` directly, so the facade names
the compose-level and backend-author API and not the advanced state-machine machinery. This
exclusion SHALL be enforced by an executable reaction, not by omission alone. The exclusion SHALL
scope to the `kernel` module only: the colorless `lifecycle` module is the backend-author surface
and is re-exported (see *Curated Public Entrypoint*), distinct from the excluded step-driver kernel.

#### Scenario: Kernel is absent from the facade surface
- **WHEN** a downstream consumer depends only on `pacta`
- **THEN** it cannot reach the step-driver kernel types through `pacta`, and must depend on `pacta-contract` directly to use them

#### Scenario: A facade kernel re-export fails governance
- **WHEN** the facade's public API re-exports any item of the `pacta-contract` `kernel` module
- **THEN** the governance reaction fails via the hunyi semantic dimension

#### Scenario: Re-exporting the lifecycle module does not trip the kernel exclusion
- **WHEN** the facade re-exports the `pacta_contract::lifecycle` module
- **THEN** the governance reaction reports no violation, because the exclusion targets the `kernel` module and `lifecycle` is a distinct colorless module that is the backend-author surface

### Requirement: Facade Carries No Logic
The facade SHALL be a pure re-export crate: its library SHALL contain only
re-exports, crate attributes, and documentation, and SHALL NOT define functions,
types, traits, or other behavior. This keeps the published entrypoint from
accreting batteries-included convenience over time. This constraint SHALL be
enforced by an executable reaction.

#### Scenario: A logic item in the facade fails governance
- **WHEN** the facade library defines an item other than a re-export (for example a function, struct, enum, or trait)
- **THEN** the governance reaction fails

#### Scenario: The facade library composes only through re-exports
- **WHEN** the facade library is reviewed
- **THEN** every public item it offers is a re-export of an item from `pacta-contract`, `pacta-executor`, or `pacta-driver`, and it holds no logic of its own

### Requirement: Facade Composition Example
The `pacta` facade SHALL carry a runnable documentation test in its crate root that
composes the lifecycle end to end through the facade — `Registry` claim, `Executor`
execution through a `Middleware` decorator, and `Registry` settlement — using only
the facade's public API. This composition proof SHALL be a doctest rather than a
separate `examples/` build target, so it runs and asserts under `cargo test` and is
rendered on the published documentation. Its middleware SHALL be a pass-through
decorator carrying no orchestration behavior, and its registry SHALL be a **complete
legal** `Registry`: it SHALL hold real lifecycle state, implement the `claim` selection,
the `lease_millis` accessor, and the atomic `apply` transition port (with heartbeat,
fulfill, breach, and release inherited as defaults), execute the passed transition
within a single atomic scope, and persist the resulting state. `claim` SHALL mint a
**distinct** retainer per claim (not a fixed value), so authority rotates on reclaim as
the contract requires. The doctest SHALL prove the lifecycle end to end on two axes:
after a pact is claimed, executed, and settled it is no longer claimable; and a pact
whose lease lapses and is reclaimed yields a different retainer than the lapsed claim.

#### Scenario: Facade composition doctest drives a fulfilled lifecycle
- **WHEN** the facade doctest runs with a registry holding one claimable pact and an executor that reports `Outcome::Fulfilled`
- **THEN** the driver performs one step that claims the pact, executes it through the middleware, and settles the claim as `Step::Fulfilled`

#### Scenario: Facade composition doctest imports only from the facade
- **WHEN** the facade doctest is compiled
- **THEN** it references only items re-exported by `pacta` (including the `lifecycle`, `Transition`, and `Uuid` backend-author surface), and does not import from `pacta-contract`, `pacta-executor`, `pacta-driver`, or `uuid` directly

#### Scenario: Facade composition middleware carries no orchestration
- **WHEN** the facade doctest's middleware wraps an executor and the driver executes a claimed pact
- **THEN** the outcome the driver observes is exactly the wrapped executor's outcome, with no retry, backoff, timeout, or rate-limit logic applied

#### Scenario: Facade composition registry is a legal stateful backend
- **WHEN** the facade doctest's registry handles the lifecycle
- **THEN** it holds real lifecycle state and implements `claim`, `lease_millis`, and an atomic `apply` that runs the passed transition and persists the next state — inheriting heartbeat, fulfill, breach, and release as defaults — rather than a no-op that applies no transition and holds no state

#### Scenario: The doctest proves a settled pact is not claimable
- **WHEN** the facade doctest claims, executes, and settles the pact and then claims again
- **THEN** the second claim returns nothing, proving the transition was actually applied and persisted rather than dropped

#### Scenario: The doctest proves authority rotates on reclaim
- **WHEN** the facade doctest claims a pact, lets its lease lapse, and reclaims it at a later injected time
- **THEN** the reclaimed claim carries a different retainer than the lapsed one, proving the backend mints a fresh retainer and is a complete legal `Registry`, not a settlement-only proof

