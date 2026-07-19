## Why

The workspace carries two near-duplicate `examples/compose.rs` targets (one on
`pacta`, one on `pacta-driver`) whose composition demonstration is already carried
— more strongly — by the facade's runnable `lib.rs` doctest: the doctest runs and
asserts under `cargo test`, renders on docs.rs, and imports only from `pacta`,
whereas an `examples/` target is compile-only and is not rendered to a consumer.
Shipping the examples in the published tarball adds weight a consumer never reads;
excluding them would leave them serving only CI. Either way they no longer earn
their keep once the doctest is the proof.

## What Changes

- Delete `crates/pacta/examples/compose.rs` and
  `crates/pacta-driver/examples/compose.rs`.
- Remove the now-unused `uuid` dev-dependency from `pacta` (the facade doctest
  uses `Default::default()`, not `uuid`). Keep `pacta-driver`'s `uuid` dev-dependency
  — its `#[cfg(test)]` unit tests use it and already drive `claim -> execute ->
  settle` through the public API.
- Retire the `composition-example` capability: every one of its requirements is
  phrased about the deleted example, so it has no remaining subject.
- Fold the still-live composition guarantees into `public-facade`, re-homed onto
  the facade doctest as the single composition proof: it drives a fulfilled
  lifecycle, imports only from the facade, wraps a pass-through middleware carrying
  no orchestration, and keeps its registry a pure lifecycle state machine.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `public-facade`: the facade's end-to-end composition demonstration is a runnable
  doctest rather than a standalone `examples/` target; the doctest carries the
  fulfilled-lifecycle, facade-only-imports, pass-through-middleware, and
  pure-registry guarantees. The requirement that a separate `pacta-driver`
  composition example be preserved is dropped.
- `composition-example`: **REMOVED** — its subject (the `pacta-driver` example)
  is deleted; its dependency guarantee stays enforced by the tianheng constitution
  and its purity/no-orchestration axioms stay in the core specs.

## Impact

- Removes two `examples/` build targets and one dev-dependency; no change to any
  library crate's public API or runtime behavior.
- Thins the published tarball (examples no longer shipped) and the workspace.
- Updates `openspec/specs/public-facade`, removes
  `openspec/specs/composition-example`, and syncs `BACKLOG.md` with the recorded
  reconsideration.
- The composition demonstration and its constraints survive as the facade doctest;
  the advanced/core composition survives as `pacta-driver`'s unit tests.
