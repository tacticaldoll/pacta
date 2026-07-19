# pacta

The curated entrypoint to Pacta: a thin, durable lifecycle contract you compose.

`pacta` is a pure re-export facade — it carries no logic of its own. It re-exports
the compose-level surface you need to run the lifecycle end to end: the contract
types and `Registry`, the `Executor` / `Execution` / `Middleware` execution seam with the
`Identity` / `Stack` / `Composition` composition mechanism, and the `Driver` runtime loop.
This is the recommended crate to depend on.

It also re-exports the **backend-author surface** — the `Transition` port, the colorless
`lifecycle` module (`State`, the `on_X` transition decisions, `is_claimable`, and the lease
arithmetic), and the `Uuid` identifier type the constructors require — so a legal `Registry`
backend is implementable from `pacta` alone: hold `lifecycle::State`, implement the native `claim`
selection (minting a fresh `Retainer` so authority rotates on reclaim), a `lease_millis` accessor,
and an atomic `apply` transition port, and inherit `heartbeat` / `fulfill` / `breach` / `release`
as defaults. The crate-root doctest demonstrates exactly such a legal, stateful backend — settling
a pact and proving a reclaim rotates its retainer.

Pacta ships the durable lifecycle from `Pact` onward — you bring your own runtime,
your own registry backend, and your own executor. Durable backends live outside this
workspace and prove themselves against
[`pacta-conformance`](https://crates.io/crates/pacta-conformance).

The advanced sans-I/O **step-driver** kernel (`pacta_contract::kernel`) is deliberately not
re-exported here; reach it through
[`pacta-contract`](https://crates.io/crates/pacta-contract) directly, and only to build a
custom runtime. (Only the step-driver kernel is excluded — the `lifecycle` kernel is part of
the re-exported backend-author surface above.)

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
