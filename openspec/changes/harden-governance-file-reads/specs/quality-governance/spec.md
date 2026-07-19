## ADDED Requirements

### Requirement: Governance Fails Loudly On Unreadable Inputs
The executable governance gate SHALL NOT let a file-scanning check pass vacuously
when a governed input is missing or unreadable. A canonical governed file that
cannot be read SHALL fail the gate rather than be silently skipped, so a governed
file that vanishes is caught rather than granting a free pass — the same
no-vacuous-pass parity the coverage check already enforces.

#### Scenario: A missing active-prose file fails the gate
- **WHEN** a file in the canonical active-prose set cannot be read (absent or unreadable)
- **THEN** the governance gate fails and its output identifies the file, rather than skipping it

#### Scenario: An unreadable facade source file fails the gate
- **WHEN** a facade source file cannot be read during the re-exports-only scan
- **THEN** the governance gate fails rather than skipping that file

#### Scenario: An empty facade source tree fails the gate
- **WHEN** the facade re-exports-only scan finds no facade source files at all
- **THEN** the governance gate fails, mirroring the coverage check's non-vacuous guard, because a facade with no scanned source cannot be certified re-exports-only

#### Scenario: The real workspace stays clean
- **WHEN** the gate runs against the workspace with all governed files present and readable
- **THEN** it reports no vacuous-input failure
