## Why

Two README improvements, now unblocked. First: with `infra-failure-lapses` shipped,
Pacta's central design pattern — *the core owns the lifecycle mechanism and no
policy; you compose the policy at the seams* — is fully shipped-true (the kernel now
fabricates no outcome and owns no disposition). The README states this only in
prose; a compact diagram makes the "what Pacta owns vs what you compose" split
legible where consumers look. Second: the root README's License section diverges
from the family's canonical `rust-openspec-starter` pattern (a single line); align
it so every project reads the same.

## What Changes

- Add a compact "what Pacta owns vs what you compose" diagram to the root README,
  visualizing the mechanism-vs-policy split (Pacta owns: lifecycle contract +
  sans-I/O kernel + lease/lapse + the reference `Driver` that decides no outcome; you
  compose: `Registry`, `Executor`, `Middleware`).
- Simplify the root README License section to the starter's one-line form,
  dropping the bullet list and the separate `### Contribution` subsection.
- **Deliberately pacta-local and sibling-blind:** the diagram shows *pacta's* shape,
  not the family-level "pattern-as-product" thesis (that names the family's shape and
  stays in the private planner roadmap, per the sibling-blind rule). No sibling is
  named; the general thin-vs-heavy point stays as the existing "Why Pacta" prose.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `product-positioning`: ADD "The Composition Pattern Is Documented" — Pacta must
  document, where consumers look, the mechanism-vs-policy split (core owns the
  lifecycle mechanism and no policy; the consumer composes policy at the
  `Registry`/`Executor`/`Middleware` seams; the reference `Driver` is shipped
  mechanism), depicting only shipped behavior and naming no sibling. This makes the
  pattern-documentation a review-governed surface rather than incidental prose. (The
  license one-liner is a docs cleanup with no requirement change.)

## Impact

- `README.md` only. No source code, no spec requirement, no manifest/version change.
- The root README is the GitHub landing page (no crate inherits it since
  `add-crate-readmes-and-license`), so this does not touch any crates.io page.
