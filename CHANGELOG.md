# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0]

First public release: the thin lifecycle foundation, not a complete durable runtime.

### Added

- **Lifecycle contract** (`pacta-contract`): the isolated core — `Pact`, `Claim`,
  `Retainer`, `Timestamp`, `Outcome`/`Settlement`, and the `Registry` trait
  (`claim`, `heartbeat`, `fulfill`, `breach`). Depends only on `serde` and `uuid`.
- **Sans-I/O lifecycle kernel** (`pacta-contract::kernel`): a pure, time-free state
  machine (`Directive`/`Notice`/`Kernel`/`StepResult`) that decides the lifecycle
  and exposes no `async fn`, so it commits to no runtime shape.
- **Lease and lapse semantics**: claims carry a bounded lease; an expired lease is
  reclaimed through the normal claim path with a rotated retainer, so a stale holder
  cannot settle. Time is injected (`now: Timestamp`); the core reads no ambient clock.
- **Execution composition** (`pacta-executor`): `Executor`, `Execution`,
  `Middleware`, and `Policy` — Pacta-native vocabulary with no orchestration
  behavior baked in.
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
  core's no-ambient-clock rule, and active-prose drift.

### Not included (deferred)

- No durable/persistent backend (SQL, Redis, etc.) — those compose in from outside
  and prove against `pacta-conformance`.
- No ingress API (`Signal -> Pact` is user-provided, not a shipped surface).
- No framework adapters (Tower, HTTP) and no retry/backoff/timeout orchestration.

[0.1.0]: https://github.com/tacticaldoll/pacta/releases/tag/v0.1.0
