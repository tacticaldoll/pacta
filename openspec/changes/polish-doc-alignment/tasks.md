## 1. Spec delta

- [x] 1.1 `governance-prose` delta: MODIFY Document Authority to add "The Definition of Done is single-sourced"
- [x] 1.2 `openspec validate polish-doc-alignment --strict` passes

## 2. Definition of Done — single-source in AGENTS.md

- [x] 2.1 `AGENTS.md`: state the complete seven-gate DoD (build, test, `cargo clippy --workspace --all-targets -- -D warnings`, fmt, `rustdoc -D warnings`, `cargo deny check`, governance) `--workspace` consistently, plus the per-gate ownership note (moved from the flow doc), plus a line noting CI additionally verifies the MSRV build (`cargo +1.88 build --workspace`)
- [x] 2.2 `docs/development-flow.md`: replace its DoD gate list with a pointer to `AGENTS.md` (keep the lifecycle checklist and the "CI runs the same gates" note)
- [x] 2.3 `README.md` Contributing: point to `AGENTS.md` for the DoD; keep the `openspec new change` quickstart; remove the divergent five-gate block

## 3. Description alignment

- [x] 3.1 `crates/pacta-contract/Cargo.toml`: change `description` from "The isolated core contract for the Pacta task runtime." to "The isolated core contract for Pacta: durable lifecycle vocabulary and a sans-I/O kernel." (matches the README opening; drops the "task runtime" identity label; keywords unchanged)

## 4. Crate README license reflow

- [x] 4.1 Reflow the license block in all seven `crates/*/README.md` to a single line, matching the root README's form; keep the absolute LICENSE URLs (do NOT make them relative)

## 5. Verify (Definition of Done — dogfooded)

- [x] 5.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 5.2 `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 5.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.4 `cargo deny check`
- [x] 5.5 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (active-prose parses the edited docs)
- [x] 5.6 Confirm the three docs now agree (AGENTS complete; README + flow point to it); crate license blocks render identically to before; `cargo metadata` shows the new `pacta-contract` description
