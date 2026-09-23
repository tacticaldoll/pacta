# Spec Delta

## MODIFIED Requirements

### Requirement: The Contention Harness Is Proven Non-Vacuous
The suite SHALL prove that its contention checks can catch a non-atomic backend, so a contention
gate cannot be vacuous and read as coverage it does not provide. Against an arbitrary backend the
contention checks repeat the contended settlement and the contended claim for a fixed number of
rounds on real concurrent workers; that repetition is a probabilistic stress and does not
guarantee that any given non-atomic backend is caught on any run. The harness's teeth are proven
only by the deterministic fixtures below. A deterministic, barrier-synchronized
non-atomic fixture — whose contended operation loads the state, waits until both contending workers
have loaded the same pre-state, then stores, so a double application is forced rather than left to
chance — SHALL make the corresponding contention check fail. A matching atomic fixture SHALL pass.
Both the settlement-contention and the claim-contention branches SHALL be covered by such a guard.
This mirrors the project's "reactions are proven to fire" discipline for governance.

#### Scenario: A non-atomic apply fails the settlement-contention check
- **WHEN** the settlement-contention check runs against a deterministically non-atomic `apply` fixture that lets both workers observe the same pre-state before either stores
- **THEN** the check fails, because the fixture double-applies the transition and violates at-most-once

#### Scenario: A non-atomic claim fails the claim-contention check
- **WHEN** the claim-contention check runs against a deterministically non-atomic `claim` fixture that lets both workers observe the pact available before either marks it held
- **THEN** the check fails, because the fixture issues two claims for the same pact

#### Scenario: A matching atomic fixture passes
- **WHEN** the contention checks run against a matching fully atomic fixture
- **THEN** they pass, so each guard distinguishes a harness with teeth from one that always passes

#### Scenario: The guard is deterministic
- **WHEN** the non-vacuity guard forces the interleaving
- **THEN** it uses a barrier so both workers load the same pre-state before either stores, making the double application deterministic rather than reliant on a lucky interleaving
