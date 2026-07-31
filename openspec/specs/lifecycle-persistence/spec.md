# lifecycle-persistence Specification

## Purpose
Define Pacta's claim lease lifecycle: a bounded lease decided from injected time, lapse recovery through the normal claim path with a rotated retainer, a heartbeat that cannot revive an expired lease, and at-least-once recovery paired with the idempotent-`Executor` obligation — mechanism, never retry/backoff policy.
## Requirements
### Requirement: Claim Lease Model
A claimed pact SHALL be held under a lease: a bounded validity window during which
the holding claim's retainer is authoritative. Lease expiry SHALL be a
claim-lifecycle state, not an orchestration decision.

#### Scenario: A claim is held for a bounded window
- **WHEN** a registry issues a claim
- **THEN** the claim is valid only until its lease expires, after which the
  registry no longer treats the retainer as the authoritative holder

#### Scenario: Lease expiry is lifecycle, not orchestration
- **WHEN** a lease expires
- **THEN** the registry records only that the claim is no longer held, computing
  no retry count, no backoff delay, no priority, and no routing

### Requirement: Lapse Recovery
The registry SHALL support lapse: recovering a pact whose retainer expired without
settlement so that the pact becomes claimable again. Lapse SHALL be a recovery
mechanism only and SHALL NOT be a retry policy.

#### Scenario: An expired unsettled claim becomes claimable again
- **WHEN** a claim's lease expires before the pact is fulfilled or breached
- **THEN** the pact becomes available to be claimed again through the normal
  claim path

#### Scenario: A settled claim is never lapsed
- **WHEN** a pact has been fulfilled or breached
- **THEN** lease expiry does not make it claimable again, because the obligation
  is already concluded

#### Scenario: Lapse decides no retry policy
- **WHEN** a pact is lapsed
- **THEN** the registry only makes it claimable and decides nothing about whether,
  when, or how many times it will be re-attempted

### Requirement: Injected Time
Lease expiry SHALL be decided from time supplied to the registry. The core SHALL
NOT read an ambient wall clock to decide expiry, so that lease behavior is
deterministic and testable and the core commits to no time source.

#### Scenario: Expiry is decided from supplied time
- **WHEN** the registry evaluates whether a lease has expired
- **THEN** it compares the lease window against time provided to it rather than a
  clock it reads on its own

#### Scenario: Core reads no ambient time
- **WHEN** the core lifecycle contract is implemented
- **THEN** it takes the current time as an input at its seam rather than calling a
  wall-clock function, and the change that introduces time-taking code adds a
  governance check enforcing this

### Requirement: At-Least-Once Recovery And Idempotent Obligation
Pacta SHALL guarantee at-least-once claim recovery: a pact whose holder stops
before settling is recovered and may be claimed again. Recovery SHALL cover a
holder that stops silently and a holder that reports an infrastructure failure
alike — an infrastructure failure leaves the claim unsettled to be recovered
through lapse, not terminally breached. The user's `Executor` therefore MUST be
idempotent. Exactly-once delivery SHALL NOT be promised as a core guarantee.

#### Scenario: A recovered pact may execute more than once
- **WHEN** a holder executes a pact but its lease lapses before settlement and the
  pact is claimed again
- **THEN** the pact may be executed a second time, which is correct at-least-once
  behavior rather than an error

#### Scenario: An infrastructure failure is recovered, not breached
- **WHEN** execution fails with an infrastructure error rather than producing a
  business `Outcome`
- **THEN** the lifecycle settles nothing and leaves the claim unsettled, so its
  lease lapses and the pact is reclaimed, giving the same at-least-once recovery as
  a holder that stopped silently rather than terminally breaching the pact

#### Scenario: User execution carries the idempotency obligation
- **WHEN** documentation or specs describe a user's `Executor`
- **THEN** they state that the executor must be idempotent because Pacta
  guarantees at-least-once, not exactly-once, recovery

#### Scenario: Exactly-once is not claimed
- **WHEN** Pacta describes its recovery guarantee
- **THEN** it does not claim exactly-once delivery, which remains deferred

### Requirement: Mechanism Not Policy
The registry SHALL own only the lease-expiry, lapse, and deferred-reclaim mechanism. It SHALL
NOT own retry, backoff, attempt limits, or Tribunal routing, which stay user-owned through
middleware or explicitly deferred. Honoring a consumer-injected reclaimable instant is a
mechanism, not a policy: the registry stores and compares that instant exactly as it honors
injected `now`, and computes no interval of its own.

#### Scenario: The registry computes no retry or backoff
- **WHEN** a pact lapses or is released with a reclaimable instant
- **THEN** the registry neither computes a backoff interval nor decides whether or how many
  times the pact will be re-attempted; any reclaimable instant it honors was computed and
  supplied by the consumer

#### Scenario: A reclaimable instant is honored, not decided
- **WHEN** a consumer releases a claim with a reclaimable `Timestamp`
- **THEN** the registry only stores that instant and makes the pact claimable at or after it,
  the same way it honors injected `now`, deciding no delay of its own

#### Scenario: Attempt limits and Tribunal routing stay outside the registry
- **WHEN** a pact has lapsed or been released one or more times
- **THEN** deciding an attempt ceiling or moving the pact to Tribunal is not a registry
  responsibility and is left to user-owned policy or a later change

### Requirement: User-Owned Lease Inputs
Lease duration and heartbeat cadence SHALL be user- and deployment-owned inputs
rather than core constants, so the core supplies the mechanism and the user
supplies the policy values.

#### Scenario: Lease duration is a supplied input
- **WHEN** a lease window is established for a claim
- **THEN** its duration comes from a user- or deployment-supplied value rather
  than a constant fixed inside the core lifecycle contract

#### Scenario: Heartbeat cadence is runtime-owned
- **WHEN** a running holder extends its lease through heartbeat
- **THEN** how often it heartbeats is decided by the runtime that drives it, not
  by the core lifecycle contract

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

### Requirement: Deferred Reclaim On Release
The registry SHALL provide a non-terminal `release` operation that relinquishes a held claim
and makes the pact reclaimable again only at or after a consumer-supplied `Timestamp`. The
retainer of the current holder and that instant SHALL be call parameters; the instant SHALL
NOT be a field of `Pact`, so `Pact` continues to carry no delay. `release` SHALL be distinct
from `fulfill` and `breach`: it concludes no obligation and leaves the pact to be attempted
again. The registry SHALL compute no delay — it honors the injected instant exactly as it
honors injected `now`. On release the registry SHALL rotate authority so the prior retainer
can no longer settle or heartbeat, identical to a lapse.

#### Scenario: A released pact is not claimable before its reclaimable instant
- **WHEN** a holder releases a claim with a future reclaimable instant and a claim is attempted before that instant
- **THEN** the registry does not return the pact

#### Scenario: A released pact is claimable at or after its reclaimable instant
- **WHEN** a claim is attempted at or after a released pact's reclaimable instant
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: An immediate reclaimable instant is a voluntary lapse
- **WHEN** a holder releases a claim with a reclaimable instant at or before now
- **THEN** the pact is immediately claimable again, as if its lease had lapsed

#### Scenario: Release rotates authority
- **WHEN** a holder releases a claim and then settles or heartbeats with the prior retainer
- **THEN** the registry rejects it, because release rotated authority away from that retainer

#### Scenario: Release requires the current holder
- **WHEN** release is called with a retainer that is not the current holder
- **THEN** the registry rejects it, the same authority check as fulfill and breach

#### Scenario: A settled pact cannot be released
- **WHEN** release is attempted on a pact already fulfilled or breached
- **THEN** the registry rejects it, because a concluded obligation has no claim to relinquish


### Requirement: Lifecycle Semantics Are A Single Shared Pure Kernel
The pact lifecycle semantics SHALL be defined once as a pure, sans-I/O, colorless `lifecycle`
module in `pacta-contract` — the claim-eligibility predicate, the state transitions (claim,
heartbeat, settle, release), the current-holder authority check, and the lease arithmetic —
and every `Registry` backend SHALL compose over that module rather than re-implementing those
semantics. The module SHALL read no clock (time is an injected parameter), perform no I/O, and
mint no non-deterministic value (the retainer/fencing value is supplied by the backend and
passed in); storage and retainer minting remain the backend's. This makes the semantics
single-sourced across backends and across a future async binding, so they cannot drift; the
other lifecycle requirements in this spec are unchanged and are now realized by this shared
kernel.

#### Scenario: A backend composes over the shared kernel
- **WHEN** a `Registry` backend decides claim-eligibility or applies a lifecycle transition
- **THEN** it calls the shared `lifecycle` module rather than computing eligibility or the transition itself

#### Scenario: The kernel is pure
- **WHEN** the `lifecycle` module computes an eligibility verdict or a transition
- **THEN** it reads only its arguments — no clock, no I/O, no minting — with time and the retainer value injected by the caller

#### Scenario: Extraction preserves behavior
- **WHEN** the reference backend is refactored to compose over the shared kernel
- **THEN** it passes the identical `pacta-conformance` suite with no change to its observable behavior
