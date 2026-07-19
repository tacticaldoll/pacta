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

#### Scenario: Tower stays outside the core executor
- **WHEN** the core executor crate is compiled
- **THEN** it does not require `tower` as a dependency

### Requirement: Mechanical Driver Skeleton
Pacta SHALL provide a driver skeleton that mechanically composes a `Registry` with an executor.

#### Scenario: Driver claims by docket
- **WHEN** the driver is asked to perform one execution step
- **THEN** it claims a pact from the configured dockets through `Registry::claim`

#### Scenario: Driver fulfills successful execution
- **WHEN** executor execution reports `Outcome::Fulfilled`
- **THEN** the driver settles the claim with `Registry::fulfill`

#### Scenario: Driver breaches failed execution
- **WHEN** executor execution reports `Outcome::Breached`
- **THEN** the driver settles the claim with `Registry::breach`

#### Scenario: Driver surfaces executor infrastructure error
- **WHEN** executor execution returns an executor error
- **THEN** the driver attempts to settle the claim with `Registry::breach` and returns an executor error to the caller

#### Scenario: Empty docket is idle
- **WHEN** the registry returns no claim
- **THEN** the driver reports an idle step without calling the executor
