## Context

The lifecycle contract is Pacta's whole product, and it is already governed — but
internally. The implementer half is executably manifest as `pacta-conformance`; the
user-obligation half (idempotent executor, lease sizing, heartbeat cadence) lives in
`lifecycle-persistence` / `domain-language` specs that a crates.io consumer never
reads. The reference status of `Driver`/`pacta-memory` is nowhere explicit. This
change projects the governed contract onto the consumer surface; it re-specifies
nothing.

## Goals / Non-Goals

**Goals:**
- Make both halves of the contract legible to whoever imports `pacta`.
- Anchor to executable proof: conformance (implementer half) + a compose doctest.
- Name reference pieces as reference, with `Driver`'s applicability boundary.
- Record the kernel-does-not-model-heartbeat finding as an explicit fork.

**Non-Goals:**
- No API change, no new behavior, no new dependency.
- No re-statement of obligations already in the specs (project, don't duplicate).
- No new spec surface for the obligations themselves (they are already governed).
- Not building the kernel `Heartbeat` directive (L3) — record only.

## Decisions

### Decision: Project into the facade rustdoc, not a CONTRACT.md
The `pacta` facade is the front door and renders on docs.rs automatically, so the
projection is seen where the consumer already looks and cannot drift out of the
crate. A standalone `CONTRACT.md` would drift from code and be invisible on docs.rs.

### Decision: Manifestation is a distinct capability, not a spec edit
`contract-manifestation` governs *projection onto the consumer surface* — a concern
distinct from the specs that *state* the obligations (`lifecycle-persistence`,
`domain-language`, `product-positioning`). It references them; it does not restate
them. This keeps the new capability from being duplication ceremony.

### Decision: A doctest, not only the example binary
The `compose.rs` example proves composition at build time; promoting an equivalent
into a `pacta` crate-root doctest makes the composition contract part of the
documented surface AND compiler-checked by `cargo test`. The example binaries stay
(they carry the governance guarantees already speced). The doctest must import only
from `pacta` and stay within the re-exports-only and `must_not_expose(kernel)`
constraints — a doctest is neither a library item nor a re-export, so both governance
scans are unaffected.

### Decision: L3 (kernel models heartbeat) is recorded, not built
The pure kernel's directives are `Claim | Execute | Settle | Idle` — no `Heartbeat`.
The sans-I/O-pure manifestation would add a `Heartbeat` directive the runtime
performs (kernel decides *that* it may heartbeat; runtime decides *when*), keeping
cadence out of the kernel. But a synchronous blocking `Executor` cannot yield to be
heartbeated mid-execution, so this pulls at the sync/async seam and is a future line.
Recording it converts the earlier heartbeat-seam finding from a silent gap into a
named fork.

## Risks / Trade-offs

- [Rustdoc projection drifts from the specs it references] → It references governed
  truth (conformance, the obligation specs) rather than restating it; the compose
  doctest is compiler-checked, so the composition half cannot silently rot.
- [Doctest trips a governance scan] → It is a doc example, not a library item or a
  re-export; the re-exports-only scan and `must_not_expose` boundary observe items,
  not doc comments. Verified at apply time by the DoD `check`.
- [Over-claiming "manifest"] → The user-obligation half stays partly prose
  (idempotency is semantic, not type-enforceable). The change is honest that
  conformance manifests the implementer half executably while the obligation half is
  projected, not enforced.

## Migration Plan

Docs + one doctest; no code or API change. Rollback is reverting the rustdoc and the
doctest.

## Open Questions

- Whether L3 (a kernel `Heartbeat` directive + a non-blocking execution shape) is
  worth a dedicated explore later. Recorded in BACKLOG; not decided here.
