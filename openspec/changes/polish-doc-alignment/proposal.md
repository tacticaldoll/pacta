## Why

A pre-finalization denoise/alignment pass, from an audit of the `release/0.1.1`
doc surface. The one genuine defect: the Definition of Done is stated in three
places and they disagree — `README.md` and `AGENTS.md` list five gates, while
`docs/development-flow.md` lists seven (adding `RUSTDOCFLAGS="-D warnings" cargo doc`
and `cargo deny check`, both required by `quality-governance`), and CI enforces those
seven **plus** an MSRV build (`cargo +1.88 build --workspace`) that no doc lists. A
contributor following `AGENTS.md` (which the flow doc itself calls authoritative)
passes locally and then fails CI on the missing gates. Two cheaper items ride along: `pacta-contract`'s
crates.io `description` says "task runtime" (mildly against `product-positioning`,
which rejects "runtime"), and the seven crate READMEs' license blocks diverge in
*form* from the root README's one-liner.

## What Changes

- Make `AGENTS.md` the single, complete Definition of Done: the seven local gates
  (clippy with `--workspace` to match CI exactly) plus the per-gate ownership note
  currently only in the flow doc, and a line noting CI additionally verifies the MSRV
  build. `README.md` Contributing and `docs/development-flow.md` point to it instead of
  restating a divergent subset.
- Govern it so the prose cannot drift again: `governance-prose`'s Document Authority
  gains a scenario requiring the Definition of Done to be single-sourced across project
  prose (prose consistency — not a mechanical doc-vs-CI guarantee).
- Fix `pacta-contract`'s `description`, which labels Pacta *as* a "task runtime" (a
  product-identity claim), to match its README opening and `product-positioning`
  ("durable contract fabric", not a runtime). The `runtime`/`task` workspace keywords
  stay — only the identity label in the description changes.
- Reflow the seven crate README license blocks to the root's one-line form, keeping
  their **absolute** LICENSE URLs (relative links would 404 on crates.io, since the
  LICENSE files live at the repo root, not in each crate's tarball).

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `governance-prose`: Document Authority gains "The Definition of Done is
  single-sourced" — `AGENTS.md` states the complete DoD and other active prose points
  to it rather than restating a divergent subset, so the documented gates stay
  consistent across project prose.

## Impact

- Docs + one crate `description`. No source code, no public API, no manifest version.
- Files: `AGENTS.md`, `README.md`, `docs/development-flow.md`, seven
  `crates/*/README.md`, `crates/pacta-contract/Cargo.toml`, and the
  `governance-prose` spec.
- Deliberately **not** touched (audit judged these defensible or intentional): the
  BACKLOG↔spec workspace-composition digest overlap, the cross-spec facade-doctest
  overlap, and the README domain-language curated subset (omitting `Registry`).
