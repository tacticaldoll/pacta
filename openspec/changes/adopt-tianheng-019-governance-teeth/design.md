## Context

Tianheng 0.1.9 shipped the three capabilities Pacta's feedback requested. The
bump `0.1.8 -> 0.1.9` is already staged in the working tree; the check runs clean
and byte-identical because every new feature is opt-in. All work here lands in the
internal, unpublished `pacta-governance` crate (`publish = false`) — the
constitution in `crates/pacta-governance/src/main.rs`, its test module, and its
manifest. An independent adversarial fidelity audit ran before this change; its
verdicts (uuid-only `strict_external`, `TempWorkspace`-based teeth, and accepting
the direct `guibiao` dependency as normal pre-1.0 practice) are folded in below.

## Goals / Non-Goals

**Goals:**
- Close the fully-qualified ambient-time hole for the `uuid` boundary.
- Prove the two semantic reactions (kernel async-exposure, facade
  kernel-exclusion) actually fire, and stay precise on non-leaking source.
- Move workspace-coverage enforcement onto Tianheng's native projection, retiring
  a hand-rolled manifest parser.

**Non-Goals:**
- No change to the published consumer surface or any runtime behavior.
- No committed fixture crates.
- No new capability specs; no change to the contract/executor/driver/memory/
  conformance/facade crates.

## Decisions

### `strict_external` on the `uuid` boundary only
`must_not_call_inline` already catches a sysroot head (`std::time::…`) by default,
but resolves a fully-qualified *external* head (`uuid::Uuid::now_v7()` with no
`use uuid`) as local and lets it pass. `.strict_external()` resolves a bare head
matching a **declared dependency** to that crate, closing the hole.

- Applied to `uuid` only. `std` is not a declared dependency, so `.strict_external()`
  on the `std::time` boundary would change nothing — a misleading no-op. (Verified
  against `guibiao` source: the flag sets `self.external = true`, which only affects
  bare heads matching a declared dependency.)
- Alternative considered: `.strict_prefix_only()` — rejected, it forbids even
  naming the type (type annotations, constants), which is broader than the
  no-ambient-read intent and would false-positive on legitimate injected-time
  signatures.
- The single-segment over-reaction bound (a local binding or def-site named like
  the crate reading as a call) is near-zero here: a false positive requires the
  core to bind a local `uuid` and call `.now_v7()`/`.now_v1()` on it, or to define
  such a fn — itself the anti-pattern the boundary targets. The
  clean-workspace test catches any regression.

### Testable teeth via `TempWorkspace`, not committed fixtures
The semantic boundaries read source, so a reaction test needs a fixture with real
leaking source. The existing in-test `TempWorkspace` helper already writes crate
manifests + `src/lib.rs` to a temp dir torn down on `Drop`, and the static teeth
test (`unapproved_core_dependency_is_rejected`) already proves the pattern.

- Extend `write_package` (or add a source-aware variant) to emit leaking source,
  then assert `tianheng::check_all(constitution().semantic_boundaries(), &manifest)`
  returns `Outcome::Violations` for the leak and `Outcome::Clean` for a matching
  non-leaking fixture.
- Because `check_all` evaluates the whole bundle against the whole workspace in one
  pass, and a boundary whose target crate OR module is absent raises a
  **constitution error (exit 2)**, not a silent skip, every fixture handed to
  `check_all` MUST be a complete mini-workspace containing BOTH target crates —
  `pacta-contract` (with a resolvable `crate::kernel` module) and `pacta` (lib root
  present) — varying only the intended leak:
  - async-firing: `pacta-contract` has `pub mod kernel { pub async fn … }`; `pacta`
    present with no kernel re-export.
  - facade-firing: `pacta` has `pub use pacta_contract::kernel::X;`; `pacta-contract`
    present with a non-async `kernel` module exposing `X`.
  - clean/precision: both crates present, `pacta-contract::kernel` non-async, `pacta`
    with no kernel re-export → `Outcome::Clean`.
  The existing `unapproved_core_dependency_is_rejected` test already writes a full
  multi-crate workspace, so this is the established `TempWorkspace` pattern.
- Firing tests assert the specific violation identity (the offending target crate +
  rule) as the existing static teeth test does with `id.target`/`id.rule`, not a bare
  `Outcome::Violations`, so a test cannot pass because the *other* boundary in the
  bundle fired while the one under test silently did not.
- Alternative considered: committed fixture crates (the COOKBOOK's other option) —
  rejected. A member fixture breaks `cargo build` and is judged by the real
  constitution; a non-member fixture is dead, deliberately-broken committed source,
  a maintenance liability on a thin repo. `TempWorkspace` adds zero committed
  surface and cannot become a workspace member.
- Entry point: `check_all(&SemanticBoundaries, …)` runs the whole semantic bundle
  (`eval_all` evaluates both the `signature` and `async_exposure` capabilities in
  one pass), so a single call covers both fixtures. `check as check_semantic`
  (`hunyi::exposure::check`) is the signature-only capability and takes
  `&[SemanticBoundary]`, so it neither type-checks against `semantic_boundaries()`
  nor exercises the async-exposure boundary — do not use it here. `check_all` is
  `#[doc(hidden)] pub use` in `tianheng` (public, not in the prelude), so call it
  fully-qualified as `tianheng::check_all`. No new dependency for this item.

### Native coverage via `guibiao::check_and_cover`
Replace `every_workspace_crate_has_a_boundary` and its `workspace_members` string
parser with `guibiao::check_and_cover(constitution().static_boundaries(), &manifest)`,
asserting the returned `Coverage { total, uncovered }` has `total > 0` (guarding a
vacuous pass) and `uncovered` empty. `coverage_from` counts a crate covered by a
crate boundary or a module boundary within it; all seven members carry a
`CrateBoundary`, so `total == 7` and `uncovered` is empty.

- `check_and_cover` takes `&guibiao::Constitution`, which `tianheng` re-exports as
  `GnomonConstitution` and returns from `Constitution::static_boundaries()` — NOT
  the unified `tianheng::Constitution` wrapper that `constitution()` returns. So the
  call must pass `constitution().static_boundaries()` (the same form the existing
  tests use for `check`, and the form `tianheng`'s own runner uses for
  `check_and_cover`). Passing `constitution()` directly is a type error regardless
  of version.
- `check_and_cover` is not re-exported by `tianheng`, so `guibiao` becomes a direct
  dependency of `pacta-governance`, declared as a workspace dependency pinned to
  the same version as `tianheng` (0.1.9) so the graph carries one `guibiao` and no
  duplicate-version drift.
- Widen `pacta-governance`'s own dependency boundary from
  `restrict_dependencies_to(["tianheng"])` to `(["tianheng", "guibiao"])`, and
  rewrite `GOVERNANCE_REASON` to state the real invariant: the governance gate
  stays independent of the *workspace graph it judges*; `tianheng` and its
  `guibiao` coverage core are governance-family tooling outside that graph.
- Rationale (audit-accepted): a pre-1.0 sub-crate dependency is normal SemVer
  practice — `tianheng` itself is 0.1.x and equally subject to change; caret
  `0.1.x` will not auto-jump to `0.2.x`; `guibiao` is already transitive, so the
  build graph gains no crate. Native `cargo metadata` resolution is more correct
  than the bespoke parser, which string-matches `"crates/"` and hand-splits the
  members array.

## Risks / Trade-offs

- [`strict_external` single-segment over-reaction on `uuid`] → Near-zero here (see
  above); the clean-workspace reaction test would fail loudly if it triggered.
- [Two `guibiao` versions could mismatch the `Constitution` type] → Pin `guibiao`
  to `tianheng`'s version via a workspace dependency and upgrade the family in
  lockstep; the DoD build/test gate would fail on a mismatch.
- [Reaction-test fixtures drift from the real boundary target paths] → The
  precision (clean-direction) assertion guards against a fixture that no longer
  matches the boundary silently passing as "reacted".

## Migration Plan

No runtime or consumer migration. Steps: apply the constitution edits and manifest
change, add the reaction tests, run the full DoD gate, confirm `Cargo.lock` carries
`guibiao 0.1.9`, then land on `release/0.1.0` via the integration ritual.

## Open Questions

None.
