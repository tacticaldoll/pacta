# Pacta Tianheng Law

> Derived from the Rust `Constitution` in `crates/pacta-governance/src/main.rs`, which remains the
> executable authority. OpenSpec specs remain the durable requirements. Do not edit this projection
> by hand; regenerate it with
> `BLESS=1 cargo test -p pacta-governance accepted_law_projection_is_fresh`.

# Constitution: pacta

## Static boundaries

### `pacta-contract`

> pacta-contract is the isolated core contract. It may depend only on serde and uuid, and never on another workspace crate or runtime framework.

- **rule**: restrict dependencies to (only: serde, uuid)
- **kind**: crate · **severity**: enforce

### `pacta-executor`

> pacta-executor owns the Pacta-native execution vocabulary. It may depend only on pacta-contract, never on drivers, adapters, backends, or external frameworks.

- **rule**: restrict dependencies to (only: pacta-contract)
- **kind**: crate · **severity**: enforce

### `pacta-driver`

> pacta-driver is mechanical runtime glue. It may depend only on pacta-contract and pacta-executor, never on adapters, backends, or external frameworks.

- **rule**: restrict dependencies to (only: pacta-contract, pacta-executor)
- **kind**: crate · **severity**: enforce

### `pacta-governance`

> the governance gate must stay independent of the workspace graph it judges: it may depend only on governance-family tooling (tianheng and its guibiao coverage core), never on a workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng, guibiao)
- **kind**: crate · **severity**: enforce

### `pacta-memory`

> pacta-memory is a registry backend outside the core. It may depend only on pacta-contract and uuid, never on drivers, executors, or other backends.

- **rule**: restrict dependencies to (only: pacta-contract, uuid)
- **kind**: crate · **severity**: enforce

### `pacta-conformance`

> pacta-conformance is a backend-agnostic test suite. It may depend only on the contract it verifies — pacta-contract (whose `async` feature carries the async binding it also exercises) — plus uuid, never on a specific backend.

- **rule**: restrict dependencies to (only: pacta-contract, uuid)
- **kind**: crate · **severity**: enforce

### `pacta`

> pacta is the curated published entrypoint. It may depend only on pacta-contract, pacta-executor, and pacta-driver, never on a backend or external framework.

- **rule**: restrict dependencies to (only: pacta-contract, pacta-executor, pacta-driver)
- **kind**: crate · **severity**: enforce

### `crate`

> the core contract must read no ambient clock; time is injected at the Registry seam

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `crate`

> the core contract must not mint time-based UUIDs; identifiers carry no ambient clock

- **rule**: inline symbol path confined to module (confined_prefix: uuid; ending_with: now_v7, now_v1; strict_external: true)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `crate`

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) may call into std::io/fs/net/process; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `crate`

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) may call into std::io/fs/net/process; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `crate`

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) may call into std::io/fs/net/process; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

### `crate`

> the sans-I/O core contract performs no I/O: no code in pacta-contract (the kernel included) may call into std::io/fs/net/process; I/O lives in runtimes and backends outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: pacta-contract

## Semantic boundaries (signature-coupling)

### `crate`

> the pacta facade is the compose-level surface: it must not re-export the sans-I/O kernel, which stays advanced-only and is reached through pacta-contract directly.

- **rule**: must not expose (forbidden: pacta_contract::kernel)
- **kind**: semantic · **severity**: enforce · **crate**: pacta

## Forbidden-marker boundaries

### `crate::kernel`

> the sans-I/O kernel is transient driving protocol, not durable state: it must not acquire Serialize/Deserialize, so persisting an in-flight directive or notice can never leak into the contract. Durable records (Pact, Claim, Retainer, Timestamp) carry serde; the kernel must not.

- **rule**: must not acquire trait (forbidden: Serialize, Deserialize)
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

## Impl-trait boundaries

### `crate::kernel`

> the sans-I/O step-driver kernel (crate::kernel) returns named types, not existentials: it must expose no return-position impl Trait. The async-exposure tooth catches only a literal async fn, but a fn -> impl Future desugars to the same runtime coloring, so this closes that hole.

- **rule**: must not expose impl trait
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

### `crate::lifecycle`

> the colorless lifecycle-state kernel (crate::lifecycle) is the single source both Registry bindings compose over; a return-position impl Trait (e.g. -> impl Future) would color it and let the two bindings drift, so it exposes no impl Trait — the RPIT analogue of the async-fn tooth.

- **rule**: must not expose impl trait
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

## Async-exposure boundaries

### `crate::kernel`

> the sans-I/O step-driver kernel (crate::kernel) must stay runtime-agnostic: its public API must never expose an async fn, so no runtime shape leaks into the contract.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract

### `crate::lifecycle`

> the colorless lifecycle-state kernel (crate::lifecycle) is the single source both the sync and async Registry bindings compose over; it must never expose an async fn, so it stays colorless and the two bindings cannot drift by coloring the shared semantics.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: pacta-contract
