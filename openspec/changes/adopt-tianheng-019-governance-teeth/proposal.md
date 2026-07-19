## Why

Pacta's earlier feedback to the Tianheng maintainer landed in Tianheng 0.1.9,
which ships exactly the three capabilities Pacta was waiting on. Two of Pacta's
governance boundaries currently under-deliver their shipped spec promises: a
fully-qualified `uuid::Uuid::now_v7()` written without a `use` slips past the
ambient-time reaction, and the kernel async-exposure and facade kernel-exclusion
reactions are asserted by no test — only the clean workspace is checked, never
that they fire on a leak. Folding these into the paused 0.1.0 makes governance
honest before publishing.

## What Changes

- Adopt Tianheng 0.1.9 (bump already staged; opt-in features keep the check
  byte-identical until turned on).
- Close the fully-qualified ambient-time hole: add `.strict_external()` to the
  `uuid` ambient-time boundary so `uuid::Uuid::now_v7()` / `now_v1()` written
  without `use uuid` is caught. `uuid` only — `std::time` is a sysroot head
  already caught by default, where `.strict_external()` would be a no-op.
- Prove the semantic reactions fire: add reaction tests that assert the kernel
  async-exposure boundary and the facade kernel-exclusion boundary return a
  violation on leaking source and stay clean on matching non-leaking source
  (precision, guarding a vacuous always-fires). Implemented by extending the
  existing in-test `TempWorkspace` helper with leaking source — no committed
  fixture crates, no new dependency.
- Adopt native workspace coverage: replace the bespoke
  `every_workspace_crate_has_a_boundary` test and its hand-rolled manifest
  parser with `guibiao::check_and_cover`, asserting `total > 0` and
  `uncovered` empty. This adds `guibiao` as a direct dependency of
  `pacta-governance` (pinned to Tianheng's version so the graph carries one
  `guibiao`), which widens `pacta-governance`'s own dependency boundary to
  `["tianheng", "guibiao"]` with a rewritten justification.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `quality-governance`: the ambient-time reaction additionally rejects a
  fully-qualified external time constructor written without an import; the
  semantic reactions (kernel async-exposure, facade kernel-exclusion) are
  required to be proven by an executable reaction test; workspace coverage is
  enforced through the native coverage projection.

## Impact

- Affects only the internal, unpublished `pacta-governance` crate
  (`publish = false`) — the constitution, its tests, and its manifest. The
  published consumer surface is untouched.
- Adds a direct `guibiao` dependency to `pacta-governance` (already transitive
  via `tianheng`; zero new crates in the build graph).
- Updates `openspec/specs/quality-governance` and syncs `BACKLOG.md`
  (Release Plan governance dependency line).
- No change to the contract, executor, driver, memory, conformance, or facade
  crates, or to any runtime behavior.
