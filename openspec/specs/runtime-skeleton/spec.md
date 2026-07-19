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

### Requirement: Sans-I/O Lifecycle Kernel
Pacta SHALL express the pact lifecycle as a sans-I/O kernel: a pure state machine
that decides the next `Directive` from its state and absorbs `Notice` reports,
performing no I/O and exposing no `async fn`. The kernel SHALL preserve the
existing lifecycle decisions without adding orchestration behavior, and SHALL
fabricate no `Outcome` it was not given — it settles only outcomes execution
actually produced.

#### Scenario: Kernel performs no I/O
- **WHEN** the kernel advances the lifecycle
- **THEN** it does not claim, execute, or settle directly; it only issues directives describing those actions for a runtime to perform

#### Scenario: Kernel exposes no async
- **WHEN** the kernel's public API is compiled
- **THEN** it exposes no `async fn`, committing to no runtime shape

#### Scenario: Kernel decides settlement from execution outcome
- **WHEN** a runtime feeds an execution outcome notice back to the kernel
- **THEN** the kernel decides `Outcome::Fulfilled` as a fulfill settlement and `Outcome::Breached` as a breach settlement

#### Scenario: Kernel leaves an executor infrastructure error unsettled
- **WHEN** a runtime feeds an executor infrastructure error notice back to the kernel
- **THEN** the kernel settles nothing and reaches an unsettled terminal, fabricating no `Outcome`, so the claim is left held-but-unsettled to lapse and be reclaimed while the runtime surfaces the error to its caller

#### Scenario: Kernel is idle when nothing is claimed
- **WHEN** a runtime feeds an empty claim notice to the kernel
- **THEN** the kernel issues an idle directive and no execution is requested

#### Scenario: No orchestration is added
- **WHEN** the kernel is implemented
- **THEN** it adds no retry, backoff, timeout, rate-limit, heartbeat scheduling, or Tribunal behavior, keeping those deferred

### Requirement: Mechanical Driver Skeleton
Pacta SHALL provide a driver that mechanically performs the directives the
sans-I/O kernel issues, composing a `Registry` and an executor without deciding
lifecycle outcomes itself.

#### Scenario: Driver performs the claim directive by docket
- **WHEN** the kernel issues a claim directive for the configured dockets
- **THEN** the driver claims through `Registry::claim` and feeds the result back to the kernel as a notice

#### Scenario: Driver performs the execute directive
- **WHEN** the kernel issues an execute directive for a claimed pact
- **THEN** the driver runs the executor and feeds the resulting `Outcome` or executor error back to the kernel as a notice

#### Scenario: Driver performs the settlement the kernel decides
- **WHEN** the kernel issues a settle directive with `Outcome::Fulfilled` or `Outcome::Breached`
- **THEN** the driver settles the claim with `Registry::fulfill` or `Registry::breach` respectively

#### Scenario: Driver surfaces executor infrastructure error and settles nothing
- **WHEN** the executor returns an executor error for an execute directive
- **THEN** the driver settles nothing, returns the executor error to the caller, and leaves the claim unsettled so its lease lapses and the pact is reclaimed

#### Scenario: Empty docket is idle
- **WHEN** the registry returns no claim for a claim directive
- **THEN** the kernel issues an idle directive and the driver runs no executor

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

### Requirement: Runtime Injects Current Time
The runtime SHALL supply the current time to time-dependent registry operations
while the sans-I/O kernel stays time-free, so reading the clock is a runtime
concern and the kernel commits to no time source.

#### Scenario: The driver injects time into time-dependent registry operations
- **WHEN** the driver performs a claim directive
- **THEN** it obtains the current time and passes it to the time-dependent
  registry operation rather than the registry reading a clock itself

#### Scenario: The kernel issues time-free directives
- **WHEN** the kernel issues a claim directive
- **THEN** the directive carries no time, and the runtime attaches the current
  time when it performs the directive

### Requirement: Public Errors Are Standard Errors
Pacta's public runtime trait errors SHALL be standard errors, so a consumer can
display them, chain them, and convert them into common error types.

#### Scenario: Registry error is a standard error
- **WHEN** the `Registry` trait declares its associated error type
- **THEN** that type is bound by `std::error::Error`

#### Scenario: Executor error is a standard error
- **WHEN** the `Executor` trait declares its associated error type
- **THEN** that type is bound by `std::error::Error`

#### Scenario: The driver error is displayable and chainable
- **WHEN** the driver returns its error
- **THEN** the error implements `Display` and `std::error::Error`, and exposes the
  underlying registry or executor error as its source

#### Scenario: A shipped backend error is a standard error
- **WHEN** a shipped registry backend returns its error
- **THEN** the error implements `Display` and `std::error::Error`

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

