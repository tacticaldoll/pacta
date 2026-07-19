## MODIFIED Requirements

### Requirement: Core Reads No Ambient Time
The `pacta-contract` core SHALL NOT read an ambient wall clock. An executable
governance reaction SHALL reject ambient current-time reads in the core —
including reads reached through a renamed or re-exported import, and a
fully-qualified external time constructor written without an import — so the
injected-time discipline is enforced rather than merely documented.

#### Scenario: An ambient clock read in the core fails governance
- **WHEN** `pacta-contract` source acquires the current time from an ambient clock
  such as a `std::time` `now()` call or a `uuid` time-based constructor
- **THEN** the governance reaction fails

#### Scenario: An aliased ambient clock read is still caught
- **WHEN** the core reaches an ambient clock read through a renamed or re-exported
  import, such as `use std::time::SystemTime as Clock; Clock::now()`
- **THEN** the governance reaction still fails, because the reaction resolves the
  call's symbol path rather than matching source text

#### Scenario: A fully-qualified time-based UUID without an import is still caught
- **WHEN** the core mints a time-based identifier through a fully-qualified path
  written without importing the crate, such as `uuid::Uuid::now_v7()` with no
  `use uuid`
- **THEN** the governance reaction still fails, because the ambient-time reaction
  resolves a bare external-crate head to its declared dependency

#### Scenario: Runtime clock reads outside the core are allowed
- **WHEN** a runtime crate such as `pacta-driver` reads the current time to inject
  it into registry operations
- **THEN** the governance reaction does not reject it, because the prohibition
  scopes to the core contract

## ADDED Requirements

### Requirement: Semantic Reactions Are Executably Proven
Pacta SHALL prove by executable tests that its semantic governance reactions —
the kernel async-exposure boundary and the facade kernel-exclusion boundary —
react, so a misconfigured or silently no-op boundary cannot pass forever. A
reaction test SHALL assert that a leaking fixture is rejected and that a matching
non-leaking fixture stays clean, so the proof is not a vacuous always-fires.

#### Scenario: The kernel async-exposure reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts that the semantic check reports a violation for a fixture
  whose kernel exposes a public `async fn`

#### Scenario: The facade kernel-exclusion reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts that the semantic check reports a violation for a fixture
  facade that re-exports an item of the `pacta-contract` `kernel` module

#### Scenario: The reaction proof is precise
- **WHEN** the governance test suite runs a semantic reaction against a fixture
  that does not commit the leak the reaction targets
- **THEN** the semantic check reports no violation, so the proof distinguishes a
  reacting boundary from one that always fires
