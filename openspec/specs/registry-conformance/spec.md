# registry-conformance Specification

## Purpose
Define Pacta's backend-agnostic conformance suite that verifies any `Registry` backend against the same lease lifecycle — claim, settlement, lapse, and heartbeat, including the at-least-once safety property — through the public trait with injected time and a single seeding hook.
## Requirements
### Requirement: Backend-Agnostic Conformance Suite
Pacta SHALL provide a conformance suite that verifies `Registry` lifecycle
behavior independent of any particular backend, so every backend is held to the
same correctness contract.

#### Scenario: The same suite runs against any backend
- **WHEN** a `Registry` backend is subjected to the conformance suite
- **THEN** the suite exercises it through the public `Registry` trait without
  depending on backend-specific internals

#### Scenario: Time is driven through the trait
- **WHEN** the conformance suite exercises time-dependent behavior such as lease
  expiry
- **THEN** it advances time by passing controlled `Timestamp` values into the
  trait rather than waiting on a wall clock

#### Scenario: Claims are scoped to requested dockets
- **WHEN** the conformance suite claims from a docket while a pact sits on a
  different docket
- **THEN** the pact is not claimed, and it is claimable only when its own docket
  is requested

### Requirement: Conformance Seeding Hook
The conformance suite SHALL populate a backend under test through a single defined
seeding hook, so the suite stays generic while each backend supplies its own way
of holding pacts.

#### Scenario: The suite populates a backend under test
- **WHEN** the conformance suite needs a backend containing known pacts
- **THEN** it constructs the backend through the defined seeding hook rather than
  reaching into backend-specific storage

### Requirement: Conformance Covers Lease Lifecycle
The conformance suite SHALL verify claim, settlement, lapse, and heartbeat
behavior, including the at-least-once safety property that a reclaimed pact
rejects the prior holder's authority.

#### Scenario: A claimed pact is settled and no longer claimable
- **WHEN** the suite claims a pact and then fulfills or breaches it
- **THEN** the pact is settled and a further claim returns nothing

#### Scenario: An expired lease is reclaimed
- **WHEN** the suite lets a lease expire and then claims again at a later time
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: A reclaimed pact rejects the prior holder
- **WHEN** the suite reclaims a lapsed pact and the original holder then settles
  with its prior retainer
- **THEN** the settlement is rejected

#### Scenario: Heartbeat guards the lease
- **WHEN** the suite heartbeats within the lease window and again after expiry
- **THEN** the in-window heartbeat extends the lease and the post-expiry heartbeat
  is rejected

### Requirement: Conformance Covers Deferred Reclaim
The conformance suite SHALL verify release and deferred reclaim for any backend: that a
released pact is not claimable before its reclaimable instant, is claimable at or after it,
that an immediate reclaimable instant behaves as a lapse, and that release rotates authority
away from the prior retainer.

#### Scenario: A released pact is withheld until its reclaimable instant
- **WHEN** the suite releases a claim with a future reclaimable instant and claims again before it
- **THEN** the claim returns nothing

#### Scenario: A released pact is reclaimable at its instant
- **WHEN** the suite claims again at or after the reclaimable instant
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: An immediate reclaimable instant reclaims like a lapse
- **WHEN** the suite releases a claim with a reclaimable instant at or before now and claims again
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: Release rotates authority away from the prior holder
- **WHEN** the suite releases a claim and the prior holder then settles with its retainer
- **THEN** the settlement is rejected

### Requirement: Conformance Is Offered For The Async Binding
The conformance suite SHALL verify an `AsyncRegistry` backend against the same lifecycle scenarios it
verifies sync `Registry` backends against, so the async binding is held to the same correctness
contract as the sync binding. The async runner SHALL reuse the sync suite's scenarios rather than a
duplicated scenario set, so sync and async coverage cannot drift. The async runner and its dependency
on the async binding SHALL be gated so a sync-only consumer of the conformance suite pulls no async
binding dependency and no async runtime.

#### Scenario: The async binding runs the same scenarios
- **WHEN** an `AsyncRegistry` backend is subjected to the async conformance runner
- **THEN** it is exercised against the same lifecycle scenarios (claim, settlement, lapse, heartbeat,
  deferred reclaim, at-least-once safety) the sync suite runs, through the public `AsyncRegistry`
  trait

#### Scenario: Coverage is single-sourced, not duplicated
- **WHEN** the async runner exercises a scenario
- **THEN** it drives the same scenario definition the sync suite uses rather than a parallel copy, so
  a change to a scenario applies to both bindings at once

#### Scenario: Sync-only consumers pull no async dependency
- **WHEN** a sync-only consumer builds the conformance suite without opting into the async runner
- **THEN** the build pulls neither the async binding crate nor an async runtime

### Requirement: Conformance Covers Concurrent Transition Contention
The conformance suite SHALL verify, for the async binding, that concurrent transitions contending on
a single claimed pact preserve the at-most-once application guaranteed by the set-if-unchanged fence —
the race surface the async binding's `load`-then-`cas` decomposition introduces and the sync fat-verb
shape does not have. This SHALL be demonstrated against the reference async backend under genuine
multi-threaded parallelism.

#### Scenario: Contending transitions apply at most once
- **WHEN** two workers concurrently attempt a transition on the same claimed pact
- **THEN** exactly one set-if-unchanged succeeds and the other reloads to a not-current-holder, so the
  transition is applied at most once

#### Scenario: The contention is exercised under real parallelism
- **WHEN** the concurrent contention scenario runs
- **THEN** it runs under genuine multi-threaded parallelism, because the reference backend's ready
  futures do not interleave on a single-threaded executor

