## MODIFIED Requirements

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

#### Scenario: Naming middleware orchestration
- **WHEN** public APIs, specs, or user-facing documentation refer to retry, timeout, rate limit, or similar execution orchestration
- **THEN** they use clear Pacta-native engineering terms such as `Middleware`
