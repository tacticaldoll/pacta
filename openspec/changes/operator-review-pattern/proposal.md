## Why

`Tribunal` is reserved vocabulary (`docs/domain-language.md`: "terminal review for
exhausted pacts that should no longer be handled automatically") with no functional
definition anywhere. `BACKLOG.md`'s Candidate Pattern Areas lists "Operator Review"
(Tribunal inspection, manual review flows for exhausted pacts, minimal operational
visibility) as still-undesigned. Re-examining the prior Recorded Reconsiderations
entry that declined a similar-looking registry capability (by-id status
classification) confirms that decision is sound on its own terms — a
registry-contract operation is warranted only when it changes claim-authority
behavior, and inspection/listing never does — but that decision was never actually
about Operator Review. `docs/blueprint.md`'s Extension Surfaces categorizes
"operator review semantics" under **User-Defined Obligation** and "operator tools"
under **Integration Boundary**, never under Lifecycle Persistence — so this pattern
was always meant to live on top of the already-shipped `Policy`/`Middleware`
composition seam, not inside `Registry`.

Concretely: `Policy::decide` already returns `Verdict::Concede` at the exact moment
a pact becomes "exhausted" (settled as a terminal breach after repeated
infrastructure failures). Operator Review is just: what a consumer's `Policy` or
`Middleware` implementation does with that moment — record it somewhere an operator
can inspect. No `Registry` or backend capability is needed.

`openspec/specs/composition-governance/spec.md` already carries a structurally
identical requirement, "Durable Retry Is Demonstrated" (its executable proof is
`crates/pacta-memory/examples/durable_retry.rs`): a demonstrated pattern proving
composability over the shipped contract, with no new core capability. Operator
Review fits the same shape and the same spec, not a new capability file.

## What Changes

- Add a new requirement, "Terminal Review Is Demonstrated", to
  `composition-governance`, alongside the existing "Durable Retry Is Demonstrated" —
  stating that Pacta carries an executable demonstration of the Tribunal/operator-
  review pattern: a `Policy` (or `Middleware`) that records an exhausted pact
  (`Verdict::Concede`) somewhere an operator can inspect, using only the public API,
  self-checking so it cannot silently regress, adding no `Registry` capability.
- Extend `pacta-executor`'s existing `#[cfg(test)]`-only `Policy`/`Middleware`
  reference fixture (`FixedThreshold`/`GiveUp`) with a demonstration: recording an
  exhausted pact's id into an inspectable log when `Verdict::Concede` fires. A
  `#[test]`, not a standalone `examples/*.rs` binary — verified in the design phase
  that `cargo test --workspace` does not actually execute `durable_retry.rs`'s
  `main()` today, so a `#[test]` is the only shape this workspace's Definition of
  Done actually runs.
- Document the pattern in `pacta`'s facade crate documentation (where the
  `Policy`/`Verdict` paragraph already lives): terminal review composes at the
  `Verdict::Concede` decision point, fully consumer-owned.
- Update `BACKLOG.md`'s Operator Review candidate entry to point at the now-shipped
  pattern rather than describe it as undesigned.

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `composition-governance`: adds "Terminal Review Is Demonstrated", a new
  requirement alongside the existing "Durable Retry Is Demonstrated", for the same
  reason and in the same shape — a demonstrated pattern, not a new core capability.

## Impact

- `openspec/specs/composition-governance/spec.md`: new requirement.
- `crates/pacta-executor/src/lib.rs`: extends the existing `#[cfg(test)]` reference
  fixture (no public API change).
- `crates/pacta/src/lib.rs`: new documentation paragraph.
- `BACKLOG.md`: Operator Review candidate entry updated.
- No `Registry`, `pacta-contract`, or `pacta-memory` change. No CI/workflow change.

## Separately Noted (Not In Scope Here)

While researching `durable_retry.rs` as precedent, found that `cargo test
--workspace --all-features` does not execute its `main()` — the "Durable Retry Is
Demonstrated" requirement's "self-check... under the Definition of Done" claim
appears unenforced today (confirmed empirically: no output, no assertion runs).
This is a pre-existing gap unrelated to Operator Review; not fixed in this change,
flagged for a separate one.
