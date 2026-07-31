# Runtime Skeleton Specification

## Purpose

Define Pacta's first runtime skeleton: a Pacta-native executor abstraction, a
mechanical driver loop, and the runtime behaviors deliberately deferred from the
initial skeleton.

## Requirements

### Requirement: Pacta-Native Executor
Pacta SHALL expose an executor abstraction using Pacta runtime vocabulary rather than Tower as the core public API.

#### Scenario: Executor is the public execution role
- **WHEN** public runtime APIs refer to the component that fulfills claimed pacts
- **THEN** they use `Executor`

#### Scenario: Execution result uses Pacta vocabulary
- **WHEN** an executor reports the result of handling a pact
- **THEN** the result is expressed as an `Outcome` or `Settlement` rather than a Tower response type

#### Scenario: Tower stays outside the core executor
- **WHEN** the core executor crate is compiled
- **THEN** it does not require `tower` as a dependency

### Requirement: Mechanical Driver Skeleton
Pacta SHALL provide a driver skeleton that mechanically composes a `Registry` with an executor.

#### Scenario: Driver claims by docket
- **WHEN** the driver is asked to perform one execution step
- **THEN** it claims a pact from the configured dockets through `Registry::claim`

#### Scenario: Driver fulfills successful execution
- **WHEN** executor execution succeeds
- **THEN** the driver settles the claim with `Registry::fulfill`

#### Scenario: Driver breaches failed execution
- **WHEN** executor execution fails
- **THEN** the driver settles the claim with `Registry::breach`

#### Scenario: Empty docket is idle
- **WHEN** the registry returns no claim
- **THEN** the driver reports an idle step without calling the executor

### Requirement: Deferred Runtime Behavior
Pacta SHALL keep orchestration policies out of the first runtime skeleton.

#### Scenario: Retry is not in the skeleton
- **WHEN** the runtime skeleton is implemented
- **THEN** it does not compute retry attempts or backoff

#### Scenario: Tribunal is not in the skeleton
- **WHEN** the runtime skeleton is implemented
- **THEN** it does not move exhausted pacts to Tribunal

#### Scenario: Backends are not in the skeleton
- **WHEN** the runtime skeleton is implemented
- **THEN** it does not add memory, SQLite, Postgres, Redis, or other registry backends
