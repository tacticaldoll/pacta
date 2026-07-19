## MODIFIED Requirements

### Requirement: Pacta-Native Composition Boundary
Pacta SHALL define execution composition through Pacta-native middleware, policy, and pattern vocabulary before exposing adapter-specific APIs.

#### Scenario: Core composition uses Pacta terms
- **WHEN** public core runtime APIs refer to execution orchestration
- **THEN** they use Pacta-native terms such as `Executor`, `Execution`, `Outcome`, `Settlement`, `Middleware`, and `Policy`

#### Scenario: Middleware wraps executors
- **WHEN** core runtime APIs expose middleware composition
- **THEN** middleware wraps an `Executor` into another `Executor` using Pacta-native execution vocabulary

#### Scenario: Policy names orchestration intent
- **WHEN** core runtime APIs expose policy vocabulary
- **THEN** policies identify orchestration intent without implementing retry, timeout, rate-limit, delay, or scheduling behavior in the middleware skeleton

#### Scenario: Foreign framework vocabulary stays outside core
- **WHEN** public core runtime APIs are added or changed
- **THEN** they do not use Tower, HTTP, request, response, service, or layer vocabulary as the governing public shape

#### Scenario: Patterns attach at extension surfaces
- **WHEN** public composition APIs introduce a new behavior pattern
- **THEN** the API identifies whether the behavior belongs to user-defined obligation, execution composition, lifecycle persistence, or integration boundary scope

### Requirement: Adapter Scope
Pacta SHALL treat framework adapters as integration scope rather than core runtime scope.

#### Scenario: Tower compatibility is adapter-owned
- **WHEN** Tower compatibility is introduced
- **THEN** it lives in an adapter-owned crate outside `pacta-contract`, `pacta-executor`, and `pacta-driver`

#### Scenario: Adapter types do not leak back into core
- **WHEN** adapter-owned public types exist
- **THEN** Tianheng semantic governance is updated so core crate public APIs do not expose those adapter-owned types
