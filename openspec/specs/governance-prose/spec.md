# Governance Prose Specification

## Purpose

Define authority for active project prose and the executable prose-governance
reaction that prevents architecture drift.
## Requirements
### Requirement: Document Authority
Pacta SHALL define authority for active prose so project documents do not create competing architectures.

#### Scenario: Specs are shipped truth
- **WHEN** an architecture or product rule must be treated as durable project truth
- **THEN** it lives in `openspec/specs/` or an active change delta before implementation

#### Scenario: Project file states vision
- **WHEN** the repository describes Pacta's identity, scope, and non-goals
- **THEN** `PROJECT.md` is the concise vision and positioning authority

#### Scenario: Agent file states operating protocol
- **WHEN** AI agents need repository workflow and review rules
- **THEN** `AGENTS.md` states protocol and points to specs rather than redefining a competing architecture

#### Scenario: Backlog is non-binding
- **WHEN** `BACKLOG.md` lists future work
- **THEN** it presents deferred decisions and candidate patterns rather than mandatory phases

#### Scenario: The Definition of Done is single-sourced
- **WHEN** the pre-commit gate list (the Definition of Done) is documented in active prose
- **THEN** `AGENTS.md` states the complete list and other active prose points to it rather than restating a divergent subset, so the documented gates stay consistent across project prose

### Requirement: Active Prose Vocabulary Governance
Pacta SHALL keep high-risk stale architecture vocabulary out of active project prose except where clearly marked as legacy or comparison context.

#### Scenario: Stale Tower-first claims are forbidden
- **WHEN** active project prose describes Pacta's current architecture
- **THEN** it does not call Pacta Tower-native or Tower-first

#### Scenario: Stale store lifecycle claims are forbidden
- **WHEN** active project prose describes Pacta's current lifecycle
- **THEN** it does not use old store-centered lifecycle method names as current architecture

#### Scenario: Legacy mapping remains allowed
- **WHEN** domain-language documentation explains historical renames or legacy mappings
- **THEN** it may mention old vocabulary only as explicitly historical context

### Requirement: Prose Governance Reaction
Pacta SHALL run an executable reaction that rejects high-risk stale architecture phrases in active prose.

#### Scenario: Governance rejects stale phrase
- **WHEN** the governance command checks active project prose containing a forbidden architecture-defining phrase
- **THEN** the governance command fails with a report identifying the file and phrase

#### Scenario: Governance ignores historical prose
- **WHEN** git history contains older vocabulary
- **THEN** the prose governance reaction does not fail because of that historical text, because it scans only the active-prose file list

