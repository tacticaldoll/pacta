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

### Requirement: Conformance Covers Deferred Re-Arm
The conformance suite SHALL verify release and deferred re-arm for any backend: that a released
pact is not claimable before its re-arm instant, is claimable at or after it, that immediate
re-arm behaves as a lapse, and that release rotates authority away from the prior retainer.

#### Scenario: A released pact is withheld until its re-arm instant
- **WHEN** the suite releases a claim with a future re-arm instant and claims again before it
- **THEN** the claim returns nothing

#### Scenario: A released pact is reclaimable at its re-arm instant
- **WHEN** the suite claims again at or after the re-arm instant
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: Immediate re-arm reclaims like a lapse
- **WHEN** the suite releases a claim with a re-arm instant at or before now and claims again
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: Release rotates authority away from the prior holder
- **WHEN** the suite releases a claim and the prior holder then settles with its retainer
- **THEN** the settlement is rejected

