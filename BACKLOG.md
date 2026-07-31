# Backlog & Deferred Decisions

This file records deferred decisions and candidate patterns. It is not a phase
roadmap and does not create implementation commitments. Shipped truth lives in
`openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- Core contract vocabulary and the `Registry` lifecycle authority are shipped.
- Pacta-native executor, middleware, policy, and driver skeletons are shipped.
- The sans-I/O lifecycle kernel is shipped: a pure state machine that issues
  directives and absorbs notices, exposing no async and reading no clock.
- The `Retainer` proof token is encapsulated behind a constructor and accessor
  rather than a bare public field.
- The lifecycle-persistence contract is shipped: a lease model, lapse through the
  normal claim path, retainer rotation on reclaim, a heartbeat that does not
  revive a lapsed lease, and time injected into `Registry::claim` and `heartbeat`
  so the core reads no ambient clock. At-least-once recovery is paired with the
  user obligation of an idempotent `Executor`.
- `pacta-memory` is shipped: the first `Registry` backend, an in-memory,
  dependency-free reference implementation.
- `pacta-conformance` is shipped: a backend-agnostic suite that any `Registry`
  backend must pass, driven by injected time.
- Product positioning and blueprint boundaries are shipped as specs in
  `openspec/specs/`.
- CI, cargo-deny, rustdoc, clippy, fmt, Tianheng dependency boundaries, workspace
  governance coverage, active-prose governance, the kernel async-exposure
  reaction, and the ambient-time scan on the core are shipped.

## Workspace Composition

The workspace stays thin. It owns the core contract, the runtime skeleton,
governance, the conformance suite, and one dependency-free reference backend
(`pacta-memory`). Durable or production backends are expected to live outside the
workspace and prove themselves against `pacta-conformance`. Adding a workspace
crate requires a justified Tianheng boundary or the coverage gate fails, and a new
backend's justification must address why the thin library, rather than a composer,
owns it.

## Candidate Pattern Areas

These areas may become future OpenSpec changes only after their boundary and
extension surface are justified.

### Execution Composition

- Retry, timeout, rate limit, tracing, and similar orchestration behavior.
- Policy evaluation semantics.
- Composition ergonomics around `Executor`.

Surface: execution composition.

### Durable Backends

Shared conformance tests, backend-agnostic correctness checks, and an in-memory
`Registry` backend have shipped. What remains on this surface:

- Durable `Registry` backends such as SQLite or Postgres, living outside the
  workspace and proving themselves against `pacta-conformance`.
- An async `Registry` variant, once an async backend forces it.

Surface: lifecycle persistence.

### Integration Boundaries

- Framework or runtime adapters.
- Transport ingress patterns.
- External observability exports.

Surface: integration boundary.

Adapter examples are not core commitments. Compatibility work must stay outside
the core unless a future spec proves a Pacta-native boundary.

### Operator Review

- Tribunal inspection patterns.
- Manual review flows for exhausted pacts.
- Minimal operational visibility.

Surface: user-defined obligation or integration boundary, depending on the
proposal.

## Recorded Reconsiderations

- Infrastructure-failure handling during execution. Now that a lease can lapse and
  recover, an infrastructure failure could leave the claim unsettled so it lapses
  and is reclaimed, rather than being terminally breached as it is today. Recorded
  for a future proposal; not decided here.

## Explicitly Deferred

- Workflow DAGs and inter-obligation dependency graphs.
- Built-in schedulers as core behavior.
- Broad broker behavior.
- Exactly-once delivery as a core guarantee.
- Backend-specific business policy in the lifecycle kernel.
- An async `Registry`, deferred until an async backend forces it.
- Durable backends inside the workspace; they live outside it (see Workspace
  Composition).
- A public pact-ingress API that turns a Signal into a stored `Pact`.
- An operator-triggered lapse sweep; lapse stays emergent in the claim path.
- Runtime heartbeat driving; when to heartbeat is runtime and user policy.

## Prioritization

Prefer changes that preserve thinness and strengthen governance:

1. Protect the lifecycle kernel and domain vocabulary.
2. Keep user obligations user-owned.
3. Add behavior only as a governed pattern on a named extension surface.
4. Reject backflow from adapters, benchmarks, or backend convenience into core.
5. Keep the workspace thin: durable backends live outside and prove themselves
   against the conformance suite.
