## Why

Publishing to crates.io freezes the public API: after 0.1.0, adding an enum
variant, adding a struct field, or removing a type is a breaking change that costs
a major version. The current surface carries irreversible traps — every core data
struct (`Pact`, `Claim`, `Execution`) exposes public fields, and the driver enums
(`Step`, `DriverError`) have only an accidental default-closed exhaustiveness
stance — alongside one inert accreted type (`Policy`) and two vision invariants
that live only in prose. This change is the single pre-publish pass that makes the
0.1.0 surface a stable, symmetric foundation: every irreversible decision made
deliberately, every drift shaved, every permanent invariant ratcheted by the
Tianheng family, and the composition pattern executably proven — without adding
weight the vision defers.

## What Changes

- **Deliberate exhaustiveness stance on every public enum.** `Directive`, `Notice`,
  `StepResult` (advanced-tier kernel protocol), `Step` (driver-loop status), and
  `DriverError` (error enum) gain `#[non_exhaustive]` because their variant sets are
  expected to grow. `Outcome` stays closed because `Fulfilled | Breached` is a
  complete settlement binary. **BREAKING to defer.**
- **Extensible core data records.** `Pact` and `Claim` gain `#[non_exhaustive]` plus
  `Pact::new`/`Claim::new` constructors so durable records can grow additively;
  `Execution` gains `#[non_exhaustive]` as the executor's designated growth seam
  (`Execution::new` already exists). `Retainer` and `Timestamp` are already opaque
  (private field + accessors) and are unchanged. **BREAKING to defer.**
- **`Retainer` gains `PartialEq`, `Eq`, `Hash`** so durable backends can use the
  lease identity as a map key; the orphan rule makes this the contract's
  responsibility. Proven present by a compile-time trait-bound assertion.
- **Remove `Policy`** (`pacta-executor`). It is an inert vocabulary marker with no
  consumer and no validating implementation — it violates the workspace discipline
  that every user-obligation type ships with a consumer and a reference impl. The
  user-composed orchestration seam already exists (`Middleware`); removing `Policy`
  removes no capability. Its correct future form (a user-obligation trait in the
  Tower `retry::Policy` sense, co-designed with the first concrete orchestration
  middleware) is recorded in the backlog. **BREAKING**, done now while free.
- **Ratchet kernel purity with the Tianheng family.** A hunyi forbidden-marker
  boundary forbids the `crate::kernel` subtree from acquiring `Serialize`/
  `Deserialize` (the sans-I/O kernel is transient driving protocol, not durable
  state), proven to fire by a reaction test. A guibiao `must_not_call_inline`
  boundary forbids synchronous `std::io`/`fs`/`net`/`process` calls anywhere in the
  `pacta-contract` core (the kernel included), completing the sans-I/O guarantee's
  non-async half. It targets the whole crate (like the sibling ambient-time tooth,
  since the guibiao module rule governs a file-based module and the whole core is
  sans-I/O) and runs in default mode (no `strict_external()`): `std` is a sysroot
  head caught by default, as the shipped `std::time` tooth already demonstrates.
- **Name and prove the delivery pattern.** A test stacks two pass-through middleware
  to prove the `Middleware` closure property (`Executor -> Executor` composes),
  executably validating the "compose the rest" promise. A composition-governance
  scenario names the pattern: execution obligations follow Service/Layer with the
  closure property; the persistence obligation follows trait-plus-conformance.
- **Honesty fix.** Reword `Notice::ExecutionFailed`'s doc, which over-claims a
  behavioral distinction that is a recorded backlog reconsideration, not shipped
  behavior.
- **Keep, with reason:** `Settlement` (the named terminal lifecycle stage, whose
  sibling stages `Pact`/`Claim`/`Execution` are all types) and `MemoryRegistry::new`
  (idiomatic empty constructor on a reference backend) stay.

## Capabilities

### New Capabilities
<!-- none — all changes modify existing capabilities -->

### Modified Capabilities
- `contract-manifestation`: core data records (`Pact`/`Claim`/`Execution`) are
  extensible via `#[non_exhaustive]` + constructors; `Outcome` is a closed binary;
  `Retainer` is usable as a durable-backend key; durable state serializes while the
  kernel protocol does not.
- `surface-tiers`: the advanced-tier kernel protocol enums declare their evolution
  with `#[non_exhaustive]`, manifesting in the compiler the stability intent the
  tier already states in prose.
- `runtime-skeleton`: the driver's `Step` and `DriverError` declare a deliberate
  `#[non_exhaustive]` stance; the `Policy` non-behavioral scenario is retired.
- `public-facade`: `Policy` is removed from the re-exported compose-level surface.
- `composition-governance`: the `Policy` vocabulary and its orchestration-intent
  scenario are retired; the user-obligation delivery pattern (Service/Layer closure
  for execution; trait-plus-conformance for persistence) is named and governed.
- `domain-language`: `Policy` is removed from the native middleware vocabulary.
- `quality-governance`: the kernel gains a forbidden-marker reaction (no serde) and
  a no-synchronous-I/O reaction, each proven executably, extending the sans-I/O
  guarantee beyond its async-only coverage.

## Impact

- **APIs (breaking, intentional, pre-publish):** `#[non_exhaustive]` on `Directive`,
  `Notice`, `StepResult`, `Step`, `DriverError`, `Pact`, `Claim`, `Execution`; new
  `Pact::new`/`Claim::new`; new `Retainer` derives; removal of `Policy`. Downstream
  literal construction of `Pact`/`Claim` (including the facade doctest) moves to the
  constructors.
- **Governance:** two new Tianheng boundaries in `pacta-governance` (forbidden-marker
  + kernel no-I/O), plus reaction/closure tests. No new runtime dependency.
- **Docs/specs:** seven capability spec deltas; `BACKLOG.md` records the `Policy`
  trait-ization plan and the reconsiderations; `Notice::ExecutionFailed` doc reword.
- **Not in scope (deferred, by design):** concrete orchestration middleware, the
  `Policy` trait, and the `ServiceBuilder`-style stack assembler — these co-arrive
  and co-validate later; shipping any now would freeze a contract with no client.
