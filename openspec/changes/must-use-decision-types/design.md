## Context

`BACKLOG.md`'s "Exhaustiveness was frozen by role, not uniformly" entry names
`#[must_use]` on result types as safe, additive freeze work left for post-1.0. The
current API has seven enums (plus one unit-struct error) that represent a decision or
outcome a caller must not silently discard: `Outcome`, `Verdict`, `StepResult`,
`Directive`, `Notice`, `lifecycle::State` (in `pacta-contract`/`pacta-executor`),
`Step`, and `DriverError` (in `pacta-driver`). Checking each one's actual protection
today:

- `Kernel::poll -> Directive`, `Kernel::result -> Option<StepResult>`, and
  `lifecycle::on_claim -> State` already carry method-level `#[must_use]`.
- `Executor::execute -> Result<Outcome, E>` and `Driver::step -> Result<Step,
  DriverError<...>>` are already protected transitively through `std::result::Result`'s
  own `#[must_use]`.
- `Policy::decide -> Verdict` (shipped in 0.3.0) carries **no** `#[must_use]` at
  either the method or the type level — the one genuine, currently-unprotected gap.
- `NotCurrentHolder` (the `Err` of `on_heartbeat`/`on_settle`/`on_release`) is a
  zero-field unit struct: excluded from scope, since there is no information to lose
  by discarding it beyond what `Result`'s own `#[must_use]` already flags — unlike
  every other type here, which carries a real decision or status payload.

## Goals / Non-Goals

**Goals:**
- Close the `Verdict` gap.
- Move protection for all six types to the type level, so a future method returning
  one of them bare is protected without a maintainer needing to remember to add
  `#[must_use]` to that new method too.
- Confirm the change is additive: no in-workspace call site currently discards any of
  these values, so `-D warnings` (clippy, both feature states) cannot newly fail.

**Non-Goals:**
- Does not touch the kernel's public exposure (`pub mod kernel` stays exactly as
  public as it is today) — `#[must_use]` is a lint annotation, orthogonal to
  visibility, and does not bear on the separately-recorded, still-undecided
  "narrowing kernel exposure" question in `BACKLOG.md`.
- Does not add a new CI job or governance reaction — the existing clippy gates in
  `AGENTS.md`'s Definition of Done already enforce this once the attribute exists.
- Does not change any spec-level requirement or observable behavior, so this change
  carries no capability delta (see proposal.md).

## Decisions

- **Tag the type, not (only) the method.** `Directive`/`StepResult`/`State` already
  have method-level tags on their sole producing methods today; tagging the type as
  well is what actually generalizes the protection (a future kernel method, or a
  value built and passed around before being read, is covered without a second
  thought).
- **Cover all seven, not just `Verdict`.** `BACKLOG.md` names this as "`#[must_use]`
  on result types" (plural); tagging only the one type with a live gap would leave
  the other six relying on incidental protection (method tags, `Result`) that a
  future refactor could quietly lose.
- **Draw the line at real decision/status payloads, excluding `NotCurrentHolder`.**
  A zero-field unit-struct error carries nothing to lose by being discarded beyond
  what `Result` already flags, so tagging it would be attribute noise rather than a
  meaningful protection — keeping scope to types that actually carry information.
- **No capability/spec delta.** This is an internal API-hardening change with no
  observable behavior change and no new enforcement mechanism beyond the compiler
  itself once the attribute exists; it does not fit any existing spec's Scenario
  shape and does not warrant inventing one.

## Risks / Trade-offs

- [Adding `#[must_use]` could, in principle, turn a warning into a hard failure for
  some in-workspace call site under `-D warnings`] → checked in the design phase:
  every current call site of `Kernel::poll`, `Kernel::result`, `Policy::decide`,
  `Driver::step`, and `lifecycle::on_claim` already consumes the return value via
  `if let`/`match`/`assert_eq!`/field assignment
  (`crates/pacta-driver/src/lib.rs:134,151,316,337,362,383`,
  `crates/pacta-contract/src/lib.rs:734,737`, `crates/pacta-executor/src/lib.rs:454`,
  `crates/pacta-memory/src/lib.rs:88`, `crates/pacta-conformance/src/lib.rs:480,1002`).
  Confirmed empirically in the apply phase by running the full Definition of Done,
  not assumed.
- [A downstream consumer with `#![deny(warnings)]` could see a new warning surface
  after upgrading] → accepted; this is exactly the Rust ecosystem's own convention for
  `#[must_use]` (a minor/compatible addition), and the whole point of the attribute is
  to surface exactly this class of previously-silent bug.

## Migration Plan

Purely additive; no CI or workflow change. Verified in the apply phase against the
real workspace before committing.

## Open Questions

None.
