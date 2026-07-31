# pacta-memory

An in-memory `Registry` backend for Pacta with real lease and lapse semantics.

This is a reference backend, not a durable or production one: it holds pacts in
memory, so nothing survives the process. It exists to demonstrate correct lifecycle
semantics and to calibrate against — it leases claims for a supplied duration,
reclaims lapsed pacts through the normal claim path, rotates the retainer on every
claim so a stale holder cannot settle, and reads no clock (time is injected). It
passes [`pacta-conformance`](https://crates.io/crates/pacta-conformance).

Durable backends live outside this workspace and prove themselves against the same
conformance suite.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of
[Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
