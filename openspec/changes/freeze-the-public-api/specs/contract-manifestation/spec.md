## ADDED Requirements

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
