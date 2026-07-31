# Composition Governance Specification

## Purpose

Define Pacta's core composition boundary: Pacta-native middleware and pattern
vocabulary in core crates, adapter-owned framework integrations outside core,
and executable dependency closure for core runtime crates.

## Requirements

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

### Requirement: Adapter Scope
Pacta SHALL treat framework adapters as integration scope rather than core runtime scope.

#### Scenario: Tower compatibility is adapter-owned
- **WHEN** Tower compatibility is introduced
- **THEN** it lives in an adapter-owned crate outside `pacta-contract`, `pacta-executor`, and `pacta-driver`

#### Scenario: Adapter types do not leak back into core
- **WHEN** adapter-owned public types exist
- **THEN** Tianheng semantic governance is updated so core crate public APIs do not expose those adapter-owned types

### Requirement: Core Dependency Closure
Pacta SHALL keep core crate normal dependencies closed by executable governance.

#### Scenario: Core dependency additions require governance amendment
- **WHEN** `pacta-contract`, `pacta-executor`, or `pacta-driver` gains a new normal dependency
- **THEN** the Tianheng constitution must explicitly allow that dependency or fail

#### Scenario: Framework dependencies are rejected from core
- **WHEN** a core crate adds a normal dependency on Tower, HTTP adapter, backend, or other integration framework crates without an explicit governance amendment
- **THEN** the Tianheng governance reaction fails

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

#### Scenario: The composition mechanism ships ahead of concrete orchestration
- **WHEN** the composition mechanism (the empty-stack value, a reified stack value, and a blind assembler) is added
- **THEN** it ships as a reification of the already-proven closure property — its client being that property and every present or future `Middleware` — rather than as inert vocabulary frozen ahead of a consumer, and it exposes no named policy method

#### Scenario: Deferred orchestration attaches to the seam, not the core
- **WHEN** concrete orchestration *policy* (retry, timeout, rate-limit) is introduced later
- **THEN** it arrives as `Middleware` implementations composed onto the existing seam — sibling- or consumer-owned, together with any policy trait it needs, co-designed so each has a real client — rather than as core features, and the core assembler grows no named method for it

### Requirement: Pattern Admission Guardrail
Pacta SHALL admit a composition pattern into a core crate only when it passes a stated
four-question guardrail, so that Pacta's pattern-leading stance ("Consumers ignite pacta's work;
they never gate it") cannot degrade into feature accumulation. A pattern is admissible only if
all four hold: (1) **native** — it is expressed in purely Pacta-native vocabulary; (2)
**sibling-clear** — it steps on no sibling product's domain; (3) **non-goal-clear** — it pulls
toward no stated non-goal (broker, job-queue platform, workflow engine, scheduler, transport
framework); (4) **mechanism-only** — it touches optional composition mechanism, not the durable
contract. A pattern failing any question SHALL be rejected from core or relocated to an extension
surface, a sibling, or the consumer.

#### Scenario: A pattern is admitted only if it passes all four questions
- **WHEN** a new composition pattern is proposed for a core crate
- **THEN** it is admitted only if it is native, sibling-clear, non-goal-clear, and
  mechanism-only, and is otherwise rejected or relocated

#### Scenario: A sibling-domain or non-goal pattern is rejected regardless of demand
- **WHEN** a proposed pattern encodes retry, timeout, backoff, witness, circuit, quota, or
  rate-limit behavior
- **THEN** it is rejected from core for failing the sibling-clear or non-goal-clear question,
  even if a consumer requests it

#### Scenario: The sibling-clear question stays adversarial-review prose by design
- **WHEN** the guardrail is enforced
- **THEN** the sibling-clear question is checked by propose- and apply-phase adversarial review
  rather than an executable reaction, because an executable reaction would have to name the
  siblings it checks against, which sibling-blindness forbids

### Requirement: Durable Retry Is Demonstrated
Pacta SHALL carry an executable example that demonstrates durable retry composed from the
shipped contract: on a failed attempt, a consumer computes a backoff instant — the policy —
and calls `release(retainer, reclaimable_at)` so the pact is withheld until that instant,
then reclaimed and finally settled. The example SHALL use only the public API, keep the
backoff policy in the consumer, inject time rather than read a clock, and self-check its
outcome so the demonstration cannot silently regress under the Definition of Done. The core
SHALL compute no backoff in this composition.

#### Scenario: A failed attempt is withheld, then reclaimed after backoff
- **WHEN** the example runs and an attempt fails
- **THEN** the pact is released with a future reclaimable instant, is not claimable before that instant, and is reclaimed through the normal claim path at or after it, then settled

#### Scenario: The backoff policy stays in the consumer
- **WHEN** the example computes the delay before the next attempt
- **THEN** the delay is computed in consumer code and injected via `release`, and the core computes no backoff interval

#### Scenario: A regressed demonstration fails the gate
- **WHEN** the example no longer reaches its expected outcome
- **THEN** running it under the Definition of Done fails rather than passing silently
