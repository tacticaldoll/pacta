## Context

The workspace ships four crates. `pacta-contract` defines the pure `Registry`
state machine and the `Pact`/`Claim`/`Retainer` envelope; `pacta-executor` defines
`Executor`, `Middleware`, `Outcome`, and `Policy`; `pacta-driver` defines the
mechanical `Driver` loop. All of these are libraries — Pacta is library-first, and
a consumer supplies the process entrypoint. `pacta-governance` is the only binary,
and it is an internal CI tool.

Every composition abstraction currently gets exercised only by `#[cfg(test)]` code
inside its own crate. There is no downstream consumer that composes the crates
together through their public surface, so the "Execution is Composition" axiom is
asserted but never demonstrated. This change adds that consumer as a minimal
example.

Source anchoring (declared, not assumed):
- Delivery target comes from BACKLOG.md Phase 2 "composition skeleton (shipped)"
  and the `composition-example` delta spec in this change — not reverse-derived
  from governance.
- Orchestration behavior is explicitly deferred in BACKLOG.md, so the example must
  not demonstrate it.
- The three axioms (AGENTS.md / PROJECT.md) bound the example as invariants.

## Goals / Non-Goals

**Goals:**

- Demonstrate `Registry -> claim -> Executor (wrapped by Middleware) -> Driver
  step -> fulfill/breach` through the public API only.
- Validate the middleware seam: prove a pass-through decorator composes into the
  exact slot a future retry/timeout middleware will occupy.
- Keep the example a build target (`crates/pacta-driver/examples/compose.rs`), not
  a new crate, so it adds no governed surface and no workspace member.

**Non-Goals:**

- No retry/backoff/timeout/rate-limit behavior (deferred in BACKLOG.md).
- No Registry-side policy, delay, visibility, or clause inspection.
- No Tower/adapter, no HTTP types, no backends.
- No user-facing getting-started tutorial (README defers this to Phase 2).

## Decisions

1. Implement the example as an `examples/` target on `pacta-driver`.

   `pacta-driver` already depends (normally) on `pacta-contract` and
   `pacta-executor`, which is exactly the public surface the example needs, and it
   already carries `uuid` in dev-dependencies for building `Pact`/`Retainer`. An
   `examples/` target reuses that surface with zero new crates.

   Alternative considered: a dedicated `pacta-example` workspace member. Rejected —
   it adds a member and CI surface for no runtime value, and the composition
   consumer does not need its own crate identity yet.

2. The example defines its own trivial `Registry`, `Executor`, and pass-through
   `Middleware` in the example file.

   The identity middleware and dummy executors that exist today live under
   `#[cfg(test)]` and are not public, so the example cannot and should not reuse
   them. Re-implementing them as example-local consumers is the honest "what a
   downstream user writes" story and keeps the example self-contained.

   Alternative considered: export the test helpers from the crates. Rejected —
   that leaks test scaffolding into the public API for no consumer benefit.

3. The middleware is strictly pass-through and is the forward-compatibility seam.

   The example wraps the executor with a `Middleware` whose `wrap` returns a
   decorator that forwards `execute` unchanged. This occupies the precise slot a
   real retry/timeout middleware will later drop into, so the example proves the
   seam is axiom-correct while introducing zero behavior.

   Alternative considered: demonstrate a "real" middleware (e.g. a retry). Rejected
   — that front-runs deferred work and leaks design intent that has not been
   proposed or adversarially reviewed.

4. Constraints are encoded as spec scenarios, not just prose.

   The `composition-example` delta spec expresses "no orchestration", "Registry
   purity", "public-API only", and "no new core dependency" as testable scenarios,
   so adversarial review checks the example against declared requirements rather
   than reviewer memory.

5. Example identifiers use the Pacta domain language, not mechanical test names.

   `docs/domain-language.md`'s Engineering Boundary lists examples alongside public
   APIs and specs as a domain-language surface; only private implementation
   (including the `#[cfg(test)]` helpers `DummyExecutor`, `IdentityMiddleware`,
   `TestRegistry`) may use mechanical names. The example therefore names its
   consumer types in the contract/arbitration register. These names are the
   consumer's own types, not additions to Pacta's canonical glossary — pick
   metaphor-consistent names (e.g. a ledger-like in-memory registry, a
   witness-like pass-through middleware) that read as a downstream user's code and
   do not masquerade as first-party Pacta terms. Final names are settled at apply
   under adversarial review. `Driver` stays framed as the mechanical loop, not a
   public role, per the glossary.

   Alternative considered: reuse the existing mechanical test-helper names for
   familiarity. Rejected — it violates the declared Engineering Boundary and makes
   the most-copied surface teach the wrong vocabulary.

## Risks / Trade-offs

- The example demonstrates a skeleton with no real behavior yet, so it could read
  as trivial. → Mitigation: its value is a correctness/seam instrument, stated as
  such in the proposal; it is explicitly not a tutorial.
- Example semantic drift (someone later hand-rolls orchestration in it) is not
  caught by Tianheng (a dependency reactor) — this is behavioral, not a dependency
  or type-surface concern. → Mitigation: the spec's "no deferred behavior",
  "Registry purity", and "domain language" scenarios plus adversarial review are
  the gate; the moment the example wants to "do something real" it must become its
  own proposed change.
- Enforcement honesty: the "no deferred behavior", "Registry purity", and
  "domain language" scenarios are **review-gated, not CI-gated** — no automated
  check asserts the absence of a retry loop or the naming register. Adversarial
  review is their sole gate; the DoD gates (build/clippy/fmt/doc/deny/governance)
  only prove the example compiles clean and adds no forbidden dependency.
- `examples/` deps land in `pacta-driver` dev-dependencies, slightly growing that
  manifest. → Mitigation: only `uuid` is needed and it is already present, so no
  manifest change is expected.
