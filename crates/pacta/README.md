# pacta

The curated entrypoint to Pacta: a thin, durable lifecycle contract you compose.

`pacta` is a pure re-export facade — it carries no logic of its own. It re-exports
the compose-level surface you need to run the lifecycle end to end: the contract
types and `Registry`, the `Executor` / `Execution` / `Middleware` execution seam,
and the `Driver` runtime loop. This is the recommended crate to depend on.

Pacta ships the durable lifecycle from `Pact` onward — you bring your own runtime,
your own registry backend, and your own executor. Durable backends live outside this
workspace and prove themselves against
[`pacta-conformance`](https://crates.io/crates/pacta-conformance).

The sans-I/O kernel is deliberately not re-exported here; reach it through
[`pacta-contract`](https://crates.io/crates/pacta-contract) directly, and only to
build a custom runtime.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of
[Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
