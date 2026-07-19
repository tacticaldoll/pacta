## 1. Name the facade everywhere

- [x] 1.1 Add a `Curated facade (pacta)` bullet to `CHANGELOG.md` 0.1.0 "Added" (the single compose-level entrypoint that re-exports the contract/executor/driver surface)
- [x] 1.2 Add the `pacta` facade to `README.md` "Status (0.1.0)" ships-list and name it the entrypoint in the architecture framing
- [x] 1.3 Sync the `release-packaging` MODIFIED requirements into `openspec/specs/release-packaging/spec.md` (Publishable Crate Set includes `pacta`; Honest Release Scope names the facade)

## 2. Fix stale placeholders and self-contradiction

- [x] 2.1 Replace the six `TBD ... Update Purpose after archive` Purpose lines with real one-line Purposes: `public-facade`, `release-packaging`, `surface-tiers`, `contract-manifestation`, `lifecycle-persistence`, `registry-conformance` (drop the false "created by archiving" provenance)
- [x] 2.2 Remove "or archiving a change" from the `AGENTS.md` Definition of Done preamble
- [x] 2.3 Add the facade kernel-exclusion and re-exports-only reactions to the `BACKLOG.md` Current Baseline shipped-governance bullet

## 3. Release packaging hygiene

- [x] 3.1 Add `LICENSE-MIT` and `LICENSE-APACHE` to each publishable crate (`pacta`, `pacta-contract`, `pacta-executor`, `pacta-driver`, `pacta-memory`, `pacta-conformance`) as symlinks to the root files
- [x] 3.2 Add the release date to the `CHANGELOG.md` 0.1.0 header (`## [0.1.0] - 2026-07-12`)

## 4. Retire ADRs (adopt git-as-provenance)

- [x] 4.1 Add the git-as-provenance principle to `AGENTS.md` (Document Authority / OpenSpec Workflow): decisions are recorded in git history + PR descriptions, reconsiderations in `BACKLOG.md`; living docs are the SSOT; there is no separate ADR file class
- [x] 4.2 Re-home into `BACKLOG.md`: the archive-dissolution rationale and the ADR-retirement decision (as reconsideration entries)
- [x] 4.3 `git rm -r docs/adr/`
- [x] 4.4 Sync the `governance-prose` and `quality-governance` MODIFIED requirements into `openspec/specs/` (historical-prose scenarios cite git history only; drop "superseded ADRs")

## 5. Definition of Done

- [x] 5.1 `openspec validate --all --strict`
- [x] 5.2 `cargo build --workspace` and `cargo test --workspace`
- [x] 5.3 `cargo clippy --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 5.4 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.5 `cargo deny check`
- [x] 5.6 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (clean — active prose still passes after edits)
- [x] 5.7 `cargo publish --workspace --dry-run` packages all six crates and each tarball now carries `LICENSE-MIT`/`LICENSE-APACHE`
- [x] 5.8 Confirm no active-file reference to `docs/adr` or ADRs remains (the two governance scenarios now cite git history only)
- [x] 5.9 Retire this change: sync deltas + Purposes, then `git rm -r openspec/changes/final-release-hygiene/` — no archive
