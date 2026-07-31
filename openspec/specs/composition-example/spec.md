# Composition Example Specification

## Purpose

Keep Pacta's public composition surface demonstrable end to end by a downstream
consumer, and keep that demonstration constrained so it never leaks orchestration
behavior, Registry-side policy, private API access, or new core dependencies.

## Requirements

### Requirement: Public-API composition example
Pacta's composition surface SHALL be demonstrable end to end by a downstream
consumer using only the public API of `pacta-contract`, `pacta-executor`, and
`pacta-driver`, and the workspace SHALL carry that demonstration as a standing
example — `Registry` claim, `Executor` execution through a `Middleware` decorator,
and `Registry` settlement. The example SHALL be an `examples/` build target on
`pacta-driver` and SHALL NOT be a new workspace member crate. This requirement
exists to keep the composability guarantee and its constraints permanent, not
merely to assert that a file exists.

#### Scenario: Example drives a fulfilled lifecycle
- **WHEN** the example runs with a registry holding one claimable pact and an executor that reports `Outcome::Fulfilled`
- **THEN** the driver performs one step that claims the pact, executes it through the middleware, and settles the claim as `Step::Fulfilled`

#### Scenario: Example composes only through public items
- **WHEN** the example is compiled
- **THEN** it references only publicly exported items of the core crates and reaches into no crate-private module or field

#### Scenario: Example is built by the standard gates
- **WHEN** `cargo build --workspace`, `cargo clippy --all-targets -- -D warnings`, and `cargo fmt --all --check` run
- **THEN** the example target is compiled, linted, and format-checked like any other target

#### Scenario: Example uses Pacta domain language
- **WHEN** the example names its consumer types and bindings
- **THEN** they follow the contract/arbitration domain language of `docs/domain-language.md`, because the Engineering Boundary lists examples as a domain-language surface, and they do NOT copy the mechanical `Dummy`/`Test`/`Identity` naming used by private `#[cfg(test)]` helpers

### Requirement: Example carries no orchestration behavior
The composition example SHALL demonstrate the middleware seam without implementing
orchestration behavior. Its middleware SHALL be a pass-through decorator that
forwards execution to the wrapped executor unchanged, occupying the same decorator
slot a future retry or timeout middleware would fill.

#### Scenario: Pass-through middleware preserves the executor outcome
- **WHEN** the example's middleware wraps an executor and the driver executes a claimed pact
- **THEN** the outcome observed by the driver is exactly the outcome the wrapped executor produced, with no retry, backoff, timeout, or rate-limit logic applied

#### Scenario: No deferred behavior is introduced
- **WHEN** the example source is reviewed
- **THEN** it contains no retry loop, backoff calculation, timeout, rate limiter, or other orchestration logic that BACKLOG.md defers

### Requirement: Example preserves Registry purity
The composition example SHALL keep the `Registry` implementation a pure lifecycle
state machine. The example's registry SHALL implement only `claim`, `heartbeat`,
`fulfill`, and `breach`, and SHALL NOT compute backoff, manage visibility delay,
inspect clause contents, or evaluate any policy.

#### Scenario: Registry implementation stays lifecycle-only
- **WHEN** the example's registry handles the lifecycle operations
- **THEN** it neither reads `Pact::clause` for business decisions nor derives scheduling, delay, or retry state, preserving the pure state machine axiom

### Requirement: Example adds no core dependency
The composition example SHALL NOT introduce any new normal dependency on a core
crate. Any dependency required only by the example SHALL live in `pacta-driver`
dev-dependencies.

#### Scenario: Tianheng boundaries remain satisfied
- **WHEN** `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` runs after the example is added
- **THEN** the Tianheng constitution reports no violation, because the example adds no normal dependency to any core crate
