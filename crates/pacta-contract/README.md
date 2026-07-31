# pacta-contract

The isolated core contract for Pacta: the durable lifecycle vocabulary, the colorless
lifecycle kernel, and the sans-I/O step-driver kernel.

This crate defines the types the workspace is built on — `Pact`, `Claim`,
`Retainer`, `Timestamp`, and `Outcome` / `Settlement` — and the `Registry`
lifecycle trait, with time injected at its seam so the core reads no ambient clock.

A backend implements three primitives — a native `claim` selection, a `lease_millis`
accessor, and an atomic `apply(retainer, transition)` transition port — and inherits
`heartbeat`, `fulfill`, `breach`, and `release` as default methods over `apply`. The
transition semantics live once in the colorless `lifecycle` module (`State`, the `on_X`
transition decisions, the `is_claimable` predicate, and the lease arithmetic), which both
bindings compose over so they cannot drift. That `lifecycle` module plus the `Transition`
port type is the backend-author surface.

Behind the `async` feature it also ships `AsyncRegistry` — the same five-op contract made
asynchronous, a second binding over the same transition port (native `async fn` in traits,
`Send`-agnostic at its futures) — and the optional `apply_via_cas` compare-and-set helper.
A sync-only consumer that does not enable `async` compiles none of it.

It also holds the advanced-tier `kernel`: a pure step-driver state machine that decides the
lifecycle through `Directive` / `Notice`, performs no I/O, and exposes no `async fn`, so it
commits to no runtime shape. (The `kernel` step-driver is distinct from the `lifecycle`
kernel: the facade re-exports `lifecycle` but excludes `kernel`.)

It depends only on `serde` and `uuid`. Most consumers should depend on the
[`pacta`](https://crates.io/crates/pacta) facade instead; depend on `pacta-contract`
directly to implement a `Registry` backend or to build a custom runtime over the
step-driver kernel.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
