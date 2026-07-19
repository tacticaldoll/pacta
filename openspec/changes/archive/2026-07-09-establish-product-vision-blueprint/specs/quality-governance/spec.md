## ADDED Requirements

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
