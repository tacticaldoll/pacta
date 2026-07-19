## MODIFIED Requirements

### Requirement: Pacta-Native Composition Boundary
Pacta SHALL define execution composition through Pacta-native middleware and policy vocabulary before exposing adapter-specific APIs.

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
