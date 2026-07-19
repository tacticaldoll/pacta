# pacta-executor

Pacta-native execution abstractions: the `Executor` role and its `Middleware`
composition seam, plus the reified composition mechanism.

`Executor` handles a claimed pact and reports an `Outcome`; `Execution` is its
input; `Middleware` decorates an executor into an executor — the Tower
`Service` / `Layer` shape narrowed to the lifecycle, with no orchestration baked in.
Orchestration such as retry, timeout, and rate limiting is deliberately deferred: it
composes onto this seam as `Middleware`, it is not built in.

Composition is reified as three values: `Identity` (the no-op middleware — the empty stack
and neutral element), `Stack<Inner, Outer>` (the closure property as a holdable value that
is itself a `Middleware`), and `Composition` (a blind assembler that accumulates `Stack`
over `Identity` through a single generic `then`, exposing no named policy method). The
order is documented and proven by an enter/exit trace: the **first** middleware added is
outermost — entered first and exited last — and the executor is innermost.

Part of [Pacta](https://github.com/tacticaldoll/pacta); most consumers depend on the
[`pacta`](https://crates.io/crates/pacta) facade rather than this crate directly.

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
