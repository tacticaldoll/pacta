## 1. Contract: time seam

- [x] 1.1 Add a pure `Timestamp` value type to `pacta-contract` with no `now()` or
  current-time constructor
- [x] 1.2 Add `now: Timestamp` to `Registry::claim` and `Registry::heartbeat`;
  leave `fulfill` and `breach` time-free
- [x] 1.3 Document lapse-through-claim and heartbeat-no-revive semantics on the
  trait so implementors know the required behavior
- [x] 1.4 Update in-crate impls and the driver to compile against the new signatures

## 2. Runtime: inject time

- [x] 2.1 Teach `Driver::step` to obtain the current time once and pass it to the
  time-dependent registry operations, leaving the kernel time-free
- [x] 2.2 Update `pacta-driver` tests and the composition example for the new
  signatures

## 3. Backend: pacta-memory

- [x] 3.1 Create the `pacta-memory` crate depending on `pacta-contract` and `uuid`
- [x] 3.2 Implement the in-memory `Registry` with real lease, lapse-through-claim,
  authority rotation, and heartbeat-no-revive semantics
- [x] 3.3 Mint a retainer only on a successful claim

## 4. Conformance: pacta-conformance

- [x] 4.1 Create the `pacta-conformance` crate depending on `pacta-contract` and
  `uuid` (the suite builds seed pacts whose ids are uuids)
- [x] 4.2 Expose the suite as a generic function taking a constructor closure
  (`make: impl Fn(seed) -> R`); define no seeding trait, so backends run it from
  their own `#[cfg(test)]` module and `pacta-conformance` stays a dev-dependency
- [x] 4.3 Implement the backend-agnostic suite covering claim, settlement, lapse,
  at-least-once safety, and heartbeat, driving time through the trait
- [x] 4.4 Run the suite against `pacta-memory` from its `#[cfg(test)]` module via
  the closure, with `pacta-conformance` as a dev-dependency only

## 5. Governance

- [x] 5.1 Add the ambient-time source scan to `pacta-governance` rejecting
  current-time constructors (`SystemTime::now`, `Instant::now`, `Uuid::now_v7`,
  `Uuid::now_v1`) inside `pacta-contract`, scoped to the core
- [x] 5.2 Add a negative test proving the scan catches both `SystemTime::now` and
  a `uuid` clock constructor, and passes when the ambient read is removed
- [x] 5.3 Add Tianheng dependency boundaries for `pacta-memory` (`pacta-contract`,
  `uuid`) and `pacta-conformance` (`pacta-contract`, `uuid`)
- [x] 5.4 Add a governance unit test asserting every workspace crate has a
  dependency boundary, since Tianheng coverage is advisory and never fails CI

## 6. Definition of Done

- [x] 6.1 `cargo build --workspace`
- [x] 6.2 `cargo test --workspace`
- [x] 6.3 `cargo clippy --all-targets -- -D warnings`
- [x] 6.4 `cargo fmt --all --check`
- [x] 6.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 6.6 `cargo deny check`
- [x] 6.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 6.8 Adversarial review passes before commit
