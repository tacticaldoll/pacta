## Context

`lifecycle-persistence` is pinned as spec but has no code: no lease type, no time
on any `Registry` method, no backend. A throwaway spike (in-memory registry plus a
generic conformance suite) validated a concrete seam before this change: injected
time as a call parameter, lapse folded into the claim path, settlement without
time, a pure `Timestamp` with no `now()`, and user-supplied lease duration. All
eleven spike conformance cases passed, including the at-least-once safety case.
This change lands that shape as real code.

## Goals / Non-Goals

**Goals:**
- Add the `Timestamp` type and the injected-time seam to `Registry`.
- Ship `pacta-memory` as the first backend and `pacta-conformance` as the
  backend-agnostic suite it passes.
- Enforce the clock-free core with an executable governance check, and govern the
  two new crates with Tianheng boundaries.

**Non-Goals:**
- No durable backend (SQLite/Postgres) — that is a later change.
- No pact ingress API (Signal to Pact to registry); the conformance suite seeds
  through a test-only hook, not a public enqueue path.
- No operator-triggered lapse sweep; lapse stays emergent in the claim path.
- No retry, backoff, attempt limits, or Tribunal behavior.

## Decisions

### Decision: Injected time is a call parameter (Option A)
`claim` and `heartbeat` take `now: Timestamp`; `fulfill` and `breach` take none.

Rationale: the spike showed this is the only shape where the conformance suite,
generic over `Registry`, controls time by passing values through the trait — no
per-backend clock knob. It mirrors the sans-I/O kernel (the runtime injects
inputs), keeps the trait signature honest about time, and lets a deployment choose
where `now` comes from (local clock, or a backend-authoritative time the runtime
reads first). Settlement needs no time because authority is checked by retainer
match, not by the clock.

Alternative considered: a `Clock` injected at construction. Rejected — it hides
time from the trait and forces the generic suite to reach a backend-specific
`set_now` hook, losing the uniformity that makes conformance cheap.

### Decision: Lapse is emergent in the claim path
`claim(now)` reclaims any pact whose lease expired without settlement and rotates
its retainer. There is no separate `lapse` method.

Rationale: the shipped `lifecycle-persistence` spec already says a lapsed pact
"becomes available to be claimed again through the normal claim path." Folding
lapse into claim is the thinnest realization and keeps a single acquisition path.
Retainer rotation on reclaim is what invalidates a stale holder, which is why
settlement needs no time.

### Decision: Heartbeat refuses to revive a lapsed lease
A heartbeat whose lease already expired is rejected; the holder must re-claim.

Rationale: reviving an expired lease would let a reclaiming holder and the
reviving holder both hold valid settlement authority. Rejecting is the safe choice
and is cheap because heartbeat already carries `now`. This bounds *settlement
authority*, not *execution*: a lapsed holder may still be running while another
reclaims and runs, which is the correct at-least-once behavior the idempotency
obligation covers — no exactly-once guarantee is implied.

### Decision: `Timestamp` is a pure value with no `now()`
The core defines the time value type but never a constructor that yields the
current time; obtaining "now" happens only in the runtime.

Rationale: this gives the governance backstop a precise bite point. Because
`hunyi` governs API-seam shape and type markers, not call sites, the backstop is a
bespoke source scan in `pacta-governance` (the shape of the existing prose
scanner) that rejects current-time constructors inside `pacta-contract`. Since the
core is allowed `uuid`, the scan must catch not only `SystemTime::now` and
`Instant::now` but also `uuid`'s clock constructors (`Uuid::now_v7`,
`Uuid::now_v1`); a `::now(`-only pattern would miss `now_v7(`. It is scoped to the
core so `pacta-driver` may still read the clock to inject it.

### Decision: The conformance seeding hook is a constructor closure, not a trait
The suite is generic over `Registry` and takes a constructor closure
(`make: impl Fn(seed) -> R`) that returns a seeded backend; it defines no seeding
trait.

Rationale: a seeding *trait* defined in `pacta-conformance` and implemented for a
backend would either force `pacta-conformance` to be a normal dependency of the
backend, or, if implemented in an integration test, break the orphan rule (both
the trait and the backend type would be foreign to the test crate). A closure
sidesteps both: the backend runs the suite from its own `#[cfg(test)]` module,
passing `|seed| MemoryRegistry::seeded(seed)`, so `pacta-conformance` stays a pure
dev-dependency. This is a refinement of the spike, which used a same-crate
`SeededRegistry` trait that would not survive the crate split as a dev-dependency.

### Decision: Two new crates, each governed
`pacta-memory` (backend) depends on `pacta-contract` and `uuid`;
`pacta-conformance` (suite) depends on `pacta-contract` and `uuid` — it builds
seed pacts, whose ids are `uuid::Uuid`, so `uuid` is in its boundary.
`pacta-memory` depends on `pacta-conformance` only as a dev-dependency to run the
suite (a Normal-scoped Tianheng boundary does not observe `[dev-dependencies]`, so
this does not trip the backend's boundary).

Tianheng coverage is advisory — it reports missing boundaries but never changes
the exit code — so a forgotten boundary would pass `check` silently. To close
that, `pacta-governance` gains a unit test asserting every workspace crate has a
dependency boundary, turning coverage from a warning into a gate.

### Decision: The delta requirements refine, not replace
The ADDED requirements refine shipped behavior without contradicting it:
"Injected Time Is A Call Parameter" sharpens the shipped "Injected Time"
requirement into a concrete signature shape, and "Runtime Injects Current Time"
adds the driver's time-injection behavior alongside the shipped "Mechanical Driver
Skeleton" scenarios, which stay true. ADDED (not MODIFIED) is therefore correct.

## Risks / Trade-offs

- [Signature change breaks in-crate `Registry` impls] → The driver tests and the
  composition example are updated in this change; the change compiles and passes
  as a whole.
- [Worker clock skew under injected time] → `now` comes from the runtime, so a
  deployment can source it from an authoritative clock; lease durations carry
  margin. Documented as a deployment concern, not solved here.
- [The seeding hook could be mistaken for a public ingress API] → It is a
  conformance/test seam only; the design states no public pact-ingress path
  exists yet, so backends and the suite do not present one.
- [A backstop that scans for `*::now()` could over- or under-match] → It scopes to
  `pacta-contract/src` and targets current-time constructors; a negative test
  proves it fails on an injected ambient read and passes when removed.

## Open Questions

- The exact `Timestamp` representation (millisecond `u64` versus a richer type) is
  an implementation detail chosen during coding; the contract only requires a pure
  value with no `now()`.
- Whether `pacta-conformance` exposes its suite as functions or as a macro is an
  ergonomics choice deferred to implementation; the requirement is only that any
  backend can run the same suite through one seeding hook.
