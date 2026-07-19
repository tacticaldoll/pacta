## MODIFIED Requirements

### Requirement: Kernel Async-Exposure Reaction
Pacta SHALL run an executable semantic reaction that keeps the lifecycle kernel
free of exposed async throughout the kernel and its submodules, so the kernel's
runtime-agnosticism cannot silently drift — not at its own seam only.

#### Scenario: Kernel async fn is rejected
- **WHEN** the lifecycle kernel's public API exposes an `async fn`
- **THEN** the governance reaction fails via the hunyi semantic dimension

#### Scenario: A submodule async fn is rejected
- **WHEN** a submodule under the lifecycle kernel exposes an `async fn`
- **THEN** the governance reaction fails, because the async-exposure boundary descends the kernel subtree

#### Scenario: Async-exposure reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the async-exposure reaction runs as part of the governance check

### Requirement: Core Reads No Ambient Time
The `pacta-contract` core SHALL NOT read an ambient wall clock. An executable
governance reaction SHALL reject ambient current-time reads in the core — including
reads reached through a renamed or re-exported import — so the injected-time
discipline is enforced rather than merely documented.

#### Scenario: An ambient clock read in the core fails governance
- **WHEN** `pacta-contract` source acquires the current time from an ambient clock
  such as a `std::time` `now()` call or a `uuid` time-based constructor
- **THEN** the governance reaction fails

#### Scenario: An aliased ambient clock read is still caught
- **WHEN** the core reaches an ambient clock read through a renamed or re-exported
  import, such as `use std::time::SystemTime as Clock; Clock::now()`
- **THEN** the governance reaction still fails, because the reaction resolves the
  call's symbol path rather than matching source text

#### Scenario: Runtime clock reads outside the core are allowed
- **WHEN** a runtime crate such as `pacta-driver` reads the current time to inject
  it into registry operations
- **THEN** the governance reaction does not reject it, because the prohibition
  scopes to the core contract
