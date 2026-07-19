## Why

Pacta is library-first: the runtime and composition skeleton (`Registry`,
`Executor`, `Middleware`, `Driver`) ship as crates that a downstream consumer
wires into their own binary. Today there is no consumer that exercises the public
composition path end to end, so nothing proves the skeleton actually composes as
the axioms claim, and nothing occupies the decorator slot that deferred
orchestration (retry, timeout) will later fill. A minimal example closes that gap
as a correctness instrument, not as a user tutorial.

## What Changes

- Add a minimal composition-smoke example that drives
  `Registry (pure) -> claim -> Executor -> Middleware (pass-through) -> fulfill/breach`
  using only the public API of `pacta-contract`, `pacta-executor`, and
  `pacta-driver`.
- The example is an `examples/` build target on `pacta-driver`
  (`crates/pacta-driver/examples/compose.rs`), not a new workspace member crate.
- The pass-through `Middleware` in the example occupies the exact decorator slot a
  future retry/timeout middleware will drop into, validating the seam without
  demonstrating any deferred behavior.
- No production code changes: only an example target plus, if needed, a `uuid`
  entry already present in `pacta-driver` dev-dependencies.

Non-goals (explicitly out of scope): no retry/backoff/timeout/rate-limit
behavior, no Registry-side policy, no Tower/adapter, no HTTP types, no backends,
no user-facing getting-started tutorial (README defers that to Phase 2
implementation).

## Capabilities

### New Capabilities
- `composition-example`: The workspace provides a public-API-only example that
  demonstrates end-to-end lifecycle composition through the middleware seam, and
  is constrained to introduce no orchestration behavior or Registry-side policy.

### Modified Capabilities
<!-- None. This change adds a consumer-facing example and does not alter the
     requirements of runtime-skeleton, composition-governance, domain-language,
     or quality-governance. -->

## Impact

- **Code**: adds `crates/pacta-driver/examples/compose.rs`. The example is a
  downstream consumer: it implements the public `Registry`, `Executor`, and
  `Middleware` traits with trivial, behavior-free bodies.
- **Dependencies**: example-only deps stay in `pacta-driver` dev-dependencies
  (`uuid` already present); no new normal `[dependencies]` on any core crate, so
  the Tianheng dependency boundaries are unaffected (dev-deps are exempt).
- **Governance**: the example is correctly ungoverned by Tianheng (a consumer,
  not a core crate). Its constraints are enforced by adversarial review, and it
  is still built/linted/formatted via `--all-targets`.
- **Public API**: none changed. Verified the example composes through existing
  public items only; no API gap found.
