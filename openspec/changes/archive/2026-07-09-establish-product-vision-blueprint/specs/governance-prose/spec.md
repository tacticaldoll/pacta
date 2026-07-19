## ADDED Requirements

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

#### Scenario: Governance ignores archived history
- **WHEN** archived ADRs or archived OpenSpec changes contain older vocabulary
- **THEN** the prose governance reaction does not fail solely because of that historical text
