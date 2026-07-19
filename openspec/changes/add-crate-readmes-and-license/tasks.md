## 1. Spec & planning

- [x] 1.1 Write the `release-packaging` delta (MODIFIED Release Metadata: crate-specific readme)
- [x] 1.2 `openspec validate add-crate-readmes-and-license --strict` passes

## 2. Per-crate READMEs & manifests

- [x] 2.1 Add `crates/pacta/README.md` (facade — recommended entrypoint)
- [x] 2.2 Add `crates/pacta-contract/README.md` (core contract + advanced-tier kernel)
- [x] 2.3 Add `crates/pacta-executor/README.md` (executor + middleware)
- [x] 2.4 Add `crates/pacta-driver/README.md` (mechanical runtime loop)
- [x] 2.5 Add `crates/pacta-memory/README.md` (in-memory reference backend)
- [x] 2.6 Add `crates/pacta-conformance/README.md` (backend-agnostic conformance suite)
- [x] 2.7 Add `crates/pacta-governance/README.md` (unpublished governance gate)
- [x] 2.8 In each `crates/*/Cargo.toml`, replace `readme.workspace = true` with `readme = "README.md"` (add the field for pacta-governance)
- [x] 2.9 Ensure the crate-local READMEs (esp. `crates/pacta/README.md`, the facade → crates.io page) are honest per Honest Release Scope: no removed vocabulary (`Policy`), no `Signal->Pact` presented as shipped ingress
- [x] 2.10 Remove the now-orphaned `readme` key from root `Cargo.toml` `[workspace.package]` (nothing inherits it once all crates set `readme` explicitly)

## 3. Root README

- [x] 3.1 Add a `## License` section (dual `MIT OR Apache-2.0` + standard Rust contribution terms)
- [x] 3.2 Correct the stale `Policy` references to match the frozen 0.1.0 surface

## 4. BACKLOG

- [x] 4.1 Add a BACKLOG thread for future packaging-metadata governance teeth
- [x] 4.2 Note in BACKLOG that the `0.1.1` version bump + changelog are the deferred mechanical release-finalization step (manifests stay at `0.1.0` until then)

## 5. Verify (Definition of Done)

- [x] 5.1 `cargo metadata --format-version 1 --no-deps` shows every publishable crate's `readme` resolving inside its own crate (not `../../README.md`)
- [x] 5.2 Confirm all 7 `crates/*/README.md` files exist on disk (cargo metadata reports the field string without checking existence, and `--workspace` dry-run skips the unpublished pacta-governance)
- [x] 5.3 `cargo build --workspace` and `cargo test --workspace`
- [x] 5.4 `cargo clippy --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 5.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.6 `cargo deny check`
- [x] 5.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 5.8 `cargo publish --dry-run --workspace` packages the publishable graph cleanly
