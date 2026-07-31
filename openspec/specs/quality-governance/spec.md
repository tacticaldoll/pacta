# Quality Governance Specification

## Purpose

Define Pacta's executable quality gates for CI, Rust style, supply-chain policy,
and Tianheng architectural reactions.

## Requirements

### Requirement: Definition Of Done CI
Pacta SHALL run its Definition of Done in GitHub Actions for push and pull request events.

#### Scenario: Rust checks run in CI
- **WHEN** a push or pull request runs CI
- **THEN** CI runs build, test, clippy with warnings denied, and rustfmt check from the workspace root

#### Scenario: Documentation warnings fail CI
- **WHEN** CI builds public documentation
- **THEN** rustdoc warnings fail the job

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

### Requirement: Kernel Async-Exposure Reaction
Pacta SHALL run an executable semantic reaction that keeps the lifecycle kernel
free of exposed async, so the kernel's runtime-agnosticism cannot silently drift.

#### Scenario: Kernel async fn is rejected
- **WHEN** the lifecycle kernel's public API exposes an `async fn`
- **THEN** the governance reaction fails via the hunyi semantic dimension

#### Scenario: Async-exposure reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the async-exposure reaction runs as part of the governance check

### Requirement: Active Prose Governance
Pacta SHALL include active prose drift in its executable governance reaction.

#### Scenario: Prose gate runs with architecture governance
- **WHEN** `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** it checks active project prose for high-risk stale architecture phrases before accepting the workspace

#### Scenario: Prose gate reports location
- **WHEN** active project prose violates a stale-phrase rule
- **THEN** the governance output identifies the relative file path, line number, phrase, and reason

#### Scenario: Historical prose is not governed
- **WHEN** archived OpenSpec changes or ADRs contain historical architecture vocabulary
- **THEN** active prose governance does not fail on that archived text

### Requirement: Supply Chain Policy
Pacta SHALL use cargo-deny for resolved dependency supply-chain policy.

#### Scenario: Supply-chain check runs
- **WHEN** a push or pull request runs CI
- **THEN** CI runs cargo-deny advisory, license, ban, and source checks

#### Scenario: Policy lives outside Tianheng
- **WHEN** dependency advisories, licenses, duplicate versions, or resolved sources are governed
- **THEN** the rule lives in `deny.toml` rather than the Tianheng constitution

### Requirement: Enforced Rust Style
Pacta SHALL prefer compiler and tool reactions over prose-only coding style rules.

#### Scenario: Formatting is enforced
- **WHEN** Rust code is changed
- **THEN** `cargo fmt --all --check` is the formatting authority

#### Scenario: Lints are enforced
- **WHEN** Rust code is changed
- **THEN** `cargo clippy --all-targets -- -D warnings` is the lint authority

#### Scenario: Unsafe code is forbidden
- **WHEN** current Pacta crates compile
- **THEN** unsafe code is forbidden by crate-level attributes

#### Scenario: Public contract docs are enforced
- **WHEN** public API is added to `pacta-contract`
- **THEN** missing public documentation is reported by a compiler or rustdoc reaction

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
