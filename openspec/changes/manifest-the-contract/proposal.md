## Why

Pacta's entire product is a lifecycle contract — "薄到只剩 lifecycle 契約". That
contract is already fully governed, but only *internally* (`openspec/specs/`), which
a crates.io consumer never reads. The contract has two halves, unevenly manifested:
the **implementer half** (what a `Registry` must do) is executably manifest as
`pacta-conformance`; the **user-obligation half** (idempotent executor, lease
sizing, heartbeat responsibility) is real and governed but invisible on the
consumer-facing surface. If the contract is the product, not manifesting it to the
person importing `pacta` is the real omission — and a warning label is the wrong
fix. This change *projects* the already-governed contract onto the surface the
consumer sees, anchored to executable proof where possible. It does not
re-specify the contract; it makes it legible.

## What Changes

- **Project the contract onto the facade's documented surface.** The `pacta`
  crate-root rustdoc states both halves: what a `Registry` must satisfy (and that
  `pacta-conformance` is the executable proof of it), and what the consumer owes —
  an idempotent `Executor` (at-least-once, not exactly-once), user-owned lease
  sizing, and runtime-owned heartbeat cadence.
- **Name the reference pieces as reference.** `Driver` and `pacta-memory` are
  documented as reference skeleton / reference backend, and `Driver`'s applicability
  boundary is stated: it drives synchronously and does not heartbeat in-flight, so it
  is safe for tasks shorter than the lease and for single-worker use; long or
  multi-worker durable workloads compose their own loop over the `Registry` contract.
- **Anchor the composition contract in a compiler-checked doctest** at the facade
  crate root, so "claim → execute → settle composes through the public surface" is
  verified by `cargo test`, not only asserted by an example binary.
- **Record the deep finding (L3) as an explicit design fork**, not a silent gap: the
  pure lifecycle kernel does not model heartbeat as a lifecycle event (its directives
  are `Claim | Execute | Settle | Idle`; there is no `Heartbeat` directive), so
  in-flight heartbeat is unmodeled by design. Manifesting it sans-I/O-purely (a
  kernel `Heartbeat` directive the runtime performs) collides with the synchronous
  `Executor` and is deferred to a future line.

## Capabilities

### New Capabilities
- `contract-manifestation`: Pacta SHALL project its lifecycle contract — both the
  implementer half and the user-obligation half — onto the consumer-facing surface,
  anchored to executable proof where possible, and SHALL name its reference pieces as
  reference with their applicability boundaries.

## Impact

- `crates/pacta/src/lib.rs`: expanded crate-root rustdoc (the contract projection)
  and a crate-root composition doctest. No API change; the facade stays re-exports
  only, so the re-exports-only governance scan and the `must_not_expose(kernel)`
  boundary are unaffected (doc + doctest are not items).
- `crates/pacta-driver/src/lib.rs`, `crates/pacta-memory/src/lib.rs`: rustdoc naming
  them reference-only, with `Driver`'s applicability boundary.
- `BACKLOG.md`: record the kernel-does-not-model-heartbeat design fork under Recorded
  Reconsiderations.
- No behavior, no dependency, no published-API change. Governance-only surfaces
  (conformance, kernel) are referenced, not modified.
