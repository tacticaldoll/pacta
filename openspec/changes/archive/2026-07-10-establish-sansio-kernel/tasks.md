## 1. Lifecycle vocabulary consolidation

- [x] 1.1 Move `Outcome` and `Settlement` from `pacta-executor` into `pacta-contract` (lifecycle vocabulary)
- [x] 1.2 Update `pacta-executor` to re-use `pacta_contract::{Outcome, Settlement}`; keep `Executor`/`Middleware`/`Policy` in `pacta-executor`
- [x] 1.3 Confirm Tianheng constitution still passes (no new normal dep on core crates)

## 2. Sans-I/O kernel module (in `pacta-contract`)

- [x] 2.1 Add a `kernel` module in `pacta-contract` (pure, `#![forbid(unsafe_code)]` already on the crate, no async); reuse the crate's existing isolation
- [x] 2.2 Define `Directive` (claim, execute, settle, idle) and `Notice` (claimed, executed incl. infra-error, settled) in the domain register
- [x] 2.3 Implement the pure kernel: `poll() -> Directive` and `on_event(Notice)`, encoding the existing decision table exactly
- [x] 2.4 Unit-test the kernel with no I/O: fulfilled→fulfill, breached→breach, infra-error→breach, empty→idle

## 3. Rewire the driver as the runtime loop

- [x] 3.1 Rewrite `Driver` to poll the kernel and perform its directives via `Registry` and `Executor`, feeding notices back
- [x] 3.2 Preserve `Driver::step` observable behavior and the executor-error surfacing semantic
- [x] 3.3 Port the existing `pacta-driver` tests; they must pass unchanged in observable outcome

## 4. Rewire the composition example

- [x] 4.1 Keep `crates/pacta-driver/examples/compose.rs` driving composition via the public `Driver` (now sans-I/O internally); it compiles unchanged since `Outcome` is re-exported, preserving the composition-example spec scenarios
- [x] 4.2 Confirm the `composition-example` spec scenarios still hold (public-API-only, no orchestration, Registry purity)

## 5. Enforcement (ship the decision as a bite)

- [x] 5.1 Read the hunyi DSL (`async_exposure`, module/crate scoping) and how `tianheng` re-exports it
- [x] 5.1a If hunyi is not reachable via `tianheng` alone, amend the `pacta-governance` boundary (currently `["tianheng"]`) explicitly to allow it
- [x] 5.2 Confirm `pacta-contract`'s existing Tianheng boundary already isolates the kernel (no new import rule needed)
- [x] 5.3 Add a hunyi reaction: the kernel module's public API must not expose `async fn`
- [x] 5.4 Wire it into `pacta-governance` and confirm the `governance` CI job runs it (clean + full coverage)

## 6. Definition of Done

- [x] 6.1 `cargo build --workspace`
- [x] 6.2 `cargo test --workspace`
- [x] 6.3 `cargo clippy --all-targets -- -D warnings`
- [x] 6.4 `cargo fmt --all --check`
- [x] 6.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 6.6 `cargo deny check`
- [x] 6.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 6.8 `cargo run -p pacta-driver --example compose` runs and settles as before
