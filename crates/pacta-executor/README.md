# pacta-executor

Pacta-native execution abstractions: the `Executor` role and its `Middleware`
composition seam.

`Executor` handles a claimed pact and reports an `Outcome`; `Execution` is its
input; `Middleware` decorates an executor into an executor — the Tower
`Service` / `Layer` shape narrowed to the lifecycle, with no orchestration baked in.
Orchestration such as retry, timeout, and rate limiting is deliberately deferred: it
composes onto this seam as `Middleware`, it is not built in.

Part of [Pacta](https://github.com/tacticaldoll/pacta); most consumers depend on the
[`pacta`](https://crates.io/crates/pacta) facade rather than this crate directly.

## License

Licensed under either of
[Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
