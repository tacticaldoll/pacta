## ADDED Requirements

### Requirement: Contract Is Manifest On The Consumer Surface
Pacta SHALL project its lifecycle contract onto the consumer-facing documentation of
the `pacta` facade, so a consumer who reads only the published crate sees both halves
of the contract without reading internal specs. The projection SHALL state the
implementer half (what a `Registry` must satisfy) and the user-obligation half (what
the consumer owes), and SHALL NOT re-specify behavior — it references the governed
truth.

#### Scenario: The facade documents the implementer half
- **WHEN** a consumer reads the `pacta` crate-root documentation
- **THEN** it states that a `Registry` must provide the claim, heartbeat, fulfill, and breach lifecycle with a lease, and that `pacta-conformance` is the executable proof a backend satisfies it

#### Scenario: The facade documents the user-obligation half
- **WHEN** a consumer reads the `pacta` crate-root documentation
- **THEN** it states the consumer's reciprocal obligations: an idempotent `Executor` (because recovery is at-least-once, not exactly-once), user-owned lease sizing, and runtime-owned heartbeat cadence

### Requirement: Reference Pieces Are Named As Reference
Pacta SHALL name its reference implementations as reference on their own documented
surface, and SHALL state the applicability boundary of the reference runtime, so a
consumer does not mistake a reference for a production component.

#### Scenario: The reference backend is named
- **WHEN** a consumer reads the `pacta-memory` documentation
- **THEN** it states that `pacta-memory` is an in-memory reference backend, not a durable or production backend, and that durable backends live outside the workspace and prove themselves against `pacta-conformance`

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
