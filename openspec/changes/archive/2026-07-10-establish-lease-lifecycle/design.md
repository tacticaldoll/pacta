## Context

The sans-I/O kernel is shipped: the kernel never performs I/O, and the `Driver`
performs the directives it issues against a `Registry` and an `Executor`. The
`Registry` trait (`claim`/`heartbeat`/`fulfill`/`breach`) is synchronous and, so
far, only the placeholder comment `// Retainer expiry information would go here`
acknowledges expiry. `docs/domain-language.md` already lists `Lapse` as a
lifecycle term, but no spec canonicalizes it and no type or method models a
lease.

Before a durable backend or a backend-agnostic conformance suite can exist
(BACKLOG "Registry Conformance", surface: lifecycle persistence), the contract
those artifacts depend on must be pinned: what a lease is, what lapse means, how
time enters the decision, and where the boundary between mechanism and policy —
and between Pacta's guarantee and the user's obligation — sits. This change pins
that contract and nothing more.

## Goals / Non-Goals

**Goals:**
- Establish `lifecycle-persistence` as the governed contract for leasing, lapse,
  injected time, and the at-least-once-versus-idempotent obligation split.
- Canonicalize `Lapse` and the reciprocal-obligation vocabulary in
  `domain-language`.
- Keep every ownership boundary explicit so backends and conformance inherit them
  rather than re-deciding them.

**Non-Goals:**
- No code. No `Registry` trait change, no lease or time types, no `lapse` method.
- No backend (in-memory or durable), no conformance suite.
- No enforcement code for injected time; the governance check lands with the
  time-taking code in a follow-on change.
- No retry, backoff, attempt-limit, or Tribunal behavior.

## Decisions

### Decision: This change is spec-only
Pin the contract in specs and vocabulary; defer all code to follow-on changes.

Rationale: adding a `lapse` method and lease/time types now yields API that no
backend or driver path exercises, which cuts against the project's working
convention of not shipping unexercised API (applied in the composition-example
change) rather than any codified rule. The kernel's neutrality means a later change can add the
trait surface and a real in-memory backend together, exercising the API on
arrival. Pinning the contract first also lets the backend and conformance work
inherit governed boundaries instead of inventing them.

Alternative considered: land the `Registry` trait surface (lapse + lease/time
seam) in this change. Rejected — the method would ship with only trivial impls and
no exercising caller, proving little and leaving unexercised surface.

### Decision: Time is injected, not ambient — enforced by design first
The core takes the current time as an input at its seam rather than reading a wall
clock. Threading time as a value is itself the primary enforcement: expiry cannot
be decided without a supplied time, so the discipline is structural.

Rationale: this mirrors the sans-I/O kernel's stance (take inputs, perform no
ambient I/O) and makes lease behavior deterministic, which the conformance suite
needs to test lapse without real elapsed time.

Alternative considered: enforce with `hunyi`. Rejected on fact — `hunyi`'s
forbidden-marker detects a type acquiring a forbidden trait or derive, and the
rest of the family governs API-seam shape (signature-coupling, dyn, impl-trait,
async-exposure); none detect a `*::now()` call site. The correct backstop is a
bespoke source scan (the shape of the existing prose scanner in `pacta-governance`)
or clippy `disallowed-methods`, and it belongs in the change that introduces the
time-taking code so no requirement outruns its enforcement.

### Decision: Mechanism versus policy is the load-bearing boundary
The registry owns lease-expiry and lapse (mechanism). Whether, when, and how many
times a lapsed pact is re-attempted, and whether it routes to Tribunal, is policy
— user-owned through middleware or explicitly deferred.

Rationale: this keeps axiom 1 intact (the lifecycle kernel stays thin; the
registry computes no retry, backoff, routing, or priority) while still admitting
the lease/lapse that at-least-once correctness requires. Lease-expiry is a
correctness primitive, not a scheduling feature, and the spec states that line
explicitly so backends cannot drift it.

### Decision: At-least-once pairs with an idempotent-executor obligation
State the reciprocal contract as a governed requirement: Pacta guarantees
at-least-once recovery, so the user's `Executor` must be idempotent; exactly-once
stays deferred.

Rationale: at-least-once means a lapsed-then-reclaimed pact can execute twice.
That assumption is currently implicit; promoting it to a governed obligation is
the domain-language worldview ("keep user obligations user-owned") applied to the
persistence surface.

## Risks / Trade-offs

- [Spec precedes code, risking drift] → Follow-on changes implement against these
  requirements directly, and OpenSpec treats the spec as the governed truth the
  code must satisfy; the gap is intentional and bounded to the next change.
- [A lease could be read as scheduling and reintroduce orchestration] → The
  "Mechanism Not Policy" and "Lease expiry is lifecycle, not orchestration"
  requirements draw the line explicitly, and the existing deferred-behavior specs
  still forbid retry/backoff/Tribunal in the runtime.
- [The idempotency obligation is unenforceable by tooling] → It is documented as a
  user obligation and will be exercised by the conformance suite on the backend
  side (a backend must actually re-serve a lapsed pact); no tool can force a
  user's executor to be idempotent, and the spec says so plainly.

## Open Questions

- The concrete shape of the injected-time seam (a `now` parameter on the relevant
  methods versus a clock value carried on the claim) is deferred to the change
  that adds the code, where it can be chosen against a real in-memory backend.
- Whether the injected-time backstop is a bespoke source scan or clippy
  `disallowed-methods` is deferred to that same change.
