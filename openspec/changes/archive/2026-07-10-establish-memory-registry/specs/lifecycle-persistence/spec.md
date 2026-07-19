## ADDED Requirements

### Requirement: Injected Time Is A Call Parameter
Time-dependent registry operations SHALL accept the current time as a call
parameter, and settlement operations SHALL NOT, so the seam that injects time is
explicit and no operation reads an ambient clock.

#### Scenario: Time-dependent operations accept time
- **WHEN** a registry claims a pact or heartbeats a claim
- **THEN** the operation accepts the current time as a `Timestamp` parameter and
  decides lease expiry from it

#### Scenario: Settlement operations take no time
- **WHEN** a registry fulfills or breaches a claim
- **THEN** the operation takes no time parameter and authorizes the settlement by
  matching the presented retainer against the current holder

### Requirement: Lapse Rotates Authority
Reclaiming a pact whose lease expired without settlement SHALL rotate its
authority, so the prior holder's retainer no longer settles it.

#### Scenario: Reclaiming a lapsed pact rotates the retainer
- **WHEN** a pact's lease expires and it is claimed again
- **THEN** the new claim carries a different retainer than the lapsed claim

#### Scenario: The prior holder cannot settle after a reclaim
- **WHEN** a lapsed pact has been reclaimed and the original holder settles with
  its prior retainer
- **THEN** the registry rejects the settlement because the retainer is no longer
  the current holder

### Requirement: Heartbeat Does Not Revive A Lapsed Lease
A heartbeat presented after its lease has already expired SHALL be rejected, so a
lapsed holder must re-claim rather than revive and two holders never both hold
valid settlement authority. This is a settlement-authority guarantee, not a
concurrency guarantee: a lapsed holder may still be executing at-least-once, which
is why the user's `Executor` must be idempotent.

#### Scenario: Heartbeat after expiry is rejected
- **WHEN** a holder heartbeats a claim whose lease has already expired
- **THEN** the registry rejects the heartbeat and the holder must claim again to
  continue
