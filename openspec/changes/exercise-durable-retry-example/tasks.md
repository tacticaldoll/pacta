## 1. Wire the demonstration into the Definition of Done

- [ ] 1.1 Make `crates/pacta-memory/examples/durable_retry.rs`'s `fn main()` `pub`.
- [ ] 1.2 Add `crates/pacta-memory/tests/durable_retry.rs`: `#[path]`-include the example and call its `main()` from a `#[test]`, with a short comment explaining the wrapper.

## 2. Verification

- [ ] 2.1 Confirm `cargo test -p pacta-memory --test durable_retry` runs and passes.
- [ ] 2.2 Confirm `cargo run --example durable_retry -p pacta-memory` still runs standalone, unchanged.
- [ ] 2.3 Run the full Definition of Done (`AGENTS.md`) before committing the apply-phase milestone.

## 3. Spec sync

- [ ] 3.1 No spec text change (requirement wording already stated this; only enforcement was missing) — confirm and note in the sync commit.
- [ ] 3.2 Remove `openspec/changes/exercise-durable-retry-example/`.
