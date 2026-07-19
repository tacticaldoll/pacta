## Context

Today the lifecycle decision is welded inline into `Driver::step`, which calls
`Registry` and `Executor` synchronously. A throwaway spike confirmed the decision
logic is already a pure state machine — `poll() -> instruction`, `on_event(report)`
— tangled only with I/O. Extracting it makes the kernel runtime-agnostic (neither
sync nor async), which is what `architecture-blueprint` ("plugin kernel", "reject
backflow") and the axioms already declare. This is convergence: identity specs are
untouched.

The spike also resolved two open questions:
- **Seam**: the kernel emits an execute instruction and consumes an outcome report,
  so `Executor`/`Middleware` stay outside the kernel; the kernel keeps only
  `Outcome`.
- **Concurrency**: in the spike a heartbeat fired while execution was outstanding,
  confirming concurrency belongs to the runtime, not the kernel.

## Goals / Non-Goals

**Goals:**
- Extract a sans-I/O lifecycle kernel: pure transitions over instructions/reports,
  no I/O, no `async fn`.
- Rewire `Driver` into the runtime loop that performs instructions via `Registry`
  and `Executor`.
- Preserve the exact decision table (zero behavior change).
- Ship enforcement with the decision: hunyi (kernel exposes no async).

**Non-Goals (deferred candidate patterns):**
- Heartbeat scheduling, retry, timeout, rate-limit, Tribunal behavior.
- Durable registry backends and a global multi-kernel manager (granularity).
- Redesigning `Registry` into an effect-performer trait.
- Async execution-composition middleware; any prose-scanning reaction.

## Decisions

1. Kernel placement — **a `kernel` module inside `pacta-contract`**.

   Correction (independent review caught this): an earlier draft justified a new
   `pacta-kernel` crate by quoting axiom 1 as "purely a data and state interface".
   That phrasing does NOT exist in the current `AGENTS.md`; it was stale memory of a
   pre-rewrite version. Axiom 1 actually reads: "Lifecycle kernel stays thin:
   `pacta-contract` owns the durable pact envelope and `Registry` lifecycle
   contract. It does not own orchestration, scheduling, routing, adapters, or
   backend business behavior."

   Under the real axiom the pure sans-I/O state machine **is** the realization of
   the `Registry` lifecycle contract that `pacta-contract` already owns; transition
   logic is not orchestration (which axiom 1 forbids). "Stays thin" argues against
   crate proliferation. So the kernel is a module in `pacta-contract`, not a new
   crate — no workspace inflation, no coverage-check churn, and it reuses the
   contract crate's existing isolation.

2. Move `Outcome`/`Settlement` into `pacta-contract`.

   They are lifecycle vocabulary (per `domain-language`), not execution mechanism.
   Moving them lets the kernel use them without a new dependency, while `Executor`/
   `Middleware` (mechanism) stay in `pacta-executor` and re-use the moved types.

3. `Registry`/`Executor` stay as traits the *runtime* uses.

   They become effect-performers the `Driver` loop calls; the kernel never calls
   them. Not redesigning `Registry` into an effect trait yet — deferred with the
   lifecycle-persistence surface.

4. Enforcement shipped with the change.

   - hunyi (new teeth): the kernel module's public API must not expose `async fn`
     — free today (no async exists), a ratchet foreclosing silent drift to an async
     kernel. (Exact hunyi DSL / module scoping read during apply.)
   - Import isolation is ALREADY enforced: `pacta-contract`'s existing Tianheng
     boundary restricts it to `serde`/`uuid`, so the kernel (living in contract)
     cannot import runtime/execution crates. No new guibiao rule is added — a
     redundant boundary would be governance bloat.
   - Governance-crate caveat: the constitution restricts `pacta-governance` to
     `["tianheng"]`. The hunyi rule must be reached via `tianheng`'s re-exports; if
     a direct `hunyi` dep is unavoidable, amend that boundary explicitly, never
     silently.

5. Zero behavior change. The fulfilled/breached/executor-error/idle table and the
   "infra error != deliberate breach" semantic are preserved, only relocated.

6. Kernel boundary vocabulary — decided now (vocabulary is governance, axiom 4).

   The sans-I/O seam is public API, so its container types must be in-world, not
   generic state-machine terms like `Effect`/`Event`. Decision: the instruction the
   kernel issues to the runtime is a **`Directive`** (variants `Claim`, `Execute`,
   `Settle`, `Idle`); the report the runtime feeds back is a **`Notice`** (variants
   `Claimed`, `Executed`, `Settled`). Rejected `Signal` — it is already the
   glossary's external trigger and would collide. Alternatives considered:
   `Mandate`/`Return`, `Writ`/`Filing`; the exact word may be adjusted in review but
   the register (contract/arbitration) and the no-collision rule are fixed here.

## Risks / Trade-offs

- Putting the kernel in `pacta-contract` grows that crate's surface. → Mitigation:
  it is pure, dependency-free logic that realizes the lifecycle contract the crate
  already owns; no new crate, no coverage churn.
- Moving `Outcome`/`Settlement` is a breaking edit across `pacta-contract`,
  `pacta-executor`, and `pacta-driver`. → Mitigation: pre-release,
  `publish = false`, no external users; done as one compiling milestone.
- Writing the hunyi rule requires reading its DSL, which the spike did not cover. →
  Mitigation: apply-time task; the async-exposure capability is documented (hunyi
  ships `async_exposure.rs`), so the essential ratchet is known to exist.
- `runtime-skeleton` scenarios shift from "driver decides" to "kernel decides,
  driver performs". → Mitigation: observable behavior is identical; the
  `composition-example` requirements are unchanged (code-only rewire).
