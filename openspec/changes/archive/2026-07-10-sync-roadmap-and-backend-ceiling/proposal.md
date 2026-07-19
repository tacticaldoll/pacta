## Why

Two changes shipped the lifecycle-persistence contract, a first in-memory backend,
and a conformance suite, but `BACKLOG.md` still describes the pre-session world: it
omits everything shipped, references an archived change as active, and lists
already-shipped conformance work as an undifferentiated candidate. More
importantly, an adversarial fidelity audit found the repo faithful to its thin
"lifecycle-contract, compose the rest" intent but standing at a drift cliff: no
spec states the principle capping how many backends the thin library owns. The
coverage gate would flag an *ungoverned* `pacta-sqlite`, but nothing says durable
backends belong outside the thin library, so the decision to add one rests on
memory rather than governance. This change consolidates the roadmap and pins that
principle so the next backend is a governed decision, not a silent one.

## What Changes

- Rewrite `BACKLOG.md` to reflect shipped truth: add the lifecycle-persistence
  contract (lease, lapse-through-claim, retainer rotation, heartbeat-no-revive,
  injected time), the sans-I/O kernel, retainer encapsulation, `pacta-memory`,
  `pacta-conformance`, the ambient-time governance scan, and the coverage gate to
  the baseline.
- Fix the stale line asserting product positioning is governed by an active
  change; that change is archived and its specs are synced.
- Graduate the shipped parts of the "Registry Conformance" area to baseline and
  narrow the remaining candidate to durable backends and an async `Registry`.
- Expand "Explicitly Deferred" with the deferrals recorded only in an archived
  design so far: async `Registry`, durable backends living outside the workspace,
  a public pact-ingress API, an operator-triggered lapse sweep, and runtime
  heartbeat driving.
- Record a candidate reconsideration surfaced by the audit: now that lapse exists,
  an infrastructure failure during execution could lapse-and-recover rather than
  terminally breach. Recorded only, not decided here.
- Document the existing workspace governance coverage as governed truth and pin
  the **backend ceiling** within it: the workspace stays thin — it owns the core
  contract, runtime skeleton, governance, conformance suite, and one
  dependency-free reference backend (`pacta-memory`); durable backends are
  expected to live outside and prove themselves against `pacta-conformance`. No
  new check is added: the coverage gate already fails CI for any crate lacking a
  boundary, and every boundary already requires a written justification, so a new
  backend cannot enter silently — its justification must address why the thin
  library owns it. This change documents the teeth that already exist and pins the
  principle their justification must satisfy.

## Capabilities

### Modified Capabilities
- `quality-governance`: add a requirement documenting workspace governance
  coverage (every crate governed by a justified dependency boundary; the coverage
  gate fails on a gap) and pinning the backend ceiling as the intent a new crate's
  justification must address.

## Impact

- Docs: `BACKLOG.md` rewritten to shipped truth plus the ceiling principle and one
  recorded reconsideration.
- Code: none. The coverage gate and required boundary justifications already exist;
  this change documents them in the spec rather than adding a redundant check. No
  crate, dependency, or test changes.
- Governance: adding any future workspace crate already requires a justified
  dependency boundary or the coverage gate fails, making the `pacta-sqlite`
  question a governed decision — this change pins that its justification must
  address the backend ceiling.
