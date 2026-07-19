## 1. CI And Supply Chain

- [x] 1.1 Add a GitHub Actions workflow for build, test, clippy, fmt, rustdoc, cargo-deny, and pacta-governance.
- [x] 1.2 Add `deny.toml` with Pacta's current advisory, license, ban, and source policy.
- [x] 1.3 Ensure CI runs on push and pull request events.

## 2. Tianheng Governance

- [x] 2.1 Review `pacta-governance` boundaries against the current crate graph.
- [x] 2.2 Keep `pacta-contract` and `pacta-governance` isolated from workspace crate dependencies.
- [x] 2.3 Run the governance reaction locally with `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.

## 3. Enforced Rust Style

- [x] 3.1 Add crate-level attributes that make unsafe code and public contract documentation react.
- [x] 3.2 Ensure rustfmt, clippy, and rustdoc warnings are represented as executable gates.
- [x] 3.3 Keep style documentation concise and point contributors at the gates rather than duplicating tool rules.

## 4. Documentation

- [x] 4.1 Update `README.md` contribution guidance to mention the quality gates.
- [x] 4.2 Update `docs/development-flow.md` with CI/governance commands.
- [x] 4.3 Update `BACKLOG.md` if quality governance changes the roadmap phase description.

## 5. Verification

- [x] 5.1 Run `cargo build`.
- [x] 5.2 Run `cargo test`.
- [x] 5.3 Run `cargo clippy --all-targets -- -D warnings`.
- [x] 5.4 Run `cargo fmt --all --check`.
- [x] 5.5 Run `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`.
- [x] 5.6 Run `cargo deny check`.
- [x] 5.7 Run `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.
