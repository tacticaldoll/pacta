## ADDED Requirements

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
before settling is recovered and may be claimed again. The user's `Executor`
therefore MUST be idempotent. Exactly-once delivery SHALL NOT be promised as a
core guarantee.

#### Scenario: A recovered pact may execute more than once
- **WHEN** a holder executes a pact but its lease lapses before settlement and the
  pact is claimed again
- **THEN** the pact may be executed a second time, which is correct at-least-once
  behavior rather than an error

#### Scenario: User execution carries the idempotency obligation
- **WHEN** documentation or specs describe a user's `Executor`
- **THEN** they state that the executor must be idempotent because Pacta
  guarantees at-least-once, not exactly-once, recovery

#### Scenario: Exactly-once is not claimed
- **WHEN** Pacta describes its recovery guarantee
- **THEN** it does not claim exactly-once delivery, which remains deferred

### Requirement: Mechanism Not Policy
The registry SHALL own only the lease-expiry and lapse mechanism. It SHALL NOT own
retry, backoff, attempt limits, or Tribunal routing, which stay user-owned through
middleware or explicitly deferred.

#### Scenario: The registry computes no retry or backoff
- **WHEN** a pact lapses
- **THEN** the registry neither schedules a delayed re-attempt nor decides a
  backoff interval

#### Scenario: Attempt limits and Tribunal routing stay outside the registry
- **WHEN** a pact has lapsed one or more times
- **THEN** deciding an attempt ceiling or moving the pact to Tribunal is not a
  registry responsibility and is left to user-owned policy or a later change

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
