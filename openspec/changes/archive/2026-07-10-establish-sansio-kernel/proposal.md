## Why

The lifecycle kernel is synchronous by an undocumented default: `Registry`,
`Executor`, and `Driver` are sync traits, with the lifecycle decision welded
inline into `Driver::step`. Two architecture reviews exposed that this core
property has no decision record and under-expresses the shipped blueprint — a
"plugin kernel" that "does not commit to a runtime" and "rejects backflow of
orchestration/adapter concerns into the kernel." A validated spike shows the
decision logic is already a pure state machine; it is only tangled with I/O.

This change extracts that state machine (sans-I/O) and makes the kernel's
runtime-agnosticism enforceable, so the property stops being a silent default an
async backend could later flip by accident. It is a convergence refactor, not a
vision change: `product-positioning` and `architecture-blueprint` are untouched.

## What Changes

- Introduce a **sans-I/O lifecycle kernel** as a module in `pacta-contract`: a pure
  state machine that decides the next lifecycle instruction (claim, execute,
  settle, idle) and absorbs the runtime's reports. It performs no I/O and exposes
  no `async fn`. Boundary type names are chosen in design (decision 6), not deferred.
- Rewire `Driver` into the **runtime loop** that performs instructions using the
  existing `Registry` and `Executor` (which become effect-performers, no longer
  called by the kernel). `Executor` and `Middleware` stay on the
  execution-composition surface.
- Move `Outcome`/`Settlement` into `pacta-contract` (lifecycle vocabulary), so the
  kernel depends on nothing new.
- Preserve behavior: the fulfilled/breached/executor-error/idle decision table and
  the "infra error != deliberate breach" semantic are kept, only relocated.
- Add **enforcement so the decision is bitten, not just documented**:
  - hunyi (semantic): the kernel's public API must not expose `async fn` (new teeth).
  - import isolation needs no new rule — the kernel lives in `pacta-contract`, whose
    existing Tianheng boundary (serde/uuid only) already forbids importing
    runtime/execution crates.
- Rewrite `composition-example` to drive the kernel via instructions (code only).

Non-goals (explicitly deferred, per blueprint candidate patterns): durable
registry backends, a global multi-kernel manager and its granularity, redesigning
`Registry` into an effect-performer trait, async execution-composition middleware,
heartbeat scheduling, Tribunal behavior, and any prose-scanning governance reaction.

## Capabilities

### New Capabilities
<!-- None. The kernel realizes the Registry lifecycle contract pacta-contract
     already owns; it modifies existing mechanism specs, not identity. -->

### Modified Capabilities
- `runtime-skeleton`: the mechanical driver skeleton becomes a sans-I/O lifecycle
  kernel plus a runtime loop that performs its instructions.
- `quality-governance`: add an executable reaction (hunyi) that keeps the kernel
  async-free. Import isolation is already covered by the existing contract boundary,
  so no new import rule is added.

<!-- composition-example is NOT modified: its observable requirements
     (public-API-only, no orchestration, Registry purity) are preserved; only its
     code is rewired. That is an Impact, not a requirement change. -->

### Zero-behavior-change constraint
This refactor preserves the existing lifecycle decision table exactly
(fulfilled -> fulfill, breached -> breach, executor-error -> breach + surface,
empty -> idle). It relocates the decision from `Driver::step` into the kernel; it
adds NO orchestration behavior (no heartbeat scheduling, retry, timeout, or
Tribunal), which stay deferred.

## Impact

- **Code**: adds a kernel module in `pacta-contract`; moves `Outcome`/`Settlement`
  into `pacta-contract`; rewrites `Driver` as the runtime loop; updates
  `pacta-executor`/`pacta-driver` wiring; rewrites
  `crates/pacta-driver/examples/compose.rs`.
- **Governance**: adds a hunyi async-exposure reaction to `pacta-governance`
  (reached via `tianheng`); free today (no async exists), it prevents silent drift
  to an async kernel. Import isolation reuses the existing contract boundary.
- **Specs**: MODIFIES `runtime-skeleton` and `quality-governance`. Identity specs
  (`product-positioning`, `architecture-blueprint`) are deliberately unchanged —
  this is convergence. `composition-example` is rewired in code only.
- **Public API**: breaking (pre-release, `publish = false`, no external users).
