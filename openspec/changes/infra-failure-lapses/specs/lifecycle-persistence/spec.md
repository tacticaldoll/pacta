## MODIFIED Requirements

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
