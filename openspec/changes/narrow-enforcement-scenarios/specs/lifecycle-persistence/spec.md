# Spec Delta

## MODIFIED Requirements

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
  governance check that rejects the inline clock calls it can observe, leaving a
  clock read outside them to review
