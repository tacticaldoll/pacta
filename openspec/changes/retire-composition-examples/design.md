## Context

This is a polish step before the paused 0.1.0 publishes. Two `examples/compose.rs`
targets (on `pacta` and `pacta-driver`) demonstrate composition, but the facade
`lib.rs` already carries a runnable "Composing the lifecycle" doctest that does the
same thing more strongly — it runs and asserts under `cargo test`, renders on
docs.rs, and imports only from `pacta`. The examples ship in the published tarball
(neither crate sets `exclude`), where docs.rs does not render them and a consumer
never reads them. The decision (option A of an explore discussion) is to retire the
examples and let the doctest be the single composition proof.

## Goals / Non-Goals

**Goals:**
- Remove the redundant example artifacts and the dev-dependency only they needed.
- Preserve every guarantee the examples encoded, re-homed onto the facade doctest
  (which already embodies them) or onto existing enforcement.
- Thin the published tarball and the spec surface.

**Non-Goals:**
- No change to any library crate's public API or runtime behavior.
- No new example crate or workspace member (that would thicken the workspace and
  pull it under governance).
- No weakening of the composability guarantee, registry purity, or the
  no-orchestration / no-core-dependency constraints.

## Decisions

### The facade doctest is the composition proof
The facade doctest already builds a pure `Registry` (`Ledger`, implementing only
the lifecycle operations), wraps a pass-through `Middleware` (`Witness`), and drives
one step to `Step::Fulfilled`, importing only from `pacta`. It is therefore a strict
superset of the deleted `pacta/examples/compose.rs` (which was compile-only): the
doctest runs, asserts, ships, and renders. No new doctest content is required.

### The advanced/core composition stays proven by pacta-driver unit tests
`pacta-driver`'s `#[cfg(test)]` unit tests already construct a `Claim`, drive
`Driver::step`, and reach `Step::Fulfilled` through the crates' public APIs, using
`uuid` for identifiers. They keep the core-crate composition proven, so deleting
`pacta-driver/examples/compose.rs` loses no coverage and `pacta-driver` keeps its
`uuid` dev-dependency. `pacta`'s `uuid` dev-dependency, used only by the deleted
example, is removed.

### Retire the `composition-example` capability; fold guarantees into `public-facade`
Every requirement in the `composition-example` capability is phrased about the
deleted example, so the capability has no remaining subject. Rather than rewrite six
requirements around the doctest — duplicating what `public-facade` already governs —
the capability is REMOVED and its still-live guarantees (pass-through middleware,
registry purity) are folded as scenarios onto the facade doctest requirement in
`public-facade`. The dependency guarantee stays enforced by the tianheng
constitution; the purity/no-orchestration axioms stay in the core specs and BACKLOG.

- Alternative considered: keep the examples but `exclude = ["examples/"]` from
  publish. Rejected — it leaves a ~90% duplicated pair serving only CI, whose
  audience (tarball consumers) never saw them anyway; the doctest already proves
  composition and is what consumers read.
- Alternative considered: rewrite `composition-example` around the doctest.
  Rejected — it would duplicate `public-facade`'s governance of the same doctest.

## Risks / Trade-offs

- [A guarantee silently lost when removing the capability] → Each removed
  requirement's guarantee is explicitly re-homed: pass-through and purity become
  facade-doctest scenarios; no-core-dependency stays in the tianheng constitution;
  the fulfilled-lifecycle proof is the doctest itself. The adversarial review checks
  nothing enforced-only-here is dropped.
- [Losing a runnable `cargo run --example` playground] → Accepted. The repo keeps
  the doctest (runnable via `cargo test`) and pacta-driver's unit tests; the
  playground served repo visitors, not consumers, and duplicated the doctest.
- [Facade governance false-positive after edits] → The facade reexports-only scan
  treats `//!` doctest content as comments; removing an example file does not touch
  the facade library body. DoD governance run confirms clean.

## Migration Plan

No consumer migration; no public API change. Delete the two example files, drop
`pacta`'s `uuid` dev-dependency, apply the spec deltas, sync BACKLOG, run the full
DoD gate, and land on `release/0.1.0` via the integration ritual.

## Open Questions

None.
