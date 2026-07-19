## ADDED Requirements

### Requirement: Workspace Governance Coverage
Every workspace crate SHALL be governed by a dependency boundary that carries a
written justification, and an executable check SHALL fail when any workspace crate
lacks a boundary, so a crate cannot enter the workspace silently or ungoverned.
The workspace is meant to stay thin: it owns the core contract, the runtime
skeleton, governance, the conformance suite, and one dependency-free reference
backend; durable or production backends are expected to live outside the workspace
and prove themselves against the conformance suite, so a new workspace crate's
justification must address why the thin library, rather than a composer, owns it.

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
