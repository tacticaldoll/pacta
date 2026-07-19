# pacta-contract-async

An asynchronous binding of the [Pacta](https://github.com/tacticaldoll/pacta) `Registry`
contract, for durable backends that do async I/O and cannot implement the synchronous trait.

It is a **second binding of the same frozen contract**, not new semantics: the five operations
(claim, heartbeat, fulfill, breach, release) mean exactly what they do in the synchronous
`Registry`. The lifecycle semantics are single-sourced in `pacta_contract::lifecycle`, which both
the sync and async bindings compose over, so they cannot drift.

A backend implements only two primitives — a native selection (`claim`) and a transition port
(`apply`, which runs a pure kernel decision within the backend's own atomic scope) — plus a
`lease_millis` accessor, and inherits the four transition operations as default methods that
compose over the shared lifecycle kernel. The backend owns *how* the scope is atomic (a lock, a
transaction, a native conditional write, or compare-and-set); a compare-and-set-only backend can
delegate to the `apply_via_cas` helper.

Licensed under either of [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT](https://opensource.org/licenses/MIT) at your option.
