## 1. Example target

- [x] 1.1 Create `crates/pacta-driver/examples/compose.rs` as an `examples/` build target (no new workspace member)
- [x] 1.2 Confirm `pacta-driver` dev-dependencies already provide `uuid`; add nothing to normal `[dependencies]`

## 2. Consumer implementations (public API only)

- [x] 2.1 Implement an example-local in-memory `Registry` (lifecycle-only: `claim`, `heartbeat`, `fulfill`, `breach`) holding one claimable `Claim`
- [x] 2.2 Implement an example-local `Executor` that reports `Outcome::Fulfilled` with no orchestration logic
- [x] 2.3 Implement an example-local pass-through `Middleware` whose `wrap` forwards `execute` unchanged (the forward-compat seam)

## 3. Compose and drive

- [x] 3.1 In `main`, build the registry, wrap the executor with the middleware, and construct `Driver::new(registry, wrapped, dockets)`
- [x] 3.2 Call `driver.step()` and assert/print `Step::Fulfilled`, demonstrating claim -> execute -> settle end to end

## 4. Verify against the spec constraints

- [x] 4.1 Confirm the example references only public items (no crate-private module or field access)
- [x] 4.2 Confirm the registry performs no clause inspection, delay, backoff, or policy evaluation (Registry purity)
- [x] 4.3 Confirm no retry/backoff/timeout/rate-limit logic exists anywhere in the example
- [x] 4.4 Confirm example type names use the Pacta domain language (contract/arbitration register), not mechanical `Dummy`/`Test`/`Identity` names

## 5. Definition of Done

- [x] 5.1 `cargo build --workspace` (note: does not compile examples; example compilation is proven by 5.3 clippy `--all-targets` and 5.8 running it)
- [x] 5.2 `cargo test --workspace`
- [x] 5.3 `cargo clippy --all-targets -- -D warnings`
- [x] 5.4 `cargo fmt --all --check`
- [x] 5.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 5.6 `cargo deny check`
- [x] 5.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 5.8 Run the example: `cargo run -p pacta-driver --example compose`
