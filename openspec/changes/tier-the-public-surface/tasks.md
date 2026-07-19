## 1. Manifest the advanced tier (kernel)

- [x] 1.1 In `crates/pacta-contract/src/lib.rs`, expand the `kernel` module rustdoc with its driving contract: obtain the next `Directive` from `poll`, perform it, report a `Notice` via `on_event`, repeat until `result` yields a terminal `StepResult`
- [x] 1.2 Add a `kernel` module doctest that drives one step manually (poll → perform → `on_event`) to a terminal `StepResult`, constructed entirely from `pacta-contract` public items (no clock read; use `Timestamp::from_millis`)
- [x] 1.3 Add an advanced-tier note to the `kernel` module rustdoc: lower stability intent than the recommended tier (its API may evolve), reached through `pacta-contract` directly — while remaining a supported, governed core surface, NOT unsupported or slated for removal

## 2. Declare the tiers on the facade + thread the seam

- [x] 2.1 In `crates/pacta/src/lib.rs`, extend the "what is not here" section into a tier statement: the facade + backend-author path are the recommended (converging) tier; `pacta_contract::kernel` is the advanced tier (lower stability intent, API may evolve; still supported/governed)
- [x] 2.2 Refine the existing conformance-proof sentence in the facade rustdoc (already governed by `contract-manifestation`) to note `pacta-conformance` is a *dev-dependency* — the backend author's two-crate journey. No new requirement; it rides the shipped one

## 3. Record the un-commit fork

- [x] 3.1 `BACKLOG.md` **Recorded Reconsiderations**: record narrowing the kernel's *exposure* (feature-gate, etc.) as a 1.0-approach option — explicitly NOT deprecating the shipped, governed kernel, and not done now — noting it is committed today only because `pacta-driver` consumes it cross-crate

## 4. Definition of Done

- [x] 4.1 `cargo build --workspace` and `cargo test --workspace` (the kernel doctest runs and passes)
- [x] 4.2 `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 4.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` (kernel/facade rustdoc links resolve)
- [x] 4.4 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (ambient-time, async-exposure, facade scans, prose, coverage all green; the doctest is doc content, not an item)
- [x] 4.5 `cargo deny check`
- [x] 4.6 `openspec validate tier-the-public-surface --strict`
