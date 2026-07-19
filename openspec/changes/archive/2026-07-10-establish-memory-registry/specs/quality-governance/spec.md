## ADDED Requirements

### Requirement: Core Reads No Ambient Time
The `pacta-contract` core SHALL NOT read an ambient wall clock. An executable
governance check SHALL reject ambient current-time reads in the core, so the
injected-time discipline is enforced rather than merely documented.

#### Scenario: An ambient clock read in the core fails governance
- **WHEN** `pacta-contract` source acquires the current time from an ambient
  clock such as a `now()` call
- **THEN** the governance check fails

#### Scenario: Runtime clock reads outside the core are allowed
- **WHEN** a runtime crate such as `pacta-driver` reads the current time to inject
  it into registry operations
- **THEN** the governance check does not reject it, because the prohibition scopes
  to the core contract
