## ADDED Requirements

### Requirement: Lease Window Vocabulary
Pacta SHALL treat `Lease` as the canonical term for the bounded validity window
during which a claim's `Retainer` is authoritative, distinct from the retired
"lease token" name that `Retainer` superseded.

#### Scenario: Naming the claim validity window
- **WHEN** public APIs, specs, or user-facing documentation refer to the bounded
  window during which a claim's retainer stays authoritative
- **THEN** they use `Lease` for that window

#### Scenario: Lease window is not the legacy lease token
- **WHEN** documentation uses `Lease`
- **THEN** it names the validity window and not the proof token, which is named
  `Retainer` — the term that superseded the legacy "lease token"

### Requirement: Lifecycle Recovery Vocabulary
Pacta SHALL treat `Lapse` as the canonical term for lifecycle recovery of a pact
whose retainer expired without settlement, rather than requeue, redeliver, or
retry terminology.

#### Scenario: Naming lease recovery
- **WHEN** public APIs, specs, or user-facing documentation refer to recovering a
  pact whose retainer expired without settlement
- **THEN** they use `Lapse` rather than requeue, redeliver, or retry terminology

#### Scenario: Lapse is recovery, not retry policy
- **WHEN** documentation explains `Lapse`
- **THEN** it presents `Lapse` as making a pact claimable again and distinguishes
  it from retry, backoff, or attempt-limit policy, which are not registry behavior

### Requirement: Reciprocal Obligation Vocabulary
Pacta SHALL name the recovery contract as at-least-once recovery paired with an
idempotent user obligation, keeping the guarantee and the obligation as governed
terms rather than implicit assumptions.

#### Scenario: Naming the recovery guarantee
- **WHEN** documentation describes what Pacta guarantees when a holder stops
  before settlement
- **THEN** it names the guarantee at-least-once recovery and does not name it
  exactly-once delivery

#### Scenario: Naming the reciprocal user obligation
- **WHEN** documentation describes what the user owes in return for at-least-once
  recovery
- **THEN** it names an idempotent `Executor` as the user's reciprocal obligation
