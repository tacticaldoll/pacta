# pacta-memory

In-memory reference `Registry` backends for Pacta with real lease and lapse semantics.

These are reference backends, not durable or production ones: they hold pacts in
memory, so nothing survives the process. They exist to demonstrate correct lifecycle
semantics and to calibrate against — they lease claims for a supplied duration,
reclaim lapsed pacts through the normal claim path, rotate the retainer on every
claim so a stale holder cannot settle, and read no clock (time is injected).

`MemoryRegistry` implements the synchronous `Registry`. Behind the `async` feature,
`MemoryRegistryAsync` implements `AsyncRegistry` over the **same** private store, so the
two bindings share one storage and cannot drift. Both delegate every eligibility decision
and state transition to the shared, colorless `pacta_contract::lifecycle` kernel, and their
`apply` locates the record held by the retainer — as a durable backend loads its row by
holder — rather than trusting the transition to police authority. They pass
[`pacta-conformance`](https://crates.io/crates/pacta-conformance), including its concurrent
claim and settlement contention checks.

Durable backends live outside this workspace and prove themselves against the same
conformance suite.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
