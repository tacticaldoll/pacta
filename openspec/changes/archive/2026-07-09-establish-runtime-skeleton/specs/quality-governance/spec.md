## ADDED Requirements

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
