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
  (proven to fire), the facade reactions (kernel-exclusion and re-exports-only), and the
  executor orchestration-vocabulary reaction (proven to fire) are shipped.
- Middleware composition is reified: `Identity` (the empty stack), `Stack` (the closure
  property as a holdable value, proven by test), and `Composition` (a blind assembler with a
  single generic `then` and no named policy method). This ships the stack assembler previously
  deferred below as "premature ahead of real layers": a composition *mechanism* earns its place
  by its own soundness under the pattern-admission guardrail — Pacta leads with patterns and
  consumers do not gate it — distinct from concrete orchestration *policy*, which stays deferred.

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
- Further composition ergonomics around `Executor` beyond the shipped `Identity`/`Stack`/
  `Composition` mechanism (the blind assembler itself is now shipped — see Current Baseline).

Surface: execution composition. These co-arrive as a cluster so each has a client.

### Durable Backends

Shared conformance tests, backend-agnostic correctness checks, and an in-memory
`Registry` backend have shipped. What remains on this surface:

- Durable `Registry` backends such as SQLite or Postgres, living outside the
  workspace and proving themselves against `pacta-conformance`.
- An async `Registry` variant — **shipped**, as `pacta-contract`'s `async` feature. A durable backend
  that does async I/O cannot implement the synchronous `Registry`, so the async binding is a **second
  binding of the frozen contract**, not new semantics: by the claim-authority test the claim authority
  behaves identically. `AsyncRegistry` and the optional `apply_via_cas` helper live behind
  `pacta-contract`'s `async` feature (native AFIT, `Send`-agnostic); `MemoryRegistryAsync` is the
  reference backend in `pacta-memory`, over the same private store as `MemoryRegistry`;
  `pacta-conformance` has a feature-gated async runner (parity via `run_async`) and a portable
  contention check (`run_async_contention`). A sync-only consumer that does not enable `async` compiles
  none of it.
  - **Positioning:** sans-I/O purity lives in a **colorless kernel + conformance**, not in the calling
    convention. `Registry` is an I/O **port**; sync and async are two bindings of one shape — a native
    selection (`claim`) plus one transition port (`apply`) that runs a pure `lifecycle::on_X` decision
    within the backend's own atomic scope, with the four transition ops as kernel-driven defaults. The
    backend owns *how* it is atomic and *how* it is colored (async / `Send` / executor); the contract
    fixes only the decision. `apply_via_cas` is the optional compare-and-set strategy for a backend
    whose only atomic primitive is set-if-unchanged. Ordering/priority stays edge policy (the
    consumer's), never a pacta spec parameter.
  - **Scope guard:** do **not** add create / breach-reason / by-id classify to any trait (the
    claim-authority triage declined all three). No async `Executor`/`Driver` — the reference async
    runtime is not forced; a consumer brings its own loop.
  - **Reclaim-fence (a pacta contract property).** Settlement fences on **reclaim**, not expiry: a
    holder whose lease lapsed but whose pact nobody reclaimed can still `fulfill`/`breach`/`release`
    (its retainer is still current); `heartbeat` treats `now == expiry` as still-alive (`>=`). Proven by
    `pacta-conformance::late_fulfill_before_reclaim_succeeds` and the `lifecycle-persistence`
    reclaim-rotation requirement; both bindings share it and cannot drift. Coverage to pin: the
    `heartbeat` `now == expiry` boundary is not yet asserted in `pacta-conformance` (the
    release-on-lapsed case is covered by `released_pact_withheld_until_reclaimable`).
  - **Conformance is the proof.** The async runner adapts an `AsyncRegistry` into the sync suite via a
    `block_on`, running the existing scenario bodies verbatim (one scenario set, no drift — state-machine
    parity); the at-most-once invariant under real contention is the portable `run_async_contention`
    (OS threads + `block_on`, no async runtime) that any async backend runs. Correctness is self-proven
    against the reference backend, on pacta's own authority — never gated on a downstream consumer (spec:
    `product-positioning` "Correctness Is Conformance-Self-Proven"). Backend throughput is consumer-owned
    edge, not a pacta gate.
  - **Remaining: the `0.2.0` finalize.** The apply-port unification changed the sync `Registry`
    required-method set (breaking at the implementer surface), so the release is `0.1.2 → 0.2.0` (0.1.3
    skipped — no additive-only content ever mapped to it; blast radius near-zero, since durable backends
    are async and callers use a provided backend). Finalize is one mechanical PR: workspace version bump,
    `CHANGELOG.md`, publish the six crates, `release/0.2.0 → main` squash, tag. Async needs **no**
    separate publish flip — it ships inside `pacta-contract` behind the `async` feature; the published
    set is unchanged.
  - **Deferred: drop the `Send + Sync` supertrait to fully deliver "coloring is the consumer's".**
    Both `Registry` and `AsyncRegistry` declare `: Send + Sync`, requiring every backend *type* to be
    thread-shareable — a mild runtime property the contract imposes, in tension with the brand's
    "the runtime and its coloring are the consumer's". A single-threaded consumer with a deliberately
    `!Send` backend (an `Rc`-based store) cannot implement the trait today. The async *futures* are
    already `Send`-free (AFIT); this is only the backend-type supertrait. Dropping it would let the
    contention harness (`run_async_contention`, which shares an `Arc` across `std::thread`s) bound
    `Send + Sync + 'static` itself, leaving the trait coloring-free. Deferred, not done in 0.2.0:
    it is a breaking trait change with unvetted ripples (the `Driver`, the conformance runners, the
    sync binding too), and a `!Send` backend is a niche case (durable backends are almost always
    `Send`). Revisit as a deliberate change with the ripple analysis; until then the prose is precise
    ("no `Send` on the *futures*; a backend type is `Send + Sync`").

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

- Durable-authority feedback triage — recorded. Porting a durable consumer that retries
  and defers work surfaced four proposed additions to the registry contract: (1) deferred re-arm/reclaim
  on release; (2) `breach` carrying a reason plus a retained, inspectable terminal state;
  (3) a by-id status classification; (4) idempotent pact creation (ingress). Only (1) was
  accepted into the contract and shipped as `release(retainer, reclaimable_at)` — because
  honoring a reclaimable instant changes what the *claim authority* does, which a consumer
  cannot fake. (2)–(4) were **declined from the contract**: recording a breach reason,
  looking up a pact by id, and creating/persisting a pact are all consumer-backend storage
  that never changes claim behavior — a durable backend does them over its own tables (as
  the reference already creates pacts via construction-time `seeded`, not a trait method,
  and a consumer that breaches already holds the reason to persist itself). (4) also
  collides with the deliberate "no ingress API is part of the release" scope
  (`release-packaging`); the retention/inspection halves of (2) and (3) are the already-
  deferred Tribunal inspection / exhausted-pact review (see Operator Review). The governing
  line: a registry-contract operation is warranted only when it must change the claim
  authority's behavior; everything else stays backend/consumer. `Outcome` remains the
  frozen `Fulfilled | Breached`.
- Infrastructure-failure handling during execution — resolved. An infrastructure
  failure now leaves the claim unsettled so it lapses and is reclaimed (at-least-once),
  rather than being terminally breached: the kernel fabricates no `Outcome` from an
  execution failure — it reaches an unsettled terminal (`StepResult::Unsettled`) — and
  the driver settles nothing and surfaces the executor error. Disposition (retry /
  fail-fast) composes at the `Middleware` seam; the core owns the mechanism, the edge
  owns the policy. Bounded retry for a poison pact stays deferred to the orchestration
  cluster (in-process middleware; cross-process via opaque operational metadata the
  core never interprets).
- A settled pact need not persist (a pacta contract property).
  The `lifecycle::State` enum carries a `Settled` variant and the reference backend keeps
  a settled pact in its store, but a real durable backend can represent "settled" by
  **removing the row** (it becomes trivially not-claimable, and `load` of it returns
  `None`) — a valid implementation of the only guarantee settlement owes ("a settled pact
  is not claimable again"). So the contract and specs must **not assume a settled pact
  persists**: `Settled` is the reference backend's representation, not a required storage
  obligation. No contract change (removal already satisfies the guarantee); a spec-wording
  clarity item to fold in when the async-binding specs are next touched, so a backend
  author is not misled into persisting a settled state it would rather drop.
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
- ~~An async `Registry`, deferred until an async backend forces it.~~ **Forced and
  decided** (2026-07-15) — see Durable Backends: deliver as a second binding of the
  frozen contract (colorless Kernel + colored ports), spike-first plan, not urgent.
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
