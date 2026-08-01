## Context

Re-examining `BACKLOG.md`'s Recorded Reconsiderations entry that declined a by-id
status classification registry method: the governing test ("a registry-contract
operation is warranted only when it must change the claim authority's behavior")
holds up under adversarial stress — no counter-argument found survives it, since
inspection/listing genuinely never changes claim-authority behavior, and
`pacta-conformance`'s purpose is proving claim-authority correctness, not query
consistency across backends.

But that decision was about a `Registry` capability. `docs/blueprint.md`'s Extension
Surfaces places "operator review semantics" under **User-Defined Obligation** and
"operator tools" under **Integration Boundary** — never under Lifecycle Persistence.
`BACKLOG.md`'s own Candidate Pattern Areas lists Operator Review separately from
Durable Backends, with "Surface: user-defined obligation or integration boundary."
So Operator Review was never blocked by the declined registry capability; it was
simply never designed.

The natural hook already exists: `Policy::decide(attempts, error) -> Verdict`
(shipped 0.3.0) returns `Verdict::Concede` at the exact moment a pact becomes
"exhausted" — the `GiveUpExecutor` reference in `pacta-executor`'s test module
already settles that as `Outcome::Breached`. Operator Review is composing a
recording step at that same point, entirely in consumer/`Policy`/`Middleware` code.

## Goals / Non-Goals

**Goals:**
- Prove, executably, that Tribunal/operator-review composes over the shipped
  `Policy`/`Middleware` seam with zero new core capability.
- Give `Tribunal`'s reserved vocabulary its first working referent, closing the
  "vocabulary-only, no definition" gap.
- Match the existing "Durable Retry Is Demonstrated" shape in
  `composition-governance` exactly, rather than inventing new spec structure.

**Non-Goals:**
- Does not add any `Registry`, `pacta-contract`, or `pacta-memory` capability —
  re-opening the by-id classification decision was considered and rejected on its
  own merits (see Context).
- Does not ship a concrete, production-usable `Tribunal` type or storage backend —
  that stays consumer- or sibling-owned, exactly as concrete orchestration
  middleware (retry, timeout, circuit) already does for Execution Composition.
- Does not fix the separately-found `durable_retry.rs` enforcement gap (see
  proposal.md's Separately Noted section) — out of scope here.

## Decisions

- **Extend `composition-governance`, not a new capability file.** It already
  carries the identical shape ("Durable Retry Is Demonstrated": an executable,
  self-checking demonstration of composability with no new core capability).
  Operator Review is the same kind of thing; splitting it into its own capability
  file would duplicate structure for no benefit.
- **A `#[test]`, not a new `examples/*.rs` binary.** `durable_retry.rs` was the
  obvious template, but checking whether it actually runs under `cargo test
  --workspace --all-features` found that it does not (no output, no assertion
  executes) — Cargo builds an example to catch compile errors but does not execute
  its `main()` under plain `cargo test`. Since this workspace's Definition of Done
  never adds a `cargo run --example` step, a `#[test]` inside the existing
  `pacta-executor` reference module is the only shape actually exercised — matching
  how `Policy` itself was validated ("an in-workspace `#[cfg(test)]`-only reference
  `Middleware` and tests").
- **Extend the existing `GiveUp`/`FixedThreshold`/`GiveUpExecutor` fixture rather
  than add a parallel one.** It already demonstrates the exact `Verdict::Concede`
  moment; adding a recording side-effect there (e.g., an `Rc<RefCell<Vec<Uuid>>>`
  "tribunal log," mirroring the `log: Rc<RefCell<Vec<String>>>` shape the
  middleware-ordering tests already use) is the smallest possible change that
  proves the pattern.
- **Document in `pacta`'s facade crate docs, next to `Policy`/`Verdict`.** That is
  where a consumer already reads about the infrastructure-failure disposition
  seam; Operator Review is the natural next paragraph, not a new doc file.

## Risks / Trade-offs

- [A test-only demonstration is not a public, reusable `Tribunal` type a consumer
  can just import] → intentional, matching the `Policy` reference precedent and
  the non-goal above; a real production implementation is consumer- or
  sibling-owned.
- [Someone might read "Terminal Review Is Demonstrated" and expect a shipped
  `Tribunal` API] → mitigated by stating the non-goal explicitly in the spec
  requirement text itself, the same way "Durable Retry Is Demonstrated" states
  "the core computes no backoff" to prevent the same misreading.

## Migration Plan

Purely additive: new spec requirement, an extended test fixture, a doc paragraph, a
`BACKLOG.md` update. No CI/workflow change — the existing `cargo test --workspace
--all-features` gate already exercises the new `#[test]`.

## Open Questions

None blocking. The `durable_retry.rs` enforcement gap is real but separate —
flagged in the proposal for a future change, not resolved here.
