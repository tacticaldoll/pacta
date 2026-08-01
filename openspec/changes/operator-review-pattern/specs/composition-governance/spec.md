## ADDED Requirements

### Requirement: Terminal Review Is Demonstrated
Pacta SHALL carry an executable demonstration of the Tribunal/operator-review
pattern: a `Policy` implementation that, upon deciding `Verdict::Concede` for an
exhausted pact, records that pact somewhere an operator can inspect, composed
entirely over the shipped `Policy`/`Middleware` seam. The demonstration SHALL use
only the public API and self-check its outcome so it cannot silently regress under
the Definition of Done. The core SHALL gain no `Registry`, `pacta-contract`, or
`pacta-memory` capability from this composition, and no concrete,
production-usable `Tribunal` type ships from this workspace.

#### Scenario: An exhausted pact is recorded for operator review
- **WHEN** the demonstration runs and a claimed pact's execution fails past the
  configured threshold, reaching `Verdict::Concede`
- **THEN** the pact's identity is recorded in an inspectable log before the claim
  settles as `Outcome::Breached`

#### Scenario: The demonstration is exercised by the Definition of Done
- **WHEN** `cargo test --workspace --all-features` runs
- **THEN** the demonstration's assertions execute and fail the gate if the
  recording behavior regresses

#### Scenario: No registry capability is added
- **WHEN** the demonstration is implemented
- **THEN** it composes only `Policy` and `Middleware` from the existing public API,
  and neither `Registry`, `pacta-contract`, nor `pacta-memory` gains a new method
  or capability

#### Scenario: No concrete Tribunal type ships
- **WHEN** the demonstration is implemented
- **THEN** it is validated by an in-workspace reference implementation (which may
  be `#[cfg(test)]`-only scaffolding), and no publicly shippable, production-usable
  `Tribunal` type or storage backend ships from this workspace
