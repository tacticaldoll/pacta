# release-packaging Specification

## Purpose
Define Pacta's crates.io release hygiene: the publishable crate set (with the governance gate unpublished), a version-carrying inter-member dependency graph, required crate metadata, a declared and CI-verified MSRV, an honest changelog, and a release scope the README states without overstating ingress or durability.
## Requirements
### Requirement: Publishable Crate Set
Pacta SHALL declare an explicit publishable crate set for crates.io. The crates
`pacta`, `pacta-contract`, `pacta-executor`, `pacta-driver`, `pacta-memory`, and
`pacta-conformance` SHALL be publishable — including the curated `pacta` facade,
which is the recommended entrypoint — and `pacta-governance` SHALL remain
unpublished because it is an internal governance gate that depends on `tianheng`.

#### Scenario: Core, backend, conformance, and facade crates are publishable
- **WHEN** the workspace manifests are read
- **THEN** `pacta`, `pacta-contract`, `pacta-executor`, `pacta-driver`, `pacta-memory`, and `pacta-conformance` each resolve to `publish = true`

#### Scenario: The governance gate is not published
- **WHEN** the workspace manifests are read
- **THEN** `pacta-governance` resolves to `publish = false`

### Requirement: Publishable Dependency Graph
Pacta SHALL keep the publishable crates' dependency graph resolvable on crates.io.
Every intra-workspace dependency of a publishable crate SHALL carry a version
requirement in addition to its path, so `cargo publish` accepts it.

#### Scenario: Workspace path dependencies carry a version
- **WHEN** a publishable crate depends on another workspace crate
- **THEN** that dependency declares a `version` alongside its `path`

#### Scenario: The publishable graph packages cleanly
- **WHEN** `cargo publish --dry-run --workspace` runs
- **THEN** it succeeds for the publishable crates, verifying the inter-member dependency graph locally rather than only that names are free (a per-crate dry-run of a dependent cannot succeed until its dependencies are actually on crates.io)

### Requirement: Release Metadata
Each publishable crate SHALL carry the metadata crates.io needs: a `description`, a
`license`, a `repository`, and discoverability metadata (`keywords`, `categories`),
sourced from `workspace.package` where shared, and a crate-specific `readme`. The
`readme` SHALL resolve to a file within the crate, not the shared workspace-root
README, so each crate's crates.io page documents that crate rather than the
workspace as a whole.

#### Scenario: Publishable crates carry required metadata
- **WHEN** a publishable crate's manifest is read
- **THEN** it resolves a non-empty `description`, `license`, `repository`, and `readme`

#### Scenario: Each publishable crate resolves its own README
- **WHEN** a publishable crate's `readme` is resolved
- **THEN** it points to a README file inside that crate rather than the shared
  workspace-root README, so the crate's crates.io page documents the crate itself

#### Scenario: Discoverability metadata is present
- **WHEN** a publishable crate's manifest is read
- **THEN** it resolves `keywords` and `categories` for crates.io discovery

### Requirement: Declared And Verified MSRV
Pacta SHALL declare a minimum supported Rust version and verify it, so the declared
floor is enforced rather than aspirational.

#### Scenario: MSRV is declared
- **WHEN** the workspace manifest is read
- **THEN** it declares a `rust-version` reflecting the language features the workspace uses

#### Scenario: CI verifies the MSRV builds
- **WHEN** CI runs
- **THEN** it builds the workspace on the declared MSRV toolchain and fails if the workspace does not compile on it

### Requirement: Changelog
Pacta SHALL maintain a changelog documenting released versions honestly.

#### Scenario: The 0.1.0 release is recorded
- **WHEN** `CHANGELOG.md` is read
- **THEN** it contains a `0.1.0` entry describing what the release ships

### Requirement: Honest Release Scope
The README SHALL state what the release actually ships and what is deferred, so a
first-time crates.io consumer is not misled about durability, ingress, or adapters
that are not part of the release.

#### Scenario: README states the release scope
- **WHEN** the README is read
- **THEN** it states that the release ships the curated `pacta` facade entrypoint, the lifecycle contract, the sans-I/O kernel, the in-memory reference backend, the conformance suite, and executable governance, and that durable backends live outside the workspace and prove themselves against the conformance suite

#### Scenario: Ingress is not overstated
- **WHEN** the README depicts the lifecycle flow
- **THEN** it does not present a `Signal`-to-`Pact` ingress as a shipped public API, because no ingress API is part of the release

