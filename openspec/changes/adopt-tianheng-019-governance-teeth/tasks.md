## 1. Dependency adoption

- [x] 1.1 Confirm `tianheng = "0.1.9"` in root `Cargo.toml` and `Cargo.lock` resolves the family at 0.1.9 (already staged)
- [x] 1.2 Add `guibiao = "0.1.9"` to `[workspace.dependencies]` in root `Cargo.toml`, pinned to Tianheng's version
- [x] 1.3 Add `guibiao.workspace = true` to `crates/pacta-governance/Cargo.toml`

## 2. Close the fully-qualified ambient-time hole (#1)

- [x] 2.1 Add `.strict_external()` to the `uuid` ambient-time `ModuleBoundary` in `crates/pacta-governance/src/main.rs` (uuid only; leave `std::time` untouched)
- [x] 2.2 Verify `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` stays clean on the real workspace

## 3. Native workspace coverage (#2)

- [x] 3.1 Widen the `pacta-governance` `CrateBoundary` in the constitution to `restrict_dependencies_to(["tianheng", "guibiao"])`
- [x] 3.2 Rewrite `GOVERNANCE_REASON` to state independence from the judged workspace graph, noting `guibiao` as governance-family tooling outside that graph
- [x] 3.3 Replace the `every_workspace_crate_has_a_boundary` test and the `workspace_members` parser with a test calling `guibiao::check_and_cover(constitution().static_boundaries(), &manifest)` (pass `static_boundaries()`, not `constitution()` — `check_and_cover` takes `&guibiao::Constitution`), asserting `total > 0` and `uncovered` empty
- [x] 3.4 Remove the now-unused `workspace_members` helper

## 4. Testable teeth for the semantic reactions (#3)

- [x] 4.1 Extend the in-test `TempWorkspace` helper to write leaking source (no committed fixture crates). Every fixture handed to `check_all` is a COMPLETE workspace containing BOTH `pacta-contract` (with a resolvable `crate::kernel` module) and `pacta` (lib root present) — a missing target crate/module makes `check_all` return `Outcome::ConstitutionError`, not a skip
- [x] 4.2 Add a reaction test asserting `tianheng::check_all(constitution().semantic_boundaries(), &manifest)` reports a violation whose target is `pacta-contract` (async-exposure rule) for a fixture whose `crate::kernel` exposes a `pub async fn`; `pacta` present with no kernel re-export. (Use `check_all`, not `check_semantic` — the latter is signature-only and takes `&[SemanticBoundary]`.) Assert the specific violation identity, not a bare `Outcome::Violations`
- [x] 4.3 Add a reaction test asserting `check_all` reports a violation whose target is `pacta` (must-not-expose rule) for a fixture `pacta` that re-exports `pacta_contract::kernel::X`; `pacta-contract` present with a non-async `kernel` exposing `X`. Assert the specific violation identity
- [x] 4.4 Add a precision (clean-direction) assertion: a fixture with both crates present, `pacta-contract::kernel` non-async, and `pacta` with no kernel re-export reports `Outcome::Clean`

## 5. Spec and prose sync

- [x] 5.1 Archive-sync the `quality-governance` delta (handled at archive time): modified "Core Reads No Ambient Time" scenario and new "Semantic Reactions Are Executably Proven" requirement
- [x] 5.2 Update `BACKLOG.md` Release Plan line so the `pacta-governance` dependency note reads `tianheng` and `guibiao`

## 6. Definition of Done

- [x] 6.1 `cargo build --workspace`
- [x] 6.2 `cargo test --workspace` (new reaction tests pass in both firing and clean directions)
- [x] 6.3 `cargo clippy --all-targets -- -D warnings`
- [x] 6.4 `cargo fmt --all --check`
- [x] 6.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 6.6 `cargo deny check`
- [x] 6.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (clean)
- [x] 6.8 Confirm `Cargo.lock` committed with `guibiao 0.1.9` pinned
- [x] 6.9 `openspec validate adopt-tianheng-019-governance-teeth --strict`
