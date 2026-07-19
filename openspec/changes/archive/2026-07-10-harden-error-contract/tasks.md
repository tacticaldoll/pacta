## 1. Bind the associated error types

- [x] 1.1 Bind `Registry::Error` to `std::error::Error` in `pacta-contract`
- [x] 1.2 Bind `Executor::Error` to `std::error::Error` in `pacta-executor`

## 2. Implement the standard error trait on concrete errors

- [x] 2.1 Implement `Display` and `std::error::Error` on `DriverError`, returning
  the wrapped registry or executor error from `source()`; the impls carry the
  `R, E: std::error::Error` bounds (plus `+ 'static` for `source`) those traits
  require
- [x] 2.2 Implement `Display` and `std::error::Error` on `NotHeld` in `pacta-memory`

## 3. Satisfy the bound everywhere it is implemented

- [x] 3.1 Replace BOTH in-crate driver test errors that used `()` (the
  `TestRegistry` and `TestExecutor` error types) with a small real error type that
  derives `Debug, PartialEq, Eq` (the driver tests `assert_eq!` on `DriverError`,
  which needs `PartialEq`) and implements `Display` + `std::error::Error`, and
  update the `Err(DriverError::Executor(()))` assertion literal accordingly
- [x] 3.2 Implement `Display` and `std::error::Error` on the executor test
  `DummyError`
- [x] 3.3 Confirm the composition example compiles unchanged (`Infallible` already
  implements `std::error::Error`)

## 4. Validate

- [x] 4.1 Run `openspec validate harden-error-contract --strict`
- [x] 4.2 Add or extend a test asserting a `DriverError` displays and exposes its
  source

## 5. Definition of Done

- [x] 5.1 `cargo build --workspace`
- [x] 5.2 `cargo test --workspace`
- [x] 5.3 `cargo clippy --all-targets -- -D warnings`
- [x] 5.4 `cargo fmt --all --check`
- [x] 5.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.6 `cargo deny check`
- [x] 5.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 5.8 Adversarial review passes before commit
