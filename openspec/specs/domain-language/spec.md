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

### Requirement: Retainer Encapsulation
The `Retainer` proof token SHALL encapsulate its identifier rather than expose it as
a bare public field, matching its documented role as an authority token that a
registry validates.

#### Scenario: Retainer identifier is not a bare public field
- **WHEN** the contract crate exposes `Retainer`
- **THEN** its identifier is constructed through a constructor and read through an accessor rather than a public field

#### Scenario: Retainer authority is registry-validated
- **WHEN** a settlement presents a `Retainer`
- **THEN** the registry validates it against the claim it issued, rather than the type system proving authority by construction

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

### Requirement: Vocabulary As Governance
Pacta SHALL treat its domain vocabulary as a governance boundary that protects the product architecture.

#### Scenario: Vocabulary carries product intent
- **WHEN** active project prose introduces Pacta domain terms
- **THEN** it explains that names such as `Pact`, `Docket`, `Clause`, `Brief`, `Registry`, `Claim`, `Retainer`, `Fulfill`, `Breach`, and `Tribunal` constrain architecture rather than decorate it

#### Scenario: Old vocabulary remains historical
- **WHEN** old queue, lane, task, store, worker, consumer, or Tower-first vocabulary is mentioned
- **THEN** the prose marks it as legacy, comparison, or adapter scope rather than current Pacta core language

#### Scenario: Elegance is expressed through names
- **WHEN** domain vocabulary is expanded
- **THEN** new names preserve Pacta's elegant, thin, obligation-centered worldview without hiding ordinary engineering mechanics

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

### Requirement: Lease Window Vocabulary
Pacta SHALL treat `Lease` as the canonical term for the bounded validity window
during which a claim's `Retainer` is authoritative, distinct from the retired
"lease token" name that `Retainer` superseded.

#### Scenario: Naming the claim validity window
- **WHEN** public APIs, specs, or user-facing documentation refer to the bounded
  window during which a claim's retainer stays authoritative
- **THEN** they use `Lease` for that window

#### Scenario: Lease window is not the legacy lease token
- **WHEN** documentation uses `Lease`
- **THEN** it names the validity window and not the proof token, which is named
  `Retainer` — the term that superseded the legacy "lease token"

### Requirement: Lifecycle Recovery Vocabulary
Pacta SHALL treat `Lapse` as the canonical term for lifecycle recovery of a pact
whose retainer expired without settlement, rather than requeue, redeliver, or
retry terminology.

#### Scenario: Naming lease recovery
- **WHEN** public APIs, specs, or user-facing documentation refer to recovering a
  pact whose retainer expired without settlement
- **THEN** they use `Lapse` rather than requeue, redeliver, or retry terminology

#### Scenario: Lapse is recovery, not retry policy
- **WHEN** documentation explains `Lapse`
- **THEN** it presents `Lapse` as making a pact claimable again and distinguishes
  it from retry, backoff, or attempt-limit policy, which are not registry behavior

### Requirement: Reciprocal Obligation Vocabulary
Pacta SHALL name the recovery contract as at-least-once recovery paired with an
idempotent user obligation, keeping the guarantee and the obligation as governed
terms rather than implicit assumptions.

#### Scenario: Naming the recovery guarantee
- **WHEN** documentation describes what Pacta guarantees when a holder stops
  before settlement
- **THEN** it names the guarantee at-least-once recovery and does not name it
  exactly-once delivery

#### Scenario: Naming the reciprocal user obligation
- **WHEN** documentation describes what the user owes in return for at-least-once
  recovery
- **THEN** it names an idempotent `Executor` as the user's reciprocal obligation

