# Pacta Tianheng Law Projection

This file is generated from `constitution()` in `crates/pacta-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p pacta-governance law_projection_is_fresh`.

# Constitution: pacta

## Static boundaries

### `pacta-contract` (crate)

> pacta-contract is the isolated core contract. Its normal dependencies are limited to serde and uuid, never another workspace crate or runtime framework.

- **rule**: restrict dependencies to (only: serde, uuid)
- **kind**: crate · **severity**: enforce

### `pacta-executor` (crate)

> pacta-executor owns the Pacta-native execution vocabulary. Its normal dependencies are limited to pacta-contract, never drivers, adapters, backends, or external frameworks.

- **rule**: restrict dependencies to (only: pacta-contract)
- **kind**: crate · **severity**: enforce

### `pacta-driver` (crate)

> pacta-driver is mechanical runtime glue. Its normal dependencies are limited to pacta-contract and pacta-executor, never adapters, backends, or external frameworks.

- **rule**: restrict dependencies to (only: pacta-contract, pacta-executor)
- **kind**: crate · **severity**: enforce

### `pacta-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: its normal dependencies are limited to tianheng, the governance-family tooling, never a workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `pacta-memory` (crate)

> pacta-memory is a registry backend outside the core. Its normal dependencies are limited to pacta-contract and uuid, never drivers, executors, or other backends.

- **rule**: restrict dependencies to (only: pacta-contract, uuid)
- **kind**: crate · **severity**: enforce

### `pacta-conformance` (crate)

> pacta-conformance is a backend-agnostic test suite. Its normal dependencies are limited to the contract it verifies — pacta-contract (whose `async` feature carries the async binding it also exercises) — plus uuid, never a specific backend.

- **rule**: restrict dependencies to (only: pacta-contract, uuid)
- **kind**: crate · **severity**: enforce

### `pacta` (crate)

> pacta is the curated published entrypoint. Its normal dependencies are limited to pacta-contract, pacta-executor, and pacta-driver, never a backend or external framework.

- **rule**: restrict dependencies to (only: pacta-contract, pacta-executor, pacta-driver)
- **kind**: crate · **severity**: enforce

### `pacta-contract::crate` (module)

> the core contract makes no inline `std::time` `now` call (such as `SystemTime::now()` or `Instant::now()`), because time is injected at the Registry seam. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, or a `now` path taken as a value rather than called, is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate` (module)

> the core contract makes no inline `uuid` `now_v7` or `now_v1` call, even fully qualified, so it mints no identifier from the ambient clock through those constructors. Coverage is partial by nature (a time-reading uuid path outside that pair, a path taken as a value, and an `extern crate … as` rename are invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: uuid; ending_with: now_v7, now_v1; strict_external: true)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) makes an inline call into a std::io/fs/net/process path; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) makes an inline call into a std::io/fs/net/process path; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) makes an inline call into a std::io/fs/net/process path; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) makes an inline call into a std::io/fs/net/process path; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

## Semantic boundaries (signature-coupling)

### `pacta::crate` (semantic)

> the pacta facade is the compose-level surface: it must not re-export the sans-I/O kernel, which stays advanced-only and is reached through pacta-contract directly.

- **rule**: must not expose (forbidden: pacta_contract::kernel)
- **kind**: semantic · **severity**: enforce · **crate**: pacta

## Forbidden-marker boundaries

### `pacta-contract::crate::kernel` (semantic)

> the sans-I/O kernel is transient driving protocol, not durable state: no type defined in it acquires Serialize/Deserialize, by derive or by a hand-written impl whose self type resolves to it, so the kernel offers no serde path for persisting an in-flight directive or notice. Durable records (Pact, Claim, Retainer, Timestamp) carry serde; the kernel's types must not. Coverage is partial by nature: a macro-generated impl, and a hand-written impl whose self type cannot be resolved (for example one reached through a glob import), are invisible to a source scan.

- **rule**: must not acquire trait (forbidden: Serialize, Deserialize)
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

## Impl-trait boundaries

### `pacta-contract::crate::kernel` (semantic)

> the sans-I/O step-driver kernel (crate::kernel) returns named types, not existentials: no public fn or method in the module itself returns a written impl Trait. The async-exposure tooth catches only a literal async fn, but a fn -> impl Future desugars to the same runtime coloring, so this closes that hole for the written form. A boxed dyn Future return, a descendant module, and a macro-generated item are outside this tooth.

- **rule**: must not expose impl trait
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate::lifecycle` (semantic)

> the colorless lifecycle-state kernel (crate::lifecycle) is the single source both Registry bindings compose over; a return-position impl Trait (e.g. -> impl Future) would color it and let the two bindings drift, so no public fn or method in the module itself returns a written impl Trait — the RPIT analogue of the async-fn tooth. Argument-position impl Trait, a descendant module, and a macro-generated item are outside this tooth.

- **rule**: must not expose impl trait
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

## Async-exposure boundaries

### `pacta-contract::crate::kernel` (semantic)

> the sans-I/O step-driver kernel (crate::kernel) must stay runtime-agnostic: no public fn, inherent method, or trait method declaration anywhere in its subtree is an async fn, so the async sugar cannot color its surface. A written -> impl Future is the impl-Trait tooth's domain, and a macro-generated item is invisible to a source scan.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

### `pacta-contract::crate::lifecycle` (semantic)

> the colorless lifecycle-state kernel (crate::lifecycle) is the single source both the sync and async Registry bindings compose over; no public fn, inherent method, or trait method declaration anywhere in its subtree is an async fn, so the async sugar cannot color the shared semantics. A written -> impl Future is the impl-Trait tooth's domain, and a macro-generated item is invisible to a source scan.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract
