## MODIFIED Requirements

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
