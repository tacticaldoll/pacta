## 1. Executor Composition API

- [x] 1.1 Add a documented `Middleware` trait to `pacta-executor` that wraps an `Executor` into another `Executor`.
- [x] 1.2 Add a documented minimal `Policy` value that names orchestration intent without implementing orchestration behavior.
- [x] 1.3 Keep `pacta-executor` free of new normal dependencies and foreign framework vocabulary.

## 2. Runtime Tests

- [x] 2.1 Add tests proving middleware can wrap an executor and preserve a fulfilled outcome.
- [x] 2.2 Add tests proving middleware can alter an outcome through Pacta-native execution types.
- [x] 2.3 Add tests proving policy vocabulary is inspectable but non-behavioral.

## 3. Documentation

- [x] 3.1 Update domain language documentation with the concrete middleware and policy skeleton.
- [x] 3.2 Update README or backlog text so the shipped skeleton is separated from deferred retry, timeout, and rate-limit behavior.

## 4. Verification

- [x] 4.1 Run `cargo build`.
- [x] 4.2 Run `cargo test`.
- [x] 4.3 Run `cargo clippy --all-targets -- -D warnings`.
- [x] 4.4 Run `cargo fmt --all --check`.
- [x] 4.5 Run `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`.
- [x] 4.6 Run `cargo deny check`.
- [x] 4.7 Run `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.
