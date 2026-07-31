## Purpose

Define Pacta's canonical public domain language for contract concepts, the
public `pacta-contract` API, roadmap terminology, and the boundary between
domain vocabulary and private implementation mechanics.

## Requirements

### Requirement: Canonical Public Vocabulary
Pacta SHALL define canonical public terms for its core contract-domain concepts.

#### Scenario: Naming durable work
- **WHEN** public APIs, specs, or user-facing documentation refer to the durable unit of obligation
- **THEN** they use `Pact`

#### Scenario: Naming pact grouping
- **WHEN** public APIs, specs, or user-facing documentation refer to the grouping from which pacts are selected
- **THEN** they use `Docket` instead of lane, queue, or topic terminology

#### Scenario: Naming pact data
- **WHEN** public APIs, specs, or user-facing documentation refer to business data carried by a pact
- **THEN** they use `Clause` instead of payload or body terminology

#### Scenario: Naming operational context
- **WHEN** public APIs, specs, or user-facing documentation refer to non-business attributes attached to a pact
- **THEN** they use `Brief` instead of metadata or headers terminology

### Requirement: Contract API Naming
The `pacta-contract` crate SHALL expose the contract-domain vocabulary in its public API.

#### Scenario: Naming storage lifecycle role
- **WHEN** the contract crate exposes the pure lifecycle storage trait
- **THEN** the trait is named `Registry`

#### Scenario: Naming claim authority
- **WHEN** the contract crate exposes short-term processing authority over a pact
- **THEN** the authority object is named `Claim` and its proof token is named `Retainer`

#### Scenario: Naming lifecycle methods
- **WHEN** the contract crate exposes lifecycle transitions for acquisition, successful completion, and failed completion
- **THEN** the methods are named `claim`, `fulfill`, and `breach`

### Requirement: Architectural Axioms Remain Intact
Pacta SHALL preserve the foundation axioms while changing public names.

#### Scenario: Registry remains pure lifecycle
- **WHEN** the storage role is renamed from store to registry
- **THEN** it still does not compute retries, backoff, routing, priority, or inspect clauses

#### Scenario: Execution remains middleware-oriented
- **WHEN** public documentation introduces `Executor` terminology
- **THEN** execution orchestration remains delegated to Pacta-native middleware and policy composition

#### Scenario: Tower remains adapter terminology
- **WHEN** Tower integration is introduced
- **THEN** Tower terms remain in adapter-owned crates rather than defining core runtime APIs

#### Scenario: Runtime loop remains mechanical
- **WHEN** private implementation refers to the loop that polls a registry and drives execution
- **THEN** it may use `Driver` terminology as a mechanical implementation term

#### Scenario: Contract remains isolated
- **WHEN** public names change in `pacta-contract`
- **THEN** the crate remains governed as the isolated contract crate with no dependency on other workspace crates

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

#### Scenario: Naming middleware policies
- **WHEN** public APIs, specs, or user-facing documentation refer to retry, timeout, rate limit, or similar execution orchestration
- **THEN** they use clear Pacta-native engineering terms such as `Middleware` and `Policy`

### Requirement: Roadmap Uses Pacta Terms
Pacta SHALL use canonical domain terms in planning and roadmap documents.

#### Scenario: Future driver work
- **WHEN** `BACKLOG.md` describes future runtime-loop work
- **THEN** it uses `Executor`, `Registry`, `Pact`, and `Docket` terminology where those concepts are public

#### Scenario: Future backend work
- **WHEN** `BACKLOG.md` describes future backend or conformance work
- **THEN** it refers to registry behavior rather than store behavior where the public role is meant

#### Scenario: Future terminal review work
- **WHEN** `BACKLOG.md` describes dead-letter or exhausted-pact handling
- **THEN** it uses `Tribunal` terminology
