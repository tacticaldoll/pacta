## 1. Runtime Vocabulary Governance

- [x] 1.1 Update `PROJECT.md` to describe Pacta as Pacta-native and middleware-oriented rather than Tower-native.
- [x] 1.2 Update `docs/domain-language.md` with runtime vocabulary and Tower adapter boundary.
- [x] 1.3 Update README and roadmap wording where Tower-native positioning appears.

## 2. Executor Skeleton

- [x] 2.1 Add `crates/pacta-executor` to the workspace.
- [x] 2.2 Define Pacta-native executor, execution, outcome, and settlement vocabulary.
- [x] 2.3 Keep `pacta-executor` free of Tower dependencies.

## 3. Driver Skeleton

- [x] 3.1 Add `crates/pacta-driver` to the workspace.
- [x] 3.2 Implement a minimal driver step that claims by docket, executes, and settles.
- [x] 3.3 Add tests for successful fulfillment, breached execution, and idle docket behavior.
- [x] 3.4 Keep retry, backoff, scheduling, shutdown, Tribunal, and backend behavior out of scope.

## 4. Governance

- [x] 4.1 Add Tianheng boundaries for `pacta-executor` and `pacta-driver`.
- [x] 4.2 Ensure the contract crate remains isolated from runtime crates.
- [x] 4.3 Run `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.

## 5. Documentation

- [x] 5.1 Update `docs/development-flow.md` only if new commands or crate layout affect contributor workflow.
- [x] 5.2 Update `BACKLOG.md` to reflect the shipped part of Phase 2 and the deferred Tower adapter.

## 6. Verification

- [x] 6.1 Run `cargo build`.
- [x] 6.2 Run `cargo test`.
- [x] 6.3 Run `cargo clippy --all-targets -- -D warnings`.
- [x] 6.4 Run `cargo fmt --all --check`.
- [x] 6.5 Run `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`.
- [x] 6.6 Run `cargo deny check`.
- [x] 6.7 Run `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.
