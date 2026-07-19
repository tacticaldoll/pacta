# Tasks

## 1. Harden the scan functions (`pacta-governance/src/main.rs`)
- [x] 1.1 `check_active_prose`: on a read failure for any `ACTIVE_PROSE_FILES` entry, push a synthetic `ProseViolation` (file path from the entry, `line: 0`, a `&'static str` phrase/reason such as `phrase: "<unreadable>"`, `reason: "a governed active-prose file must be present and readable"`) instead of `continue`, so a missing/unreadable governed file fails the gate and names the file
- [x] 1.2 `check_facade_reexports_only`: on a read failure for a facade source file, push a synthetic `SourceViolation` (file path, `line: 0`, `marker: "unreadable facade source"`) instead of `continue`
- [x] 1.3 `check_facade_reexports_only`: fail if no facade source files were scanned at all (guard parity with `coverage.total > 0`)
- [x] 1.4 Ensure `main()` surfaces the new failures with a clear message and exit code 1 (reuse the existing failure plumbing)

## 2. Prove it (tests)
- [x] 2.1 Test: `check_active_prose` against a root missing the governed files returns `Err` (fires)
- [x] 2.2 Test: `check_facade_reexports_only` against a root with no facade source tree returns `Err` (fires)
- [x] 2.3 Confirm the existing clean tests still pass (`current_active_prose_satisfies_governance`, `current_facade_is_reexports_only`) — the real workspace stays clean
- [x] 2.4 Test: a present-but-unreadable facade source file (invalid UTF-8, so `read_to_string` fails portably without permission games) returns `Err` with the `unreadable facade source` marker and NOT the empty-tree marker — backs the distinct unreadable-file scenario

## 3. Definition of Done
- [x] 3.1 `cargo build --workspace`
- [x] 3.2 `cargo test --workspace`
- [x] 3.3 `cargo clippy --all-targets -- -D warnings`
- [x] 3.4 `cargo fmt --all --check`
- [x] 3.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 3.6 `cargo deny check`
- [x] 3.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (real workspace still clean)
- [x] 3.8 `openspec validate harden-governance-file-reads --strict`
