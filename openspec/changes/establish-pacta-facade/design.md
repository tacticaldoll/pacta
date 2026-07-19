## Context

Pacta publishes to crates.io at 0.1.0. Today the public API lives in three crates
and the compose-vs-advanced line (kernel) is enforced only by omission. An earlier
proposal to add this facade was audited as DRIFT — its blocking objection was that
under `publish = false` a `cargo add pacta` front door buys nothing, and that the
facade is a composer's editorial concern the thin workspace pushes outside itself.
The publish decision removes the blocking objection; this design addresses the
audit's surviving concerns directly rather than re-litigating them.

## Goals / Non-Goals

**Goals:**
- A single curated `cargo add pacta` entrypoint re-exporting the compose-level API.
- Make the curated-vs-advanced (kernel) split executable, not a doc promise.
- Keep the facade a pure re-export surface, enforced, so it cannot accrete logic.
- Preserve the existing core-composition proof unchanged.

**Non-Goals:**
- No logic, helpers, builders, or convenience wrappers in the facade.
- No re-export of the sans-I/O kernel through the facade.
- No change to any core crate's code or public API.
- No new governance dependency; no adapter/backend re-exports.

## Decisions

### Decision: A crate, not documentation
A `cargo public-api` snapshot or rustdoc grouping can describe "recommended", but
cannot give a published one-line entrypoint nor make the kernel exclusion a CI
failure. Since the workspace now publishes, the entrypoint is a publisher's
artifact. Alternative (docs only) was the audit's suggestion under `publish=false`;
it is dominated once publishing is real.

### Decision: Re-export set
From `pacta-contract`: `Pact`, `Claim`, `Retainer`, `Timestamp`, `Outcome`,
`Settlement`, `Registry`. From `pacta-executor`: `Executor`, `Execution`,
`Middleware`, `Policy`. From `pacta-driver`: `Driver`, `Step`, `DriverError`. This
is exactly the set an end-to-end composer needs (verified against the three crates'
public APIs and the existing example's imports). `Outcome` and its alias
`Settlement` are both re-exported because composition-governance names both as
vocabulary. `uuid` is NOT re-exported — a consumer constructing a `Pact` adds it
itself; keeping it out keeps the facade Pacta's own vocabulary.

### Decision: Kernel exclusion enforced by hunyi `must_not_expose`
`pacta-contract` is an allowed dependency of the facade and `kernel` is a public
module of it, so a dependency boundary cannot stop `pub use pacta_contract::kernel`.
hunyi's signature-coupling treats a `pub use` re-export (including whole-module and
glob) as exposure and resolves it through the crate's re-export closure, so
`SemanticBoundary::in_crate("pacta").module("crate").must_not_expose("pacta_contract::kernel")`
catches exactly that leak. This is genuinely non-redundant with the dependency
boundary and the kernel async-exposure boundary (which checks a different property).
`SemanticBoundary` is re-exported by `tianheng`, so no new governance dependency.

### Decision: Re-exports-only enforced by a source-scan reaction
To keep the entrypoint from accreting "just one convenience" (the batteries-magnet
risk), a governance reaction line-scans the facade's library and fails if it
declares any item other than a re-export. It must be a brace-depth-aware line/string
scan (accepting multi-line `pub use { … }` blocks), NOT a real parser: `pacta-governance`
is boundary-locked to `tianheng` only and cannot take a `syn` dependency without
violating its own boundary. This mirrors the existing ambient-time source scan in
`pacta-governance`. Alternative (trust + review) was rejected: the audit named
the magnet risk specifically, and the whole project discipline is teeth over
promises.

### Decision: Keep the core example, add a separate facade example
`crates/pacta-driver/examples/compose.rs` exists to prove the three raw core crates
compose through their own public APIs — a guarantee its spec mandates. Routing it
through `pacta::` would weaken that proof to "the facade's re-exports resolve".
Instead the core example stays byte-for-byte, and a new
`crates/pacta/examples/compose.rs` proves the facade surface is composition-complete
on its own. Two examples, two distinct guarantees.

### Decision: Record the stance reversal explicitly
BACKLOG and the "Workspace Governance Coverage" requirement previously enumerated
the thin workspace's members without a facade and said "a composer owns" curation.
That was written under `publish = false`. The delta amends the enumeration to
include the published entrypoint and states why owning it is a publisher concern,
so the reversal is legible, not a silent exception.

## Risks / Trade-offs

- [Facade drifts out of sync with core public API] → It is re-exports only and
  built/tested by the workspace gates; a removed core item breaks the facade build.
- [Two examples duplicate lifecycle wiring] → Accepted: they assert different
  guarantees (raw-crates composability vs facade composability); the duplication is
  the point, not incidental.
- [Re-exports-only scan is a heuristic] → It parses items and allows only
  `pub use` (plus attributes/docs); a false positive is a governance failure the
  author sees immediately, never a silent pass, matching the ambient-time scan's
  posture.
- [`pacta` name unavailable on crates.io] → Verified as a publish-time pre-flight
  (`cargo publish --dry-run`); out of scope for this change, gating for release.

## Migration Plan

Additive. New crate, new example, new governance boundaries and one spec delta. No
existing code or public API changes; rollback is deleting the crate and reverting
the governance additions.

## Open Questions

None blocking. This change is ordered AFTER `prepare-release-hygiene`, which flips
the workspace to `publish = true` and establishes shared publish metadata
(`repository`, `keywords`, `categories`). The facade's "publisher owns the
entrypoint" justification is only factually true once that predecessor has landed,
so the facade must not be implemented before it — otherwise this ships the audited
`publish = false` DRIFT and writes an aspirational (false) clause into the
`quality-governance` requirement.
