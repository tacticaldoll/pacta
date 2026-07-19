## 1. Project the contract onto the facade

- [x] 1.1 Expand `crates/pacta/src/lib.rs` crate-root rustdoc with a "The contract" section stating the implementer half (Registry must provide claim/heartbeat/fulfill/breach with a lease; `pacta-conformance` is the executable proof) and the user-obligation half (idempotent `Executor` for at-least-once recovery; user-owned lease sizing; runtime-owned heartbeat cadence). Reference the governed truth; do not restate behavior.
- [x] 1.2 Add a crate-root doctest that composes claim → execute through a pass-through `Middleware` → settle using only `pacta::` items, asserting `Step::Fulfilled`

## 2. Name the reference pieces

- [x] 2.1 `crates/pacta-memory/src/lib.rs`: rustdoc states it is an in-memory reference backend, not durable/production, and durable backends live outside the workspace and prove against `pacta-conformance`
- [x] 2.2 `crates/pacta-driver/src/lib.rs`: `Driver` rustdoc states it drives synchronously and does not heartbeat in flight — safe for sub-lease and single-worker use; long/multi-worker workloads compose their own loop over the `Registry` contract

## 3. Record the L3 fork

- [x] 3.1 `BACKLOG.md` Recorded Reconsiderations: the kernel does not model heartbeat (directives are `Claim | Execute | Settle | Idle`); a sans-I/O-pure `Heartbeat` directive collides with the synchronous `Executor` — recorded as a future fork, not decided

## 4. Definition of Done

- [x] 4.1 `cargo build --workspace` and `cargo test --workspace` (the new facade doctest runs and passes)
- [x] 4.2 `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 4.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` (rustdoc links resolve)
- [x] 4.4 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (re-exports-only + `must_not_expose(kernel)` + prose gate still green; the doctest and docs are not items)
- [x] 4.5 `cargo deny check`
- [x] 4.6 `openspec validate manifest-the-contract --strict`
