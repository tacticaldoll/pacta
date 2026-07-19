## 1. Pin the lifecycle-persistence contract

- [x] 1.1 Author `specs/lifecycle-persistence/spec.md` with the lease model,
  lapse recovery, injected time, at-least-once-versus-idempotent obligation,
  mechanism-versus-policy boundary, and user-owned lease inputs
- [x] 1.2 Verify every requirement reads as mechanism or obligation, not
  orchestration, so no scenario implies retry, backoff, attempt limits, or
  Tribunal behavior

## 2. Canonicalize the vocabulary

- [x] 2.1 Author `specs/domain-language/spec.md` delta canonicalizing `Lease`
  (the claim-validity window, distinct from the retired "lease token") and `Lapse`
  (lifecycle recovery), and the reciprocal-obligation vocabulary (at-least-once
  recovery paired with an idempotent `Executor`)
- [x] 2.2 Confirm the delta agrees with the existing `Lapse` glossary entry and
  the retired "lease token" legacy mapping in `docs/domain-language.md`, and does
  not contradict shipped `domain-language` requirements

## 3. Validate and cross-check

- [x] 3.1 Run `openspec validate establish-lease-lifecycle --strict` and resolve
  any findings
- [x] 3.2 Cross-check against BACKLOG: this change advances the "Registry
  Conformance" area and keeps "exactly-once delivery" deferred, adding no code and
  no dependencies
- [x] 3.3 Confirm no code, crate, or dependency changed in this change (spec-only)

## 4. Definition of Done

- [x] 4.1 `cargo build --workspace`
- [x] 4.2 `cargo test --workspace`
- [x] 4.3 `cargo clippy --all-targets -- -D warnings`
- [x] 4.4 `cargo fmt --all --check`
- [x] 4.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 4.6 `cargo deny check`
- [x] 4.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 4.8 Adversarial review passes before commit
