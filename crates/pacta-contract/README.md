# pacta-contract

The isolated core contract for Pacta: the durable lifecycle vocabulary and the
sans-I/O lifecycle kernel.

This crate defines the types the workspace is built on — `Pact`, `Claim`,
`Retainer`, `Timestamp`, and `Outcome` / `Settlement` — and the `Registry`
lifecycle trait (`claim`, `heartbeat`, `fulfill`, `breach`), with time injected at
its seam so the core reads no ambient clock. It also holds the advanced-tier
`kernel`: a pure state machine that decides the lifecycle through `Directive` /
`Notice`, performs no I/O, and exposes no `async fn`, so it commits to no runtime
shape.

It depends only on `serde` and `uuid`. Most consumers should depend on the
[`pacta`](https://crates.io/crates/pacta) facade instead; depend on `pacta-contract`
directly to implement a `Registry` backend or to build a custom runtime over the
kernel.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of
[Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
