# Tasks

## 1. Contract types (`pacta-contract`)
- [x] 1.1 Add `#[non_exhaustive]` to kernel `Directive`, `Notice`, `StepResult`
- [x] 1.2 Add `#[non_exhaustive]` to `Pact` and `Claim`; add `Pact::new(id, docket, kind, clause)` and `Claim::new(pact, retainer, lease_expiry)` constructors; keep fields public for reading
- [x] 1.3 Add `PartialEq, Eq, Hash` derives to `Retainer`
- [x] 1.4 Add a compile-time assertion that `Retainer: Eq + Hash` (const trait-bound assert)
- [x] 1.5 Confirm `Outcome` stays closed (no `#[non_exhaustive]`)
- [x] 1.6 Reword `Notice::ExecutionFailed` doc so it does not over-claim a behavioral distinction that is a recorded backlog reconsideration
- [x] 1.7 Update in-crate construction sites (kernel tests) to the new constructors where they used field literals

## 2. Executor (`pacta-executor`)
- [x] 2.1 Add `#[non_exhaustive]` to `Execution` (keep `Execution::new`)
- [x] 2.2 Remove the `Policy` struct and its impl
- [x] 2.3 Remove the `policy_is_inspectable` test (and any other test referencing `Policy::new`); note `Policy` is *defined* in this crate (not re-exported here) — the facade re-export removal is task 4.1
- [x] 2.4 Update in-crate `Execution`/`Pact` construction in tests to the constructors

## 3. Driver (`pacta-driver`)
- [x] 3.1 Add `#[non_exhaustive]` to `Step` and `DriverError`
- [x] 3.2 Update in-crate construction of `Pact`/`Claim` in driver tests to the new constructors
- [x] 3.3 **(freeze-interaction)** Add wildcard arms to `pacta-driver::step()`'s cross-crate matches on `Directive` (the `poll()` match) and `StepResult` (the `result` match), which become `#[non_exhaustive]` in task 1.1 and otherwise fail to compile (E0004) from outside `pacta-contract`. Use `_ => unreachable!("driver does not handle this kernel directive; a new variant needs driver support")` — the reference driver ships with the kernel and is exhaustive over the current protocol, so the arm is genuinely unreachable at matched-version runtime while keeping a later added variant non-breaking. Confirm `cargo build --workspace` compiles.

## 3b. Backends & conformance (cross-crate construction)
- [x] 3b.1 `pacta-memory`: migrate the `Claim { ... }` literal in `claim()` to `Claim::new` (cross-crate; breaks under `#[non_exhaustive]`)
- [x] 3b.2 `pacta-conformance`: migrate the `Pact { ... }` literal in `a_pact_on` to `Pact::new`

## 4. Facade (`pacta`)
- [x] 4.1 Remove `Policy` from the `pub use pacta_executor::{...}` re-export and from the crate-root doc bullet
- [x] 4.2 Update the composition doctest to build `Pact`/`Claim` via `Pact::new`/`Claim::new` (they are now `#[non_exhaustive]`)
- [x] 4.3 Verify the doctest still imports only from `pacta` and still asserts `Step::Fulfilled`

## 5. Composition proof (closure property)
- [x] 5.1 Add a test in `pacta-executor` (where `IdentityMiddleware` already exists) that stacks two pass-through middleware over an executor and drives the composed executor to a settlement, proving `Middleware` composes `Executor -> Executor`. Do NOT place it in `crates/pacta/src/lib.rs` — the facade reexports-only scan (`FACADE_NON_REEXPORT`) flags any non-`pub use` item at crate depth 0; if a facade test is wanted it must live in `crates/pacta/tests/`

## 6. Governance teeth (`pacta-governance`)
- [x] 6.1 Add a hunyi `ForbiddenMarkerBoundary::in_crate("pacta-contract").module("crate::kernel").must_not_acquire("Serialize").and_not_acquire("Deserialize").because(...)` and wire it into the constitution
- [x] 6.2 Add a reaction test proving the no-serde boundary fires (fixture kernel deriving `Serialize` → violation) and stays clean (matching non-deriving fixture → none)
- [x] 6.3 Add guibiao `must_not_call_inline` boundaries on `pacta-contract` `module("crate")` for `std::io`, `std::fs`, `std::net`, `std::process` (default mode — sysroot heads, no `strict_external()`), with a reason stating partial coverage. Target the whole crate, not `crate::kernel`: the guibiao module rule governs a file-based module (the `kernel` module is inline and owns no file) and the whole core is sans-I/O — broader and correct, matching the ambient-time tooth's scope
- [x] 6.4 Confirm the constitution still reports all workspace crates covered and clean

## 7. Docs & backlog
- [x] 7.1 `BACKLOG.md`: record the `Policy` trait-ization plan (return as a `tower::retry::Policy`-style user-obligation trait, co-designed with the first concrete orchestration middleware); note the deferred stack-assembler; add the exhaustiveness/extensibility decisions and the two new governance teeth to the baseline
- [x] 7.2 Confirm `CHANGELOG.md` 0.1.0 entry reflects the frozen surface (non-exhaustive stance, constructors, `Policy` removal) without overstating
- [x] 7.3 **(sync-time prose)** When syncing the specs, update the `composition-governance` Purpose (top-matter, no delta mechanism) from "middleware and policy vocabulary" to "middleware and pattern vocabulary" to match the modified `Pacta-Native Composition Boundary` requirement. Leave `domain-language`'s "middleware and policy composition" (Architectural Axioms) intact: it names the orchestration-*policy concept*, which survives (it returns as a trait, per 7.1), not the removed `Policy` type. Confirm no remaining spec text references the `Policy` *type*.

## 8. Definition of Done
- [x] 8.1 `cargo build --workspace`
- [x] 8.2 `cargo test --workspace` (incl. facade doctest and closure-property test)
- [x] 8.3 `cargo clippy --all-targets -- -D warnings`
- [x] 8.4 `cargo fmt --all --check`
- [x] 8.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 8.6 `cargo deny check`
- [x] 8.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 8.8 `openspec validate freeze-the-public-api --strict`
