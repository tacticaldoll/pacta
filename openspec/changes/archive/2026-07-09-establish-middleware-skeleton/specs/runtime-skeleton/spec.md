## MODIFIED Requirements

### Requirement: Pacta-Native Executor
Pacta SHALL expose an executor abstraction using Pacta runtime vocabulary rather than Tower as the core public API.

#### Scenario: Executor is the public execution role
- **WHEN** public runtime APIs refer to the component that fulfills claimed pacts
- **THEN** they use `Executor`

#### Scenario: Execution result uses Pacta vocabulary
- **WHEN** an executor reports the lifecycle result of handling a pact
- **THEN** the result is expressed as an `Outcome` or `Settlement` rather than a Tower response type

#### Scenario: Executor infrastructure errors are distinct
- **WHEN** an executor cannot report a lifecycle outcome because execution infrastructure fails
- **THEN** the failure is represented as the executor error rather than being collapsed into `Outcome::Breached`

#### Scenario: Middleware composes executors
- **WHEN** the executor crate exposes middleware composition
- **THEN** the middleware API accepts an executor and returns an executor without requiring Tower or HTTP types

#### Scenario: Policy remains non-behavioral in the skeleton
- **WHEN** the executor crate exposes policy vocabulary
- **THEN** policy values do not execute retry, timeout, rate-limit, delay, or scheduling behavior

#### Scenario: Tower stays outside the core executor
- **WHEN** the core executor crate is compiled
- **THEN** it does not require `tower` as a dependency

### Requirement: Deferred Runtime Behavior
Pacta SHALL keep orchestration policies out of the first runtime skeleton.

#### Scenario: Retry is not in the skeleton
- **WHEN** the runtime skeleton is implemented
- **THEN** it does not compute retry attempts or backoff

#### Scenario: Timeout is not in the skeleton
- **WHEN** the middleware skeleton is implemented
- **THEN** it does not enforce execution deadlines

#### Scenario: Rate limiting is not in the skeleton
- **WHEN** the middleware skeleton is implemented
- **THEN** it does not throttle or schedule executions

#### Scenario: Tribunal is not in the skeleton
- **WHEN** the runtime skeleton is implemented
- **THEN** it does not move exhausted pacts to Tribunal

#### Scenario: Backends are not in the skeleton
- **WHEN** the runtime skeleton is implemented
- **THEN** it does not add memory, SQLite, Postgres, Redis, or other registry backends
