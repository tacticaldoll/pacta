## MODIFIED Requirements

### Requirement: Prose Governance Reaction
Pacta SHALL run an executable reaction that rejects high-risk stale architecture phrases in active prose.

#### Scenario: Governance rejects stale phrase
- **WHEN** the governance command checks active project prose containing a forbidden architecture-defining phrase
- **THEN** the governance command fails with a report identifying the file and phrase

#### Scenario: Governance ignores historical prose
- **WHEN** git history or superseded ADRs contain older vocabulary
- **THEN** the prose governance reaction does not fail because of that historical text, because it scans only the active-prose file list
