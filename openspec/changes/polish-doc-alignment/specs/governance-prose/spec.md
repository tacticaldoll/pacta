## MODIFIED Requirements

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
