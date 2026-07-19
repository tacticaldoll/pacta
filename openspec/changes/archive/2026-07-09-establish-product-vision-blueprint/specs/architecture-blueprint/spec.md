## ADDED Requirements

### Requirement: Plugin Kernel
Pacta SHALL treat the thinnest durable execution core as a plugin kernel over user-defined obligations.

#### Scenario: Kernel owns the contract lifecycle
- **WHEN** active architecture prose describes the Pacta core
- **THEN** it names the durable lifecycle as `Signal -> Pact -> Claim -> Execution -> Settlement`

#### Scenario: Kernel leaves obligations to users
- **WHEN** active architecture prose describes business behavior
- **THEN** it states that users define obligation semantics outside the durable lifecycle kernel

### Requirement: Extension Surfaces
Pacta SHALL define future growth through named extension surfaces rather than feature phases.

#### Scenario: Surfaces are named
- **WHEN** active blueprint prose describes extensibility
- **THEN** it names user-defined obligation, execution composition, lifecycle persistence, and integration boundary as extension surfaces

#### Scenario: Examples are non-commitments
- **WHEN** active blueprint prose gives examples for an extension surface
- **THEN** it states or structurally implies that the examples are possible patterns rather than required roadmap items

### Requirement: Pattern Growth
Pacta SHALL grow by introducing governed design patterns that compose around the kernel without backflow into the kernel.

#### Scenario: New pattern declares its surface
- **WHEN** a future proposal introduces a new runtime pattern
- **THEN** it identifies the extension surface it belongs to before implementation

#### Scenario: Backflow is rejected
- **WHEN** a future proposal would move orchestration, scheduling, routing, or adapter behavior into the durable lifecycle kernel
- **THEN** the proposal must reject that design or redesign it as an extension pattern

### Requirement: Blueprint Boundary
Pacta SHALL use its architecture blueprint to define boundaries, not to create mandatory implementation phases.

#### Scenario: Backlog avoids phase commitments
- **WHEN** `BACKLOG.md` describes future work
- **THEN** it groups work as deferred decisions or candidate patterns rather than sequential required phases

#### Scenario: Blueprint blocks feature accretion
- **WHEN** future work is proposed because another queue, broker, or workflow framework has a feature
- **THEN** the blueprint requires the proposal to justify the feature as a thin Pacta pattern before it enters core scope
