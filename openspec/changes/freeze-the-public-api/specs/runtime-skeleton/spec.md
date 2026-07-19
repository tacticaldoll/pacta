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

#### Scenario: Tower stays outside the core executor
- **WHEN** the core executor crate is compiled
- **THEN** it does not require `tower` as a dependency

## ADDED Requirements

### Requirement: Runtime Result Types Declare Their Exhaustiveness
Pacta SHALL give each public runtime result type a deliberate exhaustiveness stance,
chosen by whether its shape is expected to grow, so freezing at 0.1.0 is a decision
rather than an accident. The driver's `Step` (a runtime-loop status that already
carries a non-settlement `Idle` state and foresees further loop states) and
`DriverError` (an open error enumeration) SHALL be `#[non_exhaustive]`, and
`Execution` — the executor's designated input, which exists to carry future
execution context — SHALL be `#[non_exhaustive]` while retaining its constructor.

#### Scenario: Driver result enums are non-exhaustive
- **WHEN** `Step` and `DriverError` are compiled
- **THEN** both are `#[non_exhaustive]`, so a downstream exhaustive match requires a wildcard arm and a later added variant is not breaking

#### Scenario: Execution is an extensible input seam
- **WHEN** `Execution` gains execution-context fields in a later release
- **THEN** downstream executors that read `Execution` continue to compile, because it is `#[non_exhaustive]` and constructed through `Execution::new`
