## MODIFIED Requirements

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
