## 0. Precondition (ordering)

- [x] 0.1 This change depends on `prepare-release-hygiene` having flipped the workspace to `publish = true` (workspace default true; only `pacta-governance` keeps explicit `publish = false`). Do NOT implement the facade until that predecessor has landed — a `publish = false` facade is the audited DRIFT state, and `cargo publish` of the facade requires its path-dependencies to be publishable.

## 1. Facade crate

- [x] 1.1 Add `crates/pacta/Cargo.toml`: package `pacta`, inherit version/edition/license/publish (the workspace default is `true` after the predecessor, so the facade is publishable), `description`, normal deps on `pacta-contract`/`pacta-executor`/`pacta-driver` (workspace), `uuid` as a dev-dependency
- [x] 1.2 Add `crates/pacta/src/lib.rs`: crate docs, `#![forbid(unsafe_code)]`, `#![warn(missing_docs)]`, and re-exports ONLY — contract (`Pact`, `Claim`, `Retainer`, `Timestamp`, `Outcome`, `Settlement`, `Registry`), executor (`Executor`, `Execution`, `Middleware`, `Policy`), driver (`Driver`, `Step`, `DriverError`); no kernel, no logic
- [x] 1.3 Add `crates/pacta` to `members` in root `Cargo.toml`, and add `pacta-driver = { path = "crates/pacta-driver" }` to `workspace.dependencies`

## 2. Facade example

- [x] 2.1 Add `crates/pacta/examples/compose.rs`: compose the lifecycle end to end (claim → execute through a pass-through `Middleware` → settle) importing only from `pacta::`, asserting `Step::Fulfilled`; use Pacta domain-language names
- [x] 2.2 Confirm `crates/pacta-driver/examples/compose.rs` is unchanged (the core three-crate proof stays)

## 3. Governance

- [x] 3.1 Add a `CrateBoundary` for `pacta` restricting deps to `{pacta-contract, pacta-executor, pacta-driver}` with a `.because(...)` justification (publisher-owned entrypoint)
- [x] 3.2 Add a `signature_boundary` (hunyi): `SemanticBoundary::in_crate("pacta").module("crate").must_not_expose("pacta_contract::kernel").because(...)`
- [x] 3.3 Add a source-scan reaction that fails when `crates/pacta/src` declares any item other than a re-export (allow `pub use`, attributes, docs); wire it into the `check` path like the ambient-time scan, with a unit test for a violating and a clean sample
- [x] 3.4 Update `unapproved_core_dependency_is_rejected` TempWorkspace: add the `pacta` package and list it in the synthetic root manifest members
- [x] 3.5 Confirm `every_workspace_crate_has_a_boundary` passes with `pacta` now governed
- [x] 3.6 Add a `cargo test` regression for the hunyi kernel-exclusion boundary if feasible (a sample facade re-exporting the kernel expects a violation), so the gate has teeth beyond the DoD `check` run; confirm `.module("crate")` resolves the crate root during 3.2

## 4. Prose / roadmap sync

- [x] 4.1 Update `BACKLOG.md` Workspace Composition to record the published facade entrypoint as workspace-owned (stance reversal), consistent with the `quality-governance` delta

## 5. Definition of Done

- [x] 5.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 5.2 `cargo clippy --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 5.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.4 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (dep boundary, hunyi kernel-exclusion, re-exports-only, prose, ambient-time all green)
- [x] 5.5 `cargo deny check`
- [x] 5.6 `cargo run --example compose -p pacta` and `cargo run --example compose -p pacta-driver` both succeed
- [x] 5.7 `cargo publish --dry-run -p pacta` succeeds (proves the facade's dependency graph is publishable, not only that the name is free)
- [x] 5.8 `openspec validate establish-pacta-facade --strict`
