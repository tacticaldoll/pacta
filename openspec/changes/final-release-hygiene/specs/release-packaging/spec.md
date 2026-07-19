## MODIFIED Requirements

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
