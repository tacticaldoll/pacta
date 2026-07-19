## 1. Encapsulate Retainer

- [x] 1.1 Make `Retainer`'s field private in `pacta-contract`
- [x] 1.2 Add `Retainer::new(Uuid)` and `Retainer::id(&self) -> Uuid` with docs
- [x] 1.3 Correct the `Retainer` doc: authority is registry-validated, not type-proven

## 2. Update construction sites

- [x] 2.1 Update the kernel test in `pacta-contract` to use `Retainer::new`
- [x] 2.2 Update the driver test in `pacta-driver` to use `Retainer::new`
- [x] 2.3 Update `crates/pacta-driver/examples/compose.rs` to use `Retainer::new`

## 3. Definition of Done

- [x] 3.1 `cargo build --workspace`
- [x] 3.2 `cargo test --workspace`
- [x] 3.3 `cargo clippy --all-targets -- -D warnings`
- [x] 3.4 `cargo fmt --all --check`
- [x] 3.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 3.6 `cargo deny check`
- [x] 3.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 3.8 `cargo run -p pacta-driver --example compose` still runs
