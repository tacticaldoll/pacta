# contract-manifestation Specification

## Purpose
Project Pacta's lifecycle contract onto the `pacta` facade's published documentation — both the implementer half and the user-obligation half — name the reference implementations as reference, and anchor the public composition contract in a compiler-checked facade doctest.
## Requirements
### Requirement: Contract Is Manifest On The Consumer Surface
Pacta SHALL project its lifecycle contract onto the consumer-facing documentation of
the `pacta` facade, so a consumer who reads only the published crate sees both halves
of the contract without reading internal specs. The projection SHALL state the
implementer half (what a `Registry` must satisfy) and the user-obligation half (what
the consumer owes), and SHALL NOT re-specify behavior — it references the governed
truth. The **async binding** SHALL likewise manifest both halves on its own surface,
including the obligations specific to it: the implementer half SHALL state that `apply`
must apply the kernel decision within one atomic scope (or exactly-once and fencing
break) and that `claim` must honor the eligibility invariant as a native, full-scan-free
selection; the user-obligation half SHALL state the same reciprocal obligations as the
sync facade, and SHALL additionally state that the runtime and its coloring (async,
`Send`, executor choice) are the consumer's to compose — the async binding does not force
a runtime property.

#### Scenario: The facade documents the implementer half
- **WHEN** a consumer reads the `pacta` crate-root documentation
- **THEN** it states that a `Registry` must provide the claim, heartbeat, fulfill, and breach lifecycle with a lease, and that `pacta-conformance` is the executable proof a backend satisfies it

#### Scenario: The facade documents the user-obligation half
- **WHEN** a consumer reads the `pacta` crate-root documentation
- **THEN** it states the consumer's reciprocal obligations: an idempotent `Executor` (because recovery is at-least-once, not exactly-once), user-owned lease sizing, and runtime-owned heartbeat cadence

#### Scenario: The async binding documents its implementer half
- **WHEN** a consumer reads the async binding's documentation
- **THEN** it states that a backend implements the selection and the `apply` transition port, that `apply` must apply the kernel decision within one atomic scope (or exactly-once and fencing break), and that `claim` must honor the eligibility invariant as a native, full-scan-free selection

#### Scenario: The async binding documents its user-obligation half
- **WHEN** a consumer reads the async binding's documentation
- **THEN** it states the reciprocal obligations: idempotent work because recovery is at-least-once, user-owned lease sizing, and runtime-owned heartbeat cadence, and that the runtime and its coloring (async, `Send`, executor choice) are the consumer's — the binding forces no runtime property

### Requirement: Reference Pieces Are Named As Reference
Pacta SHALL name its reference implementations as reference on their own documented
surface, and SHALL state the applicability boundary of the reference runtime, so a
consumer does not mistake a reference for a production component.

#### Scenario: The reference backend is named
- **WHEN** a consumer reads the `pacta-memory` documentation
- **THEN** it states that `pacta-memory` is an in-memory reference backend, not a durable or production backend, and that durable backends live outside the workspace and prove themselves against `pacta-conformance`

#### Scenario: The async reference backend is named
- **WHEN** a consumer reads the `pacta-memory-async` documentation
- **THEN** it states that `pacta-memory-async` is the in-memory reference backend for the async binding, not a durable or production backend, and that durable async backends live outside the workspace and prove themselves the same way

#### Scenario: The reference runtime states its boundary
- **WHEN** a consumer reads the `Driver` documentation
- **THEN** it states that `Driver` drives synchronously and does not heartbeat a claim in flight, so it is safe for tasks shorter than the lease and for single-worker use, and that long or multi-worker durable workloads compose their own loop over the `Registry` contract

### Requirement: The Composition Contract Is Compiler-Checked
Pacta SHALL anchor its public composition contract in a doctest at the `pacta` facade
crate root, so "claim, execute through middleware, and settle composes through the
public surface" is verified by the test gate rather than only asserted by an example
binary.

#### Scenario: The facade doctest composes the lifecycle
- **WHEN** `cargo test --workspace` runs
- **THEN** a `pacta` crate-root doctest claims a pact, executes it through a pass-through `Middleware`, and settles it through the facade's public API, and fails if that composition path stops compiling

### Requirement: Core Contract Records Are Extensible
Pacta SHALL keep its core contract data records extensible across a published minor
series, so a record can gain a field without a breaking change. The records `Pact`
and `Claim` SHALL be `#[non_exhaustive]` and SHALL provide a constructor
(`Pact::new`, `Claim::new`), so downstream code constructs them through the
constructor rather than a field literal and cannot be broken by a later added field.
Opaque value and identity types (`Retainer`, `Timestamp`) SHALL remain encapsulated
behind a private field with accessors, which already gives them the same freedom.

#### Scenario: A record gains a field without breaking downstream construction
- **WHEN** `Pact` or `Claim` gains a new field in a later minor release
- **THEN** downstream crates that construct it through `Pact::new`/`Claim::new` continue to compile, because the record is `#[non_exhaustive]` and was never constructed by field literal across the crate boundary

#### Scenario: Downstream constructs records through the constructor
- **WHEN** a downstream crate builds a `Pact` or `Claim`
- **THEN** it calls `Pact::new`/`Claim::new` rather than a struct literal, and can still read the public fields

### Requirement: Settlement Outcomes Are A Closed Binary
Pacta SHALL keep `Outcome` a closed enumeration of `Fulfilled` and `Breached`,
because a settled pact is exactly one of those two, and a recommended-tier consumer
SHALL be able to match it exhaustively. `Outcome` SHALL NOT be `#[non_exhaustive]`.

#### Scenario: Outcome is matched exhaustively
- **WHEN** a consumer matches an `Outcome`
- **THEN** matching `Fulfilled` and `Breached` is exhaustive with no wildcard arm required, because the settlement binary is complete and closed

### Requirement: Lease Identity Is A Usable Backend Key
Pacta SHALL make the lease identity usable as a key by durable backends. `Retainer`
SHALL derive `PartialEq`, `Eq`, and `Hash`, because a backend must be able to compare
and index by the holder identity and the orphan rule prevents a backend from adding
those implementations to a type it does not own. This SHALL be proven present by a
compile-time trait-bound assertion.

#### Scenario: A backend uses the retainer as a map key
- **WHEN** a durable backend indexes lease state by holder identity
- **THEN** `Retainer` satisfies `Eq + Hash` so it can be used directly as a key

#### Scenario: The key capability is compiler-proven
- **WHEN** the workspace compiles
- **THEN** a compile-time assertion requires `Retainer: Eq + Hash`, so removing the derives fails the build rather than silently regressing the backend contract

### Requirement: Durable State Serializes, Kernel Protocol Does Not
Pacta SHALL keep its durable lifecycle state serializable and its transient kernel
driving protocol non-serializable, so persistence lives at the durable boundary and
never leaks into the sans-I/O kernel. The durable records (`Pact`, `Claim`,
`Retainer`, `Timestamp`) SHALL implement serde; the kernel driving-protocol types
(`Directive`, `Notice`, `StepResult`) SHALL NOT acquire serde, because they are
transient decisions to be performed now, not durable state.

#### Scenario: Durable records serialize
- **WHEN** a durable backend persists lifecycle state
- **THEN** `Pact`, `Claim`, `Retainer`, and `Timestamp` are serializable through serde

#### Scenario: Kernel protocol stays non-serializable
- **WHEN** the kernel driving-protocol types are inspected
- **THEN** they do not implement serde, because persisting an in-flight `Directive` or `Notice` would contradict the kernel's transient, sans-I/O nature

