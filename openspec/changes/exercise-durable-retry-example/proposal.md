## Why

`composition-governance`'s "Durable Retry Is Demonstrated" requirement states the
example at `crates/pacta-memory/examples/durable_retry.rs` SHALL "self-check its
outcome so the demonstration cannot silently regress under the Definition of
Done," with a Scenario stating a regressed demonstration "fails the gate" when run
under the Definition of Done. Verified empirically (found while designing the
operator-review-pattern change): `cargo test --workspace --all-features` does not
execute this example's `main()` at all — no output, no assertion runs. Cargo
builds an example to catch compile errors but does not execute it under plain
`cargo test`, and this workspace's Definition of Done never adds a `cargo run
--example` step. The spec's claim is currently false in practice: the
demonstration could regress (an assertion could start failing, or be deleted
entirely) without the Definition of Done ever noticing.

## What Changes

- Make `durable_retry.rs`'s `main` function `pub` (a one-word change; does not
  affect running it via `cargo run --example durable_retry`).
- Add `crates/pacta-memory/tests/durable_retry.rs`: a thin integration test that
  includes the example via `#[path = "../examples/durable_retry.rs"]` and calls
  `durable_retry_example::main()` from a `#[test]`. Zero duplication — the example
  stays the single source of the demonstration; the test just makes `cargo test`
  actually run it, so a panicking assertion inside it fails the test.
- No change to the demonstration's own logic, assertions, or behavior — it already
  self-checks via `assert!`/`assert_eq!`; the fix is purely making the Definition
  of Done execute it, closing the gap between the spec's claim and reality.

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `composition-governance`: the "Durable Retry Is Demonstrated" requirement's "A
  regressed demonstration fails the gate" scenario is currently unenforced in
  practice; this change makes it true rather than aspirational. (No wording
  change needed in the requirement itself — the requirement was always correct in
  what it demanded; only the implementation was incomplete.)

## Impact

- `crates/pacta-memory/examples/durable_retry.rs`: `fn main()` becomes `pub fn
  main()`.
- `crates/pacta-memory/tests/durable_retry.rs`: new file (thin wrapper, no logic).
- No CI/workflow change — the existing `cargo test --workspace --all-features`
  (and `--no-default-features`, though this example needs no feature) already
  runs; it will now actually execute this demonstration for the first time.
