## 1. Enable publishing

- [x] 1.1 In root `Cargo.toml` `[workspace.package]`, set `publish = true`
- [x] 1.2 Keep `crates/pacta-governance/Cargo.toml` at explicit `publish = false`; on `pacta-contract`, `pacta-memory`, and `pacta-conformance` replace explicit `publish = false` with `publish.workspace = true` (consistent with `pacta-executor`/`pacta-driver`, which already inherit)

## 2. Publishable dependency graph

- [x] 2.1 Add `version = "0.1.0"` beside `path` in each `workspace.dependencies` entry consumed by a publishable crate (`pacta-contract`, `pacta-executor`, `pacta-conformance`)

## 3. Release metadata

- [x] 3.1 In `[workspace.package]` add `repository = "https://github.com/tacticaldoll/pacta"`, `readme = "README.md"`, `keywords`, and `categories`. Keywords: ≤5, each ≤20 chars, charset `[a-z0-9_-]` starting alphanumeric. Categories: MUST be exact slugs from the crates.io fixed category list (cross-check against it) — `cargo publish --dry-run` does NOT validate category slugs, so a bad slug passes the DoD and then fails the irreversible real publish
- [x] 3.2 In each publishable crate, opt into the shared metadata (`repository.workspace = true`, `readme.workspace = true`, `keywords.workspace = true`, `categories.workspace = true`); leave `pacta-governance` unchanged

## 4. MSRV

- [x] 4.1 In `[workspace.package]` add `rust-version = "1.88"`; opt each publishable crate into `rust-version.workspace = true`
- [x] 4.2 Add a CI job/step that explicitly installs and selects the 1.88 toolchain (there is no `rust-toolchain.toml` pin; e.g. `dtolnay/rust-toolchain@1.88` or `rustup toolchain install 1.88` then `cargo +1.88 build --workspace`) and builds the workspace on it

## 5. Changelog

- [x] 5.1 Add `CHANGELOG.md` (Keep a Changelog format) with a single `0.1.0` entry describing shipped truth: lifecycle contract, sans-I/O kernel, lease/lapse, in-memory reference backend, conformance suite, executable governance — and what is NOT shipped (durable backend, ingress, adapters)

## 6. README honesty

- [x] 6.1 Add a "Status (0.1.0)" section stating what the release ships and that durable backends live outside the workspace and prove against the conformance suite
- [x] 6.2 Mark the `Signal -> Pact` step as user-provided ingress, not a shipped public API
- [x] 6.3 Confirm edits stay green against the prose-governance stale-phrase gate

## 7. Roadmap sync

- [x] 7.1 Update `BACKLOG.md` to record the 0.1.0 crates.io release plan, the publishable crate set, and the `establish-pacta-facade` follow-on with its ordering dependency on this change, so the predecessor rationale is traceable

## 8. CI clarity

- [x] 8.1 Make the Definition-of-Done gates explicitly `--workspace` in `.github/workflows/ci.yml`

## 9. Definition of Done

- [x] 9.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 9.2 `cargo clippy --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 9.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 9.4 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (prose gate green after README edits)
- [x] 9.5 `cargo deny check`
- [x] 9.6 `cargo publish --dry-run --workspace` succeeds (verifies the inter-member dependency graph locally; a per-crate dry-run of a dependent would fail pre-publish because its verify build resolves deps from crates.io)
- [x] 9.7 `openspec validate prepare-release-hygiene --strict`
