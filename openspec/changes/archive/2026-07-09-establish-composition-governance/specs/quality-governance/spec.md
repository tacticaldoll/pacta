## MODIFIED Requirements

### Requirement: Tianheng Governance Reaction
Pacta SHALL run its Tianheng architecture constitution as a CI reaction.

#### Scenario: Architecture check runs
- **WHEN** a push or pull request runs CI
- **THEN** CI runs `pacta-governance` against the workspace manifest

#### Scenario: Contract crate remains isolated
- **WHEN** `pacta-contract` gains a forbidden dependency
- **THEN** the governance reaction fails

#### Scenario: Core framework leakage is rejected
- **WHEN** `pacta-contract`, `pacta-executor`, or `pacta-driver` gains an unapproved normal dependency on adapter, backend, or framework crates
- **THEN** the governance reaction fails

### Requirement: Runtime Crate Governance
Pacta SHALL keep new runtime crates covered by executable quality and architecture reactions.

#### Scenario: Runtime crates have architecture boundaries
- **WHEN** `pacta-executor` or `pacta-driver` is added to the workspace
- **THEN** `pacta-governance` declares explicit Tianheng boundaries for those crates

#### Scenario: Runtime crates obey quality gates
- **WHEN** runtime crates are added to the workspace
- **THEN** build, test, clippy, fmt, rustdoc, cargo-deny, and governance gates cover them from the workspace root

#### Scenario: Contract remains upstream of runtime
- **WHEN** runtime crates are added
- **THEN** `pacta-contract` remains isolated from all other workspace crates

#### Scenario: Runtime dependencies stay closed
- **WHEN** core runtime crates require normal dependencies
- **THEN** each allowed dependency is declared in the Tianheng constitution for that crate
