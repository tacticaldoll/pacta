## Why

Because Pacta publishes each crate individually, its commitment surface is larger
than the curated `pacta` facade. In particular `pacta_contract::kernel`
(`Directive`/`Notice`/`StepResult`/`Kernel`) is a committed public API — not because
it was offered as a feature, but because `pacta-driver` consumes it across the crate
boundary and Rust has no cross-crate "friend" visibility, so it must be `pub`. The
`manifest-the-contract` change made the *curated* tier legible; the *advanced* tier
is committed yet implicit. Leaving "we committed to the kernel" unstated is the kind
of thing nobody notices until someone builds on it. This change makes the commitment
honest: it declares stability tiers over the public surface and manifests the
advanced tier's driving contract, so what we promise matches what we intend.

## What Changes

- **Declare stability tiers** over the public surface. Tier 1 (recommended,
  converging toward long-term contract): the `pacta` facade and the backend-author
  path (`Registry` + proving with `pacta-conformance`). Tier 2 (advanced — lower
  stability intent, its API may evolve — but a governed, supported core surface, not
  a throwaway): `pacta_contract::kernel`. The tiers are a statement of *intent* — at
  0.1.x SemVer already holds everything unstable — and the Tier-1/Tier-2 boundary is
  already partly governed: the existing `must_not_expose("pacta_contract::kernel")`
  reaction keeps the kernel out of the Tier-1 facade.
- **Manifest the advanced tier's driving contract.** Document the kernel's driving
  protocol (`poll` → perform the `Directive` → feed a `Notice` via `on_event` →
  until `result`) and add a doctest that drives a step manually, so "compose your own
  runtime over the kernel" is a proven, legible extension point rather than a leaked
  internal.
- **State the tier in the two places a consumer looks:** the `pacta` facade
  crate-root rustdoc (its "what is not here" section) and the `kernel` module itself.
- **Refine the backend-author pointer.** The facade rustdoc already names
  `pacta-conformance` as the proof (governed by `contract-manifestation`); this only
  refines that existing sentence to note it is a *dev-dependency* — the two-crate
  journey. No new requirement; it rides the shipped one.
- **Record the un-commit options under Recorded Reconsiderations** in BACKLOG:
  narrowing the kernel's *exposure* (e.g. feature-gating) as a 1.0-approach option —
  explicitly not deprecating the shipped, governed kernel, and not done now.

## Capabilities

### New Capabilities
- `surface-tiers`: Pacta SHALL declare stability tiers over its public surface and
  manifest the advanced tier's driving contract, so its commitment is explicit and
  its intended stability is legible.

## Impact

- `crates/pacta-contract/src/lib.rs`: kernel module rustdoc gains the driving-contract
  manifestation, a driving doctest, and an advanced-tier note.
- `crates/pacta/src/lib.rs`: facade rustdoc gains the tier statement and the
  backend-author seam pointer.
- `BACKLOG.md`: record the kernel-un-commit options as a 1.0-approach fork.
- No API, behavior, or dependency change. The kernel doctest lives in a doc comment
  (not an item), so no governance scan is affected.
