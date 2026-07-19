## MODIFIED Requirements

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
