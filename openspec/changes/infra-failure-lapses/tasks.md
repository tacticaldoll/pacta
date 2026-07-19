## 1. Spec deltas

- [x] 1.1 `runtime-skeleton` delta: kernel leaves infra error unsettled; driver settles nothing and surfaces the error
- [x] 1.2 `lifecycle-persistence` delta: at-least-once covers infra failure via lapse
- [x] 1.3 `openspec validate infra-failure-lapses --strict` passes

## 2. Kernel (pacta-contract)

- [x] 2.1 Add `Phase::DoneUnsettled` and `StepResult::Unsettled` (the enum stays `#[non_exhaustive]`)
- [x] 2.2 `on_event`: `(Executing, ExecutionFailed)` → `DoneUnsettled` (no longer `Settling { Breached }`)
- [x] 2.3 `poll`: `DoneUnsettled` → `Directive::Idle`; `result`: `DoneUnsettled` → `StepResult::Unsettled`
- [x] 2.4 Reword the `Notice::ExecutionFailed` doc: kernel settles nothing and leaves the claim to lapse; it fabricates no outcome
- [x] 2.5 Kernel rustdoc doctest verified unaffected: the happy-path (Fulfilled) example still drives to a terminal and passes (it never feeds `ExecutionFailed`); the unsettled behavior is documented in the `Notice::ExecutionFailed` and `StepResult::Unsettled` prose (2.4, 2.1)
- [x] 2.6 Fix the kernel unit test `infrastructure_error_settles_breached` (asserts `Settled(Breached)`): rename to `infrastructure_error_is_unsettled` and assert `StepResult::Unsettled`

## 3. Driver (pacta-driver)

- [x] 3.1 `step()`: on an executor error, feed `ExecutionFailed`, settle nothing (remove the `Registry::breach` call on the infra-error path), and return the executor error to the caller
- [x] 3.2 Handle the kernel's `StepResult::Unsettled` terminal in `step()` (map to returning the captured executor error; keep the existing wildcard arm on the non-exhaustive match)
- [x] 3.3 Update/adjust driver tests: an executor error leaves the claim unsettled (not breached) and surfaces the error; a follow-up claim after lease expiry reclaims the pact (flip the existing `breached == 1` assertion to `== 0`)
- [x] 3.4 Reword the `DriverError::Executor` doc comment ("after the claim was breached" is now false → "the claim is left unsettled to lapse and be reclaimed")

## 4. Docs & BACKLOG

- [x] 4.1 Resolve the BACKLOG "Infrastructure-failure handling during execution" reconsideration (now addressed by this change)

## 5. Verify (Definition of Done)

- [x] 5.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 5.2 `cargo clippy --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 5.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` (kernel doctest passes)
- [x] 5.4 `cargo deny check`
- [x] 5.5 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 5.6 A test proves an infra failure leaves the pact reclaimable after lease expiry (at-least-once), and that no breach is settled on infra failure
