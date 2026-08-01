## MODIFIED Requirements

### Requirement: Release Metadata
Each publishable crate SHALL carry the metadata crates.io needs: a `description`, a
`license`, a `repository`, and discoverability metadata (`keywords`, `categories`),
sourced from `workspace.package` where shared, and a crate-specific `readme`. The
`readme` SHALL resolve to a file within the crate, not the shared workspace-root
README, so each crate's crates.io page documents that crate rather than the
workspace as a whole. `pacta-governance`'s `check` command SHALL verify this
requirement, so a regression cannot ship silently through prose review alone.

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

#### Scenario: Governance verifies release metadata
- **WHEN** `pacta-governance`'s `check` command runs
- **THEN** it fails if any publishable crate's manifest is missing a required
  metadata field, or resolves `readme` to the shared workspace-root README instead
  of a crate-local file
