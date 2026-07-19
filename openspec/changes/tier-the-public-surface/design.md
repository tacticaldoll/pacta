## Context

`manifest-the-contract` made the curated (facade) surface legible. But the
commitment surface is the whole published set, not the facade — and one committed
face, `pacta_contract::kernel`, is public for a structural reason: `pacta-driver`
consumes it across the crate boundary, and Rust has no cross-crate "friend"
visibility, so it must be `pub`. It is therefore committed whether or not we intended
it as a user feature. This change makes that commitment honest.

## Goals / Non-Goals

**Goals:**
- Declare stability tiers so the intended stability of each face is legible.
- Manifest the advanced tier (the kernel-driving contract) and prove it with a
  doctest.
- Thread the backend-author two-crate seam.

**Non-Goals:**
- No API/behavior/dependency change.
- Not un-committing the kernel (feature-gate / doc-hidden) — recorded as a 1.0 fork.
- Not promising the kernel is stable — the point is the opposite: name it advanced.

## Decisions

### Decision: Embrace + tier, rather than hide or feature-gate
The kernel cannot be cheaply un-committed: merging driver into contract violates the
governed isolated-core split; `pub(crate)` is impossible (cross-crate use);
`#[doc(hidden)]` leaves it committed while pretending otherwise; feature-gating adds
flag complexity for near-zero benefit at 0.1.x (the driver pulls it into the graph
regardless, and SemVer already signals instability). The honest response to a
structurally-committed surface is to state what it is and what it costs, and to make
it actually usable — so embrace it as an advanced extension point and tier it.

### Decision: Tiers are intent, and the boundary is already half-governed
At 0.1.x SemVer holds everything unstable, so a "tier" is a statement of intent
(which faces are converging into long-term contracts) — the same posture tianheng
takes for itself. It need not be fully executable. Notably the Tier-1/Tier-2
boundary is already partly enforced: `must_not_expose("pacta_contract::kernel")`
keeps the kernel out of the Tier-1 facade, so the tiering is not pure prose.

### Decision: Manifest the driving contract with a doctest in `pacta-contract`
The kernel already has conceptual rustdoc but not a "how to drive me" protocol
statement. Add it plus a doctest that runs the `poll → perform → on_event → result`
loop for one step. The doctest lives in a doc comment (not an item and not a
re-export), and it reads no clock (it uses `Timestamp::from_millis`), so it does not
interact with the ambient-time `must_not_call_inline` rule, the async-exposure
boundary, or the facade scans. It is constructible entirely from `pacta-contract`
public items.

### Decision: `surface-tiers` is its own capability
Stability tiering is a distinct axis from `contract-manifestation` (what the contract
IS). Keeping it separate avoids overloading that capability, and it references the
existing kernel-exclusion governance rather than restating it.

## Risks / Trade-offs

- [Manifesting the kernel raises the support burden] → Mitigated by the explicit
  advanced tier: it is documented AND labeled lower-stability (API may evolve; still
  supported/governed), so
  legibility does not imply a stability promise.
- [A doctest in the core could trip a governance rule] → It is doc-comment content,
  reads no clock, exposes no async; verified at apply time by the DoD `check`.
- [Tiers read as vague at 0.1.x] → Anchored to the governed kernel-exclusion boundary
  (Tier 2 is provably out of Tier 1) and to SemVer honesty, not marketing.

## Migration Plan

Docs + one doctest + BACKLOG note; no code or API change. Rollback is reverting the
rustdoc and the doctest.

## Open Questions

- Whether to eventually narrow the kernel's exposure (feature-gate) as Pacta
  approaches 1.0. Recorded in BACKLOG; not decided here.
