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

### Requirement: Active Prose Governance
Pacta SHALL include active prose drift in its executable governance reaction.

#### Scenario: Prose gate runs with architecture governance
- **WHEN** `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** it checks active project prose for high-risk stale architecture phrases before accepting the workspace

#### Scenario: Prose gate reports location
- **WHEN** active project prose violates a stale-phrase rule
- **THEN** the governance output identifies the relative file path, line number, phrase, and reason

#### Scenario: Historical prose is not governed
- **WHEN** git history or superseded ADRs contain historical architecture vocabulary
- **THEN** active prose governance does not fail on that historical text, because it scans only the active-prose file list

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

### Requirement: Workspace Governance Coverage
Every workspace crate SHALL be governed by a dependency boundary that carries a
written justification, and an executable check SHALL fail when any workspace crate
lacks a boundary, so a crate cannot enter the workspace silently or ungoverned.
The workspace is meant to stay thin: it owns the core contract, the runtime
skeleton, governance, the conformance suite, one dependency-free reference backend,
and — because the workspace publishes to crates.io — one curated facade crate that
is the published entrypoint. Durable or production backends are expected to live
outside the workspace and prove themselves against the conformance suite, so a new
workspace crate's justification must address why the thin library, rather than a
composer, owns it. Owning the published entrypoint is a publisher concern the thin
library legitimately holds: it is a pure re-export surface, governed to carry no
logic, and is distinct from a composer's batteries-included convenience.

#### Scenario: An ungoverned crate fails the coverage check
- **WHEN** a workspace crate has no dependency boundary
- **THEN** the executable coverage check fails

#### Scenario: A new crate cannot enter silently
- **WHEN** a crate is added to the workspace
- **THEN** it cannot pass governance until a dependency boundary with a written
  justification is declared for it

#### Scenario: Every dependency boundary is justified
- **WHEN** a dependency boundary is declared
- **THEN** it carries the reason the boundary exists

#### Scenario: The published entrypoint is owned by the thin library
- **WHEN** the facade crate that serves as the published entrypoint is added to the
  workspace
- **THEN** it is governed by a dependency boundary with a written justification, and
  its justification rests on its being a pure re-export entrypoint the publisher
  owns, not a batteries-included convenience

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

