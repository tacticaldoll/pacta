## Context

`crates/pacta-memory/examples/durable_retry.rs` is a `fn main()` with
`assert!`/`assert_eq!` calls demonstrating durable retry via `release`. Confirmed
empirically that `cargo test --workspace --all-features` (part of the Definition
of Done) builds this example (to catch compile errors) but never executes its
`main()` — no output, no assertion runs, verified with `-v` too. The
`composition-governance` spec's claim that this demonstration "cannot silently
regress under the Definition of Done" is therefore currently false in practice.

## Goals / Non-Goals

**Goals:**
- Make the existing spec claim actually true: a regression in
  `durable_retry.rs`'s behavior fails `cargo test --workspace`.
- Zero duplication of the demonstration's logic.
- Zero change to the Definition of Done's command list in `AGENTS.md` (keep it the
  single source, unmodified).
- Keep the example runnable standalone via `cargo run --example durable_retry`,
  unchanged in behavior, for a consumer browsing the crate.

**Non-Goals:**
- Does not touch `pacta-executor`'s `TerminalReviewExecutor` or any other test
  fixture from the operator-review-pattern change.
- Does not change the demonstration's assertions or narrative — only makes it
  actually run under test.

## Decisions

- **`#[path]`-include the example from a new `tests/durable_retry.rs`, calling its
  `main()` from a `#[test]`.** Considered and rejected two alternatives:
  1. *Convert `examples/durable_retry.rs` into `tests/durable_retry.rs` directly*
     (move, don't duplicate). Rejected: this loses the file's discoverability as a
     `cargo run --example` a consumer can find and run, which is exactly what the
     spec's word "example" promises. Cargo's `examples/` and `tests/` directories
     serve different audiences (a consumer running the crate vs. the crate's own
     CI); moving loses the former.
  2. *Add a `cargo run --example durable_retry -p pacta-memory` line to the
     Definition of Done in `AGENTS.md`.* Rejected: `AGENTS.md` states the DoD list
     is deliberately "the single source of truth" for gates, restated by
     `README.md`/`docs/development-flow.md`; adding a bespoke per-example run
     command there does not generalize (a future second demonstration would need
     its own line) and is more visible/riskier to touch than a self-contained fix
     inside `pacta-memory` that the existing `cargo test --workspace` step already
     covers with no DoD wording change at all.
  The `#[path]` approach needed `fn main()` to become `pub fn main()` (verified:
  this does not change `cargo run --example durable_retry`'s behavior — visibility
  modifiers do not affect binary execution, only cross-module access, which is
  exactly what the `#[path]`-included test module needs).

## Risks / Trade-offs

- [`pub fn main()` reads slightly unusually for a binary entry point] → minor;
  the alternative (duplicating ~80 lines of demonstration logic into a separate
  test) is strictly worse for a single-source-of-truth demonstration.
- [A future contributor might not realize `tests/durable_retry.rs` is a thin
  wrapper, not a separate test suite] → mitigated by a short comment in the new
  file stating exactly what it does and why.

## Migration Plan

Purely additive; no CI/workflow change. Verified in the design phase: both `cargo
test -p pacta-memory --test durable_retry` (new) and `cargo run --example
durable_retry -p pacta-memory` (existing) pass after the change.

## Open Questions

None.
