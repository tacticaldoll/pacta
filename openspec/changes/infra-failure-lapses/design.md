## Context

`pacta_contract::kernel` is a pure state machine except one transition:
`(Executing, Notice::ExecutionFailed) => Settling { outcome: Breached }`. Every
other transition routes what it was given; this one fabricates `Breached` from the
*absence* of an outcome. The `lifecycle-persistence` lease/lapse machinery already
recovers a holder that stops before settling (at-least-once), but an executor that
honestly returns `Err` is routed to a terminal breach instead — so the two failure
paths (silent death vs honest report) are handled asymmetrically, and the honest
one is punished. Worklane/mirrorlane are the cautionary lineage: they modeled
disposition richly in the core (broker verbs, attempts, dead-letter), which grew too
heavy to abstract. The fix here goes the other way — the core does *less*.

## Goals / Non-Goals

**Goals:**
- The kernel fabricates no outcome; `ExecutionFailed` reaches a neutral unsettled
  terminal, leaving the claim to lapse (symmetric with silent death).
- Failure disposition is a composed concern at the existing `Middleware` seam, not a
  core decision.

**Non-Goals:**
- No retry/backoff/attempt-limit/dead-letter in the core (that is the worklane
  weight this refuses). Bounded retry stays deferred to the orchestration cluster.
- No new public trait (no revived `Policy`) and no new middleware type — disposition
  rides the existing `Middleware` (Executor→Executor).
- No version bump / changelog — deferred to the mechanical finalization step.
- No new `Directive` (no "release/abandon" instruction); recovery uses the existing
  lease-expiry lapse, accepting its latency (see Risks).

## Decisions

- **`ExecutionFailed` → a neutral unsettled terminal, not a fabricated `Breached`.**
  Add `Phase::DoneUnsettled` and `StepResult::Unsettled`; `poll()` on that phase
  yields `Directive::Idle`; `result()` yields `StepResult::Unsettled`. This is the
  minimal honest terminal a total state machine needs when execution produced no
  outcome. *Alternative — enrich the executor result / split `Notice` into
  transient-vs-terminal:* rejected as adding surface; the transient/terminal
  decision is the user's, expressed by `Ok(Breached)` (give up) vs `Err` (lapse).
- **Why keep `Notice::ExecutionFailed` + a new terminal, rather than a driver-only
  fix.** A thinner-looking option is: the reference driver returns `Err` on executor
  error and simply drops the per-step kernel, needing no new `StepResult`/`Phase`.
  Rejected, and it is actually *less* pure: the kernel accepts an `ExecutionFailed`
  notice, so a total, honest state machine must have a terminal to route it to —
  otherwise the kernel is left mid-`Executing` with no legible terminal, and any
  custom runtime driving the kernel directly (the manifested advanced-tier driving
  contract, `surface-tiers`) has no clean end after a failure. Keeping the shipped
  `Notice::ExecutionFailed` (removing it would be breaking) and giving it an honest
  terminal makes the *kernel itself* total for every runtime, not just the reference
  driver. The purity win is the kernel's completeness, so the new terminal earns its
  place.
- **Terminal is reason-free for now.** `StepResult::Unsettled` carries no cause; the
  executor error is surfaced by the driver's `Err` return, which is where the "why"
  lives. Carrying an opaque reason on the terminal is deferred until a real consumer
  (e.g. Dychwel-style operator visibility) justifies it — restraint over
  speculative observability.
- **The default disposition is the reference driver's behavior, not a shipped
  middleware.** On an executor error the driver feeds `ExecutionFailed`, settles
  nothing, and returns the executor error (unchanged surfacing). The zero-policy
  default is "surface + lapse" — the absence of a decision, not a policy. Users
  *override* by composing `Middleware`: convert `Err`→`Ok(Breached)` for fail-fast,
  or wrap a retry middleware (deferred). No reference middleware ships for the
  default because the default is "do nothing special."
- **Poison-pact bounding is out of scope, deliberately.** With no attempt limit, a
  pact that always fails infra will lapse→retry unboundedly. That is the honest
  at-least-once consequence and matches the pre-existing lapse behavior for silent
  death; bounding it needs attempt state, which is the deferred orchestration
  cluster's job (in-process `Middleware`; cross-process via opaque operational
  metadata the core never interprets). This change does not regress safety *relative
  to the silent-death path it unifies with* — for the infra-failure path specifically
  it does trade bounded (terminal breach) for unbounded (lapse/retry), a deliberate,
  disclosed trade, not a hidden one.

## Risks / Trade-offs

- **Recovery latency.** An unsettled claim is reclaimable only at lease expiry, not
  immediately. → Accepted as the thin choice; lease duration is already a
  user-owned input. An immediate-release `Directive` would be faster but is additive
  surface, rejected here.
- **Unbounded retry for a genuinely poisonous pact.** → Same exposure the shipped
  silent-death lapse already has; bounding is deferred policy, not a regression this
  introduces. Documented, not silently dropped.
- **Behavioral change for existing consumers.** Anyone relying on infra-failure →
  breach sees lapse instead. → Pre-1.0; likely a minor (0.2.0) bump. Flagged in the
  proposal Impact; the version-line decision is the finalization step's.

## Migration Plan

Lands on the active integration branch via the ritual (change → release branch),
behind both adversarial gates. The kernel enum change is compile-compatible
(`#[non_exhaustive]`), but "non-breaking" needs a caveat: the reference driver's
`StepResult` match had an `unreachable!()` wildcard, so it gains an explicit
`Unsettled` arm here (task 3.2); an external custom runtime whose `StepResult`
wildcard is `unreachable!`/a no-op will panic or silently swallow the new terminal
until it adds an arm. (The kernel rustdoc doctest is unaffected — its only wildcard
is over `Directive`, which gained no variant.) Compilation is non-breaking; behavior
at a `StepResult` wildcard is not automatic.

**Version line:** this is a behavioral change, so it likely ships as a **minor**
(`0.2.0`) rather than a `0.1.1` patch. The content accumulates on the current
integration branch regardless; the version-line and changelog are decided at the
deferred mechanical finalization step, which may re-cut the branch as `0.2.0`.
Manifests are untouched here.
