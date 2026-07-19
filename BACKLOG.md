# Backlog & Deferred Decisions

This file records deferred decisions and candidate patterns. It is not a phase
roadmap and does not create implementation commitments. Shipped truth lives in
`openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- Core contract vocabulary and the `Registry` lifecycle authority are shipped.
- Pacta-native executor, middleware, and driver skeletons are shipped.
- The sans-I/O lifecycle kernel is shipped: a pure state machine that issues
  directives and absorbs notices, exposing no async and reading no clock.
- The public surface is frozen for 0.1.0 with a deliberate, role-based
  exhaustiveness/extensibility stance: the kernel protocol enums
  (`Directive`/`Notice`/`StepResult`), the driver `Step`/`DriverError`, and the
  executor `Execution` are `#[non_exhaustive]` (they grow); `Outcome` stays closed
  (a complete settlement binary). The durable records `Pact` and `Claim` are
  `#[non_exhaustive]` with `new` constructors so they can gain fields additively.
- The `Retainer` proof token is encapsulated behind a constructor and accessor
  rather than a bare public field, and derives `Eq`/`Hash` so durable backends can
  key on the lease identity (compile-asserted).
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
  reaction, the ambient-time scan on the core, the core no-synchronous-I/O scan
  (`std::io`/`fs`/`net`/`process`), the kernel no-serde forbidden-marker reaction
  (proven to fire), and the facade reactions (kernel-exclusion and re-exports-only)
  are shipped. The closure property of `Middleware` (that it stacks) is proven by test.

## Workspace Composition

The workspace stays thin. It owns the core contract, the runtime skeleton,
governance, the conformance suite, one dependency-free reference backend
(`pacta-memory`), and — because the workspace publishes to crates.io — one curated
facade crate (`pacta`) that is the published entrypoint. Durable or production
backends are expected to live outside the workspace and prove themselves against
`pacta-conformance`. Adding a workspace crate requires a justified Tianheng boundary
or the coverage gate fails, and a new backend's justification must address why the
thin library, rather than a composer, owns it. Owning the published entrypoint is a
publisher concern the thin library legitimately holds: `pacta` is a pure re-export
surface, governed to carry no logic, distinct from a composer's batteries-included
convenience.

## Release Plan

0.1.0 publishes to crates.io. The publishable crates are `pacta`, `pacta-contract`,
`pacta-executor`, `pacta-driver`, `pacta-memory`, and `pacta-conformance`;
`pacta-governance` stays unpublished (an internal gate that depends on `tianheng`
and its `guibiao` coverage core).
The `pacta` facade is the curated published entrypoint; it was added only once the
workspace became publishable, because its "publisher owns the entrypoint"
justification holds only when the workspace actually publishes.

## Candidate Pattern Areas

These areas may become future OpenSpec changes only after their boundary and
extension surface are justified.

### Execution Composition

- Retry, timeout, rate limit, tracing, and similar orchestration behavior, delivered
  as `Middleware` implementations composed onto the existing seam.
- A `Policy` user-obligation trait, in the sense of `tower::retry::Policy` — a trait
  the user implements and a concrete orchestration middleware consumes. It was
  removed from 0.1.0 as an inert value type (no consumer, no reference impl) and
  must return only co-designed with its first consuming middleware, so its method
  set is validated by a real client rather than frozen ahead of one.
- A stack assembler in the sense of Tower's `ServiceBuilder`, once there are
  multiple middleware worth composing readably; premature ahead of real layers.
- Composition ergonomics around `Executor`.

Surface: execution composition. These co-arrive as a cluster so each has a client.

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
- The lifecycle kernel models no heartbeat. Its directives are `Claim`, `Execute`,
  `Settle`, and `Idle` — there is no `Heartbeat` directive — so nothing in the pure
  decision machine ever extends a lease, and the reference `Driver` cannot heartbeat
  a claim in flight. This is distinct from the deferred *cadence* question below
  (how often to heartbeat stays user/runtime policy regardless): the fork here is
  structural — whether the kernel should model *that* a heartbeat may occur (e.g. a
  `Heartbeat` directive the runtime performs on a tick, keeping cadence out of the
  core). A sans-I/O-pure version of this collides with the synchronous `Executor`
  (which cannot yield to be heartbeated mid-execution), so it pulls at the sync/async
  seam. Recorded as a future fork; not decided here.
- The kernel's public exposure. `pacta_contract::kernel` is a committed public API
  only because `pacta-driver` consumes it across the crate boundary and Rust has no
  cross-crate "friend" visibility — not because it was offered as a feature. It is
  declared the advanced tier and its driving contract is manifested. Narrowing its
  *exposure* as Pacta approaches 1.0 (e.g. feature-gating it, so a consumer who does
  not build a custom runtime never sees it) is an option — narrowing exposure, NOT
  deprecating the shipped, governed kernel. Not decided here.
- The standalone composition examples were retired. The `pacta` and `pacta-driver`
  `examples/compose.rs` targets duplicated a proof the facade `lib.rs` doctest already
  carries more strongly — the doctest runs and asserts under `cargo test`, renders on
  docs.rs, and imports only from `pacta`, whereas an `examples/` target is compile-only
  and (shipped in the tarball) is never rendered to a consumer. The composition proof is
  now the facade doctest; the core-crate composition stays proven by `pacta-driver`'s
  unit tests. Reintroducing a runnable `examples/` playground later is an option if a
  richer, non-duplicative example earns its keep; not decided here.
- The OpenSpec change archive was dissolved. `openspec/changes/archive/` held a frozen
  copy of each synced delta spec plus deliberation git already retains, so it was
  redundant against git + `openspec/specs/` + this file. The lifecycle is now
  `explore -> propose -> apply -> sync`, where sync promotes delta specs into
  `openspec/specs/` and removes the change directory — no archive. Recorded here rather
  than in a decision-record file (see below).
- Architecture-decision-record files were retired in favor of git-as-provenance. The
  `docs/adr/` records had duplicated decisions already carried by the living docs
  (`AGENTS.md`, `PROJECT.md`, the specs) and by git — the same redundancy that retired
  the change archive, plus the supersession-note maintenance tax of a second copy.
  Decision rationale now lives in git commit bodies and pull requests; reconsiderations
  live here; the living docs are the single source of truth for current state. Adopting
  a separate decision-record class again is an option if in-tree browsable provenance is
  later judged worth the duplication; not decided here.
- The `Policy` value type was removed for the 0.1.0 freeze. It was public API wired to
  nothing — an inert vocabulary marker with no consumer and no reference implementation,
  breaking the workspace discipline that every user-obligation type ships with a consumer
  and a validator. The orchestration seam users compose against already exists
  (`Middleware`, the Tower `Layer`), so removal took no capability. Its correct form is a
  user-obligation trait (Tower `retry::Policy`) that returns with its first consuming
  middleware (see Execution Composition). Reintroducing it earlier is an option only if a
  real client appears to validate its shape; not before.
- `Settlement` was kept while `Policy` was removed — deliberately, not inconsistently.
  `Settlement` is the named terminal stage of the core lifecycle (`Signal -> Pact ->
  Claim -> Execution -> Settlement`), and its sibling stages are all types; removing it
  would make the terminal stage the only one without a type. `Policy` named no lifecycle
  stage. Collapsing `Settlement` into `Outcome` at the call sites is an option if the
  alias is later judged pure noise; not decided here.
- Exhaustiveness was frozen by role, not uniformly. `Outcome` stays closed because a
  settlement is exactly `Fulfilled | Breached`; the growing enums are `#[non_exhaustive]`.
  Opening `Outcome` later would be breaking and is not anticipated. Additive freeze work
  left for post-1.0 (safe because additive): `#[must_use]` on result types, serde/derive
  additions, and `cargo-semver-checks` against the published 0.1.0 baseline from 0.1.1 on.
- Packaging metadata has no governance teeth. The `release-packaging` requirements
  (crate-specific `readme`, `keywords`/`categories`, MSRV, the version-carrying
  dependency graph) are verified by prose review and tooling (`cargo publish --dry-run`,
  CI), not by a `pacta-governance` reaction — the governance crate reads no `Cargo.toml`
  metadata today. A file-presence/metadata check (e.g. every publishable crate resolves a
  crate-local README, not the shared workspace root) could live there, consistent with the
  existing active-prose checks, so the crate-specific-readme requirement cannot silently
  regress. Deferred as asymmetric to add for one field alone; recorded for a future
  governance pass. Not decided here.
- The 0.1.1 version bump and changelog are a deferred, standalone step. Content for 0.1.1
  (per-crate READMEs and the license section first; code-base fixes to follow) lands on
  `release/0.1.1` with manifests left at `0.1.0`. The `workspace.package.version` bump, the
  `[workspace.dependencies]` requirement bump (`0.1.0` → `0.1.1`), and the `CHANGELOG.md`
  `0.1.1` entry are one purely mechanical release-finalization PR run once all 0.1.1
  content has landed — keeping every content PR free of release bookkeeping and the
  changelog a one-shot, honest record of the whole release.

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
