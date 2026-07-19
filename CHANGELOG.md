# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-07-13

A kernel behavior correction plus documentation and packaging polish. No breaking
API change — the one added enum variant is `#[non_exhaustive]`-safe.

### Changed

- **An infrastructure failure now lapses instead of terminally breaching.** The
  sans-I/O kernel no longer fabricates `Outcome::Breached` from an execution failure:
  an executor error leaves the claim unsettled, so its lease lapses and the pact is
  reclaimed (at-least-once), rather than being terminally breached. The driver settles
  nothing on an executor error and still surfaces it to the caller. Failure
  disposition (retry, fail-fast) composes at the `Middleware` seam; an executor that
  returns `Ok(Outcome::Breached)` still settles a breach.
- Each publishable crate now ships its own README, so its crates.io page documents
  that crate rather than rendering the shared workspace README.
- `pacta-contract`'s crate description no longer labels Pacta a "task runtime".

### Added

- `StepResult::Unsettled` — the kernel's terminal for a step whose execution produced
  no outcome. It is `#[non_exhaustive]`, so downstream matches are unaffected.
- Root README: a "what Pacta owns vs what you compose" composition-pattern section
  and a License section.

## [0.1.0] - 2026-07-12

First public release: the thin lifecycle foundation, not a complete durable runtime.

### Added

- **Curated facade** (`pacta`): the recommended single entrypoint — a pure
  re-export crate that composes the contract, executor, and driver surface, carries
  no logic of its own, excludes the sans-I/O kernel, and proves end-to-end lifecycle
  composition with a crate-root doctest.
- **Lifecycle contract** (`pacta-contract`): the isolated core — `Pact`, `Claim`,
  `Retainer`, `Timestamp`, `Outcome`/`Settlement`, and the `Registry` trait
  (`claim`, `heartbeat`, `fulfill`, `breach`). Depends only on `serde` and `uuid`.
- **Sans-I/O lifecycle kernel** (`pacta-contract::kernel`): a pure, time-free state
  machine (`Directive`/`Notice`/`Kernel`/`StepResult`) that decides the lifecycle
  and exposes no `async fn`, so it commits to no runtime shape.
- **Lease and lapse semantics**: claims carry a bounded lease; an expired lease is
  reclaimed through the normal claim path with a rotated retainer, so a stale holder
  cannot settle. Time is injected (`now: Timestamp`); the core reads no ambient clock.
- **Execution composition** (`pacta-executor`): `Executor`, `Execution`, and
  `Middleware` — the Tower `Service`/`Layer` shape narrowed to the lifecycle, with
  no orchestration behavior baked in. `Middleware` composes `Executor` into
  `Executor` (the closure property), proven by test.
- **Runtime driver** (`pacta-driver`): a mechanical loop that performs the kernel's
  directives against a `Registry` and `Executor`, injecting the wall clock at the
  runtime edge. Public errors implement `std::error::Error` with source chaining.
- **In-memory reference backend** (`pacta-memory`): a dependency-free `Registry`
  implementation with real lease/lapse semantics; the reference other backends
  calibrate against.
- **Conformance suite** (`pacta-conformance`): backend-agnostic behavior tests any
  `Registry` implementation must pass. Durable backends live outside this workspace
  and prove themselves against this suite.
- **Executable governance** (`pacta-governance`, not published): a Tianheng
  constitution enforcing dependency boundaries, the kernel's async-freedom, the
  core's no-ambient-clock and no-synchronous-I/O rules, the kernel's no-serde rule
  (transient protocol is not durable state), the facade's kernel-exclusion and
  re-exports-only shape, and active-prose drift.
- **Frozen public surface**: a deliberate exhaustiveness/extensibility stance for
  0.1.0 — growing enums (`Directive`/`Notice`/`StepResult`/`Step`/`DriverError`) and
  the extensible records `Pact`/`Claim`/`Execution` are `#[non_exhaustive]` (records
  gain `new` constructors); `Outcome` stays a closed settlement binary.

### Not included (deferred)

- No durable/persistent backend (SQL, Redis, etc.) — those compose in from outside
  and prove against `pacta-conformance`.
- No ingress API (`Signal -> Pact` is user-provided, not a shipped surface).
- No framework adapters (Tower, HTTP) and no retry/backoff/timeout orchestration.

[0.1.1]: https://github.com/tacticaldoll/pacta/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/pacta/releases/tag/v0.1.0
