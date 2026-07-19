## 1. Documentation And Contract Wording

- [x] 1.1 Replace misleading `zero-dependency` wording with isolated contract or zero workspace dependency wording in project docs and crate metadata.
- [x] 1.2 Update domain language documentation to describe Pacta-native composition and adapter scope without introducing Tower as core vocabulary.
- [x] 1.3 Update roadmap text so middleware/policy work is Pacta-native and Tower compatibility is explicitly adapter-owned.

## 2. Executable Governance

- [x] 2.1 Change `pacta-governance` crate boundaries from workspace-only restrictions to closed normal dependency allowlists for current core crates.
- [x] 2.2 Add governance tests or checks that prove an unapproved core dependency is rejected by Tianheng.
- [x] 2.3 Keep governance itself on a closed dependency boundary that allows Tianheng but no workspace crates.

## 3. Runtime Skeleton Semantics

- [x] 3.1 Update driver error types so executor infrastructure errors are surfaced to callers.
- [x] 3.2 Update driver settlement behavior so executor errors attempt `Registry::breach` before returning the executor error.
- [x] 3.3 Update runtime tests for fulfilled, breached, executor-error, and idle steps under the clarified semantics.

## 4. Verification

- [x] 4.1 Run `cargo build`.
- [x] 4.2 Run `cargo test`.
- [x] 4.3 Run `cargo clippy --all-targets -- -D warnings`.
- [x] 4.4 Run `cargo fmt --all --check`.
- [x] 4.5 Run `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`.
- [x] 4.6 Run `cargo deny check`.
- [x] 4.7 Run `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.
