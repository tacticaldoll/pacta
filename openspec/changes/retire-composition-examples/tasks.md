## 1. Remove the example artifacts

- [x] 1.1 Delete `crates/pacta/examples/compose.rs`
- [x] 1.2 Delete `crates/pacta-driver/examples/compose.rs`
- [x] 1.3 Remove the `uuid` dev-dependency from `crates/pacta/Cargo.toml` (unused once the example is gone; facade doctest uses `Default::default()`)
- [x] 1.4 Confirm `crates/pacta-driver/Cargo.toml` keeps `uuid` (its `#[cfg(test)]` unit tests still use it)

## 2. Confirm the doctest carries the proof

- [x] 2.1 Verify the facade `lib.rs` "Composing the lifecycle" doctest still drives to `Step::Fulfilled`, imports only from `pacta`, uses a pass-through middleware, and a lifecycle-only registry (no source change expected; it already does)

## 3. Spec and prose sync

- [x] 3.1 Archive-sync the `public-facade` delta (handled at archive time): the Facade Composition Example requirement now governs the facade doctest and carries the pass-through and registry-purity scenarios
- [x] 3.2 Archive-sync the `composition-example` REMOVED delta (handled at archive time): the capability is retired
- [x] 3.3 Record the reconsideration in `BACKLOG.md` (composition examples retired; the facade doctest and pacta-driver unit tests are the proof; why)

## 4. Definition of Done

- [x] 4.1 `cargo build --workspace`
- [x] 4.2 `cargo test --workspace` (facade doctest runs and asserts; pacta-driver unit tests still pass)
- [x] 4.3 `cargo clippy --all-targets -- -D warnings`
- [x] 4.4 `cargo fmt --all --check`
- [x] 4.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 4.6 `cargo deny check`
- [x] 4.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (clean)
- [x] 4.8 `openspec validate retire-composition-examples --strict`
