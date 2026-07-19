# pacta-contract-async

An asynchronous binding of the [Pacta](https://github.com/tacticaldoll/pacta) `Registry`
contract, for durable backends that do async I/O and cannot implement the synchronous trait.

It is a **second binding of the same frozen contract**, not new semantics: the five operations
(claim, heartbeat, fulfill, breach, release) mean exactly what they do in the synchronous
`Registry`. The lifecycle semantics are single-sourced in `pacta_contract::lifecycle`, which both
the sync and async bindings compose over, so they cannot drift.

A backend implements only a small set of primitives — a native selection (`claim`) and a
transition port (`load` the held state, then `cas` set-if-unchanged) — and inherits the four
transition operations as default methods that compose over the shared lifecycle kernel.

Licensed under either of [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT](https://opensource.org/licenses/MIT) at your option.
