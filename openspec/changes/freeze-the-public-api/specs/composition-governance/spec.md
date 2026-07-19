## MODIFIED Requirements

### Requirement: Pacta-Native Composition Boundary
Pacta SHALL define execution composition through Pacta-native middleware and pattern vocabulary before exposing adapter-specific APIs.

#### Scenario: Core composition uses Pacta terms
- **WHEN** public core runtime APIs refer to execution orchestration
- **THEN** they use Pacta-native terms such as `Executor`, `Execution`, `Outcome`, `Settlement`, and `Middleware`

#### Scenario: Middleware wraps executors
- **WHEN** core runtime APIs expose middleware composition
- **THEN** middleware wraps an `Executor` into another `Executor` using Pacta-native execution vocabulary

#### Scenario: Foreign framework vocabulary stays outside core
- **WHEN** public core runtime APIs are added or changed
- **THEN** they do not use Tower, HTTP, request, response, service, or layer vocabulary as the governing public shape

#### Scenario: Patterns attach at extension surfaces
- **WHEN** public composition APIs introduce a new behavior pattern
- **THEN** the API identifies whether the behavior belongs to user-defined obligation, execution composition, lifecycle persistence, or integration boundary scope

## ADDED Requirements

### Requirement: User-Obligation Delivery Pattern
Pacta SHALL deliver its user obligations through a stated, executably-proven pattern,
so a consumer knows how to fulfill an obligation and the pattern cannot silently
regress. Each user obligation SHALL be a trait, not an inert value. The execution
obligation SHALL follow the Service/Layer shape: `Executor` is the request handler
(narrowed to the lifecycle — input `Execution`, result `Outcome`), and `Middleware`
is the decorator that wraps an `Executor` into an `Executor`, giving the closure
property under which middleware compose arbitrarily. The persistence obligation
(`Registry`) SHALL follow trait-plus-conformance: a backend is any type implementing
the trait and is validated by `pacta-conformance`. Each shipped obligation trait
SHALL have an in-workspace consumer and at least one reference or exercising
implementation, so no obligation is a contract without a client.

#### Scenario: Execution composition has the closure property
- **WHEN** `cargo test --workspace` runs
- **THEN** a test stacks two pass-through middleware over an executor and drives the composed executor to a settlement, proving that `Middleware` composes `Executor` into `Executor` and failing if the closure property regresses

#### Scenario: Every obligation trait ships with a consumer and a validator
- **WHEN** a user-obligation trait (`Registry`, `Executor`, `Middleware`) is part of the surface
- **THEN** the workspace also ships a consumer that drives it and a reference or exercising implementation that validates its shape, and does not ship an obligation type that no consumer exercises

#### Scenario: Deferred orchestration attaches to the seam, not the core
- **WHEN** concrete orchestration (retry, timeout, rate-limit) is introduced later
- **THEN** it arrives as `Middleware` implementations composed onto the existing seam — together with any policy trait and stack-assembler it needs, co-designed so each has a real client — rather than as inert vocabulary frozen into the core ahead of its consumer
