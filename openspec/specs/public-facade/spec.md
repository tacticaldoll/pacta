# public-facade Specification

## Purpose
Define the curated `pacta` facade as the workspace's single compose-level public entrypoint: a pure re-export crate that excludes the sans-I/O kernel and proves end-to-end lifecycle composition with a crate-root doctest, all enforced by executable governance.
## Requirements
### Requirement: Curated Public Entrypoint
Pacta SHALL provide a single facade crate `pacta` that is the curated public
entrypoint to the compose-level API. The facade SHALL re-export the public items a
downstream consumer needs to compose the lifecycle end to end — implement
`Registry`, implement `Executor` and `Middleware`, and run the `Driver` — drawing
them from `pacta-contract`, `pacta-executor`, and `pacta-driver`. The facade SHALL
depend only on those three crates and SHALL NOT depend on any backend crate.

#### Scenario: Facade re-exports the compose-level surface
- **WHEN** a downstream consumer depends only on `pacta`
- **THEN** it can name `Pact`, `Claim`, `Retainer`, `Timestamp`, `Outcome`, `Settlement`, and `Registry`; `Executor`, `Execution`, and `Middleware`; and `Driver`, `Step`, and `DriverError`, without depending on the individual core crates directly

#### Scenario: Facade depends on no backend
- **WHEN** `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** the Tianheng constitution reports no violation, because `pacta` depends only on `pacta-contract`, `pacta-executor`, and `pacta-driver`

### Requirement: Facade Excludes The Kernel
The facade's curated surface SHALL exclude the sans-I/O lifecycle kernel. The
kernel (`Directive`, `Notice`, `Kernel`, `StepResult`, and the `kernel` module)
SHALL remain reachable only through `pacta-contract` directly, so the facade names
the compose-level API and not the advanced state-machine machinery. This exclusion
SHALL be enforced by an executable reaction, not by omission alone.

#### Scenario: Kernel is absent from the facade surface
- **WHEN** a downstream consumer depends only on `pacta`
- **THEN** it cannot reach the kernel types through `pacta`, and must depend on `pacta-contract` directly to use them

#### Scenario: A facade kernel re-export fails governance
- **WHEN** the facade's public API re-exports any item of the `pacta-contract` `kernel` module
- **THEN** the governance reaction fails via the hunyi semantic dimension

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
decorator carrying no orchestration behavior, and its registry SHALL be a pure
lifecycle state machine implementing only the lifecycle operations.

#### Scenario: Facade composition doctest drives a fulfilled lifecycle
- **WHEN** the facade doctest runs with a registry holding one claimable pact and an executor that reports `Outcome::Fulfilled`
- **THEN** the driver performs one step that claims the pact, executes it through the middleware, and settles the claim as `Step::Fulfilled`

#### Scenario: Facade composition doctest imports only from the facade
- **WHEN** the facade doctest is compiled
- **THEN** it references only items re-exported by `pacta`, and does not import from `pacta-contract`, `pacta-executor`, or `pacta-driver` directly

#### Scenario: Facade composition middleware carries no orchestration
- **WHEN** the facade doctest's middleware wraps an executor and the driver executes a claimed pact
- **THEN** the outcome the driver observes is exactly the wrapped executor's outcome, with no retry, backoff, timeout, or rate-limit logic applied

#### Scenario: Facade composition registry stays lifecycle-only
- **WHEN** the facade doctest's registry handles the lifecycle operations
- **THEN** it implements only `claim`, `heartbeat`, `fulfill`, and `breach`, and neither inspects clause contents nor derives delay, backoff, or policy

