# pacta-memory-async

The reference **async** [Pacta](https://github.com/tacticaldoll/pacta) `Registry` backend — the
asynchronous counterpart of `pacta-memory`. It holds pacts in memory (nothing survives the
process) and exists to demonstrate correct lifecycle semantics through the async binding and to
calibrate real async durable backends against.

It implements only `AsyncRegistry`'s three primitives — a native `claim` selection and the
`load` + `cas` transition port — plus the `lease_millis` accessor; the four transition
operations come from the trait's default methods, which compose over the shared
`pacta_contract::lifecycle` kernel. So its semantics are the same single source as the
synchronous `pacta-memory`: the two cannot drift.

Licensed under either of [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT](https://opensource.org/licenses/MIT) at your option.
