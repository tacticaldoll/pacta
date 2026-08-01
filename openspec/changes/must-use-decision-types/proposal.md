## Why

`BACKLOG.md` names `#[must_use]` on result types as safe, additive freeze work left for
post-1.0. Checking the current API surface: `Verdict` (the decision type behind the
0.3.0 `Policy` trait) carries no `#[must_use]` anywhere — neither on the enum nor on
`Policy::decide` — so a consumer calling `policy.decide(attempts, &error)` and
discarding the result silently does nothing, exactly the bug class this attribute
exists to catch. `Kernel::poll`/`Kernel::result`/`lifecycle::on_claim` already carry
method-level `#[must_use]`, and `Outcome`/`Step`/`DriverError` are already protected
transitively through `Result`'s own `#[must_use]` — but none of the seven
decision/result-shaped public types carries the attribute at the type level, so a
future method returning one of them bare would not be protected unless someone
remembers to tag that method too.

## What Changes

- Add `#[must_use]` at the type level to `Outcome`, `Verdict`, `StepResult`,
  `Directive`, `Notice`, and `lifecycle::State` (all in `pacta-contract` and
  `pacta-executor`), closing the concrete `Verdict` gap and making the rest's
  protection type-level rather than dependent on each producing method remembering
  to tag itself.
- Add `#[must_use]` to `Step` and `DriverError` in `pacta-driver` for the same
  consistency.
- Does not tag `NotCurrentHolder` (a zero-field unit-struct error): it carries no
  information a caller could lose by discarding it beyond what `Result`'s own
  `#[must_use]` already flags at the call site, unlike every other type in scope
  here, which is a real decision/status value.
- No behavior change: this only adds a compiler lint that fires when one of these
  values is produced and then discarded without being read — additive per Rust's own
  semver conventions (adding `#[must_use]` to a public item is a minor/compatible
  change, not a breaking one).

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
(none — this changes no spec-level requirement or observable behavior; it adds a
compiler lint annotation to existing public types, verified by the workspace's own
`cargo clippy -- -D warnings` gate rather than by a new or changed Scenario)

## Impact

- `crates/pacta-contract/src/lib.rs`: `Outcome`, `lifecycle::State`, and (inside
  `kernel`) `Directive`, `Notice`, `StepResult`.
- `crates/pacta-executor/src/lib.rs`: `Verdict`.
- `crates/pacta-driver/src/lib.rs`: `Step`, `DriverError`.
- No CI or workflow change — the existing `cargo clippy --workspace --all-targets
  --all-features -- -D warnings` (and the `--no-default-features` variant) already
  runs; verified in the apply phase that no in-workspace call site currently discards
  one of these values (none do today), so this cannot newly fail the gate it is added
  under.
