## ADDED Requirements

### Requirement: Runtime Universe Vocabulary
Pacta SHALL define canonical public terms for its runtime skeleton before exposing runtime APIs.

#### Scenario: Naming execution role
- **WHEN** public APIs, specs, or user-facing documentation refer to the role that handles claimed pacts
- **THEN** they use `Executor`

#### Scenario: Naming execution process
- **WHEN** public APIs, specs, or user-facing documentation refer to a single pact handling attempt
- **THEN** they use `Execution`

#### Scenario: Naming execution result
- **WHEN** public APIs, specs, or user-facing documentation refer to the result of handling a pact
- **THEN** they use `Outcome` or `Settlement`

#### Scenario: Naming mechanical loop
- **WHEN** public APIs, specs, or user-facing documentation refer to the loop that claims, executes, and settles pacts
- **THEN** they may use `Driver` as a mechanical implementation term

#### Scenario: Naming middleware policies
- **WHEN** public APIs, specs, or user-facing documentation refer to retry, timeout, rate limit, or similar execution orchestration
- **THEN** they use clear engineering terms such as `Middleware` and `Policy`

## MODIFIED Requirements

### Requirement: Architectural Axioms Remain Intact
Pacta SHALL preserve the foundation axioms while changing public names.

#### Scenario: Registry remains pure lifecycle
- **WHEN** the storage role is renamed from store to registry
- **THEN** it still does not compute retries, backoff, routing, priority, or inspect clauses

#### Scenario: Execution remains middleware-oriented
- **WHEN** public documentation introduces `Executor` terminology
- **THEN** execution orchestration remains delegated to the middleware ecosystem

#### Scenario: Tower remains adapter terminology
- **WHEN** Tower integration is introduced
- **THEN** Tower terms remain in adapter crates or features rather than defining core runtime APIs

#### Scenario: Runtime loop remains mechanical
- **WHEN** private implementation refers to the loop that polls a registry and drives execution
- **THEN** it may use `Driver` terminology as a mechanical implementation term

#### Scenario: Contract remains isolated
- **WHEN** public names change in `pacta-contract`
- **THEN** the crate remains governed as the isolated zero-dependency contract crate
