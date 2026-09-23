# Spec Delta

## MODIFIED Requirements

### Requirement: Tianheng Governance Reaction
Pacta SHALL run its Tianheng architecture constitution as a CI reaction. The dependency boundaries
observe each crate's normal dependency table; a dev or build dependency is review-governed.

#### Scenario: Architecture check runs
- **WHEN** a push or pull request runs CI
- **THEN** CI runs `pacta-governance` against the workspace manifest

#### Scenario: Contract crate remains isolated
- **WHEN** `pacta-contract` gains a normal dependency outside its declared allowlist
- **THEN** the governance reaction fails

#### Scenario: Core framework leakage is rejected
- **WHEN** `pacta-contract`, `pacta-executor`, or `pacta-driver` gains an unapproved normal dependency on adapter, backend, or framework crates
- **THEN** the governance reaction fails

### Requirement: Kernel Async-Exposure Reaction
Pacta SHALL run an executable semantic reaction that keeps its sans-I/O kernels free of exposed
async — both the **step-driver kernel** (`crate::kernel`) and the **colorless lifecycle-state
kernel** (`crate::lifecycle`), each throughout its own submodules — so their runtime-agnosticism
cannot silently drift, not at one seam only. The lifecycle kernel is the single source that both
the synchronous and asynchronous `Registry` bindings compose over; an exposed `async fn` there
would colour that shared source and let the two bindings drift, so it is guarded by its own
boundary, distinct from the step-driver kernel's. The reaction observes a public `fn`, inherent
method, or trait method declaration written as an `async fn` in each kernel's subtree; a written
`-> impl Future` is the impl-trait reaction's domain, and a macro-generated item or a module nested
inside a function body is review-governed.

#### Scenario: Step-driver kernel async fn is rejected
- **WHEN** the step-driver kernel's (`crate::kernel`) public API declares an `async fn`
- **THEN** the governance reaction fails via the hunyi semantic dimension

#### Scenario: Lifecycle kernel async fn is rejected
- **WHEN** the colorless lifecycle-state kernel's (`crate::lifecycle`) public API declares an `async fn`
- **THEN** the governance reaction fails, because the lifecycle kernel must stay colorless so the sync and async bindings cannot drift

#### Scenario: A submodule async fn is rejected
- **WHEN** a submodule under either kernel declares a public `async fn`
- **THEN** the governance reaction fails, because each async-exposure boundary descends its kernel subtree

#### Scenario: Async-exposure reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the async-exposure reaction runs as part of the governance check

### Requirement: Core Reads No Ambient Time
The `pacta-contract` core SHALL NOT read an ambient wall clock. An executable
governance reaction SHALL reject an inline `std::time` `now` call and an inline
`uuid` `now_v7` or `now_v1` call in the core — including one reached through a
renamed `use` import, and a fully-qualified `uuid` constructor written without an
import — so the observable part of the injected-time discipline is enforced rather
than merely documented. A clock read through a method on a value (such as
`Instant::elapsed`), a `now` path taken as a value rather than called, a
time-reading `uuid` path outside that pair, a path reached through another crate's
re-export, and an `extern crate … as` rename are not observed and stay
review-governed.

#### Scenario: An ambient clock read in the core fails governance
- **WHEN** `pacta-contract` source makes an inline `std::time` `now` call (such as
  `SystemTime::now()`) or an inline `uuid` `now_v7` or `now_v1` call
- **THEN** the governance reaction fails

#### Scenario: An aliased ambient clock read is still caught
- **WHEN** the core reaches such a call through a renamed `use` import, such as
  `use std::time::SystemTime as Clock; Clock::now()`
- **THEN** the governance reaction still fails, because the reaction resolves the
  call's symbol path rather than matching source text

#### Scenario: A fully-qualified time-based UUID without an import is still caught
- **WHEN** the core mints a time-based identifier through a fully-qualified path
  written without importing the crate, such as `uuid::Uuid::now_v7()` with no
  `use uuid`
- **THEN** the governance reaction still fails, because the ambient-time reaction
  resolves a bare external-crate head to its declared dependency

#### Scenario: Runtime clock reads outside the core are allowed
- **WHEN** a runtime crate such as `pacta-driver` reads the current time to inject
  it into registry operations
- **THEN** the governance reaction does not reject it, because the prohibition
  scopes to the core contract

### Requirement: Kernel Is Not Serializable
Pacta SHALL enforce by an executable reaction that the sans-I/O kernel does not
acquire serialization, so the split "durable state serializes, transient driving
protocol does not" cannot silently drift. A Tianheng forbidden-marker reaction SHALL
reject the `pacta-contract` `crate::kernel` subtree acquiring `Serialize` or
`Deserialize` — whether by `#[derive]` or a hand-written `impl` — because a
`Directive` or `Notice` is a decision to be performed now, not durable state. The
reaction observes a derive or hand-written impl in kernel source whose self type it
resolves; a macro-generated impl, and a hand-written impl whose self type it cannot
resolve (for example one reached through a glob import), are review-governed. The
reaction SHALL be proven to fire.

#### Scenario: A kernel serde derive fails governance
- **WHEN** a type in the `pacta-contract` `crate::kernel` subtree acquires `Serialize` or `Deserialize` by a written derive or a hand-written impl whose self type resolves to it
- **THEN** the governance reaction fails via the hunyi forbidden-marker dimension

#### Scenario: The no-serde reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts that the forbidden-marker check reports a violation for a fixture whose kernel derives `Serialize`, and reports none for a matching fixture that does not, so the proof distinguishes a reacting boundary from one that always fires

#### Scenario: No-serde reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the kernel no-serde reaction runs as part of the governance check

### Requirement: The Core Contract Performs No Synchronous I/O
Pacta SHALL extend the sans-I/O guarantee beyond its async-only coverage with an
executable reaction that rejects inline calls into synchronous standard-library I/O
paths anywhere in the core contract crate, so the observable part of the core's
I/O-free nature — the kernel included — is enforced and not only documented. A
Tianheng `must_not_call_inline` reaction SHALL reject inline calls into `std::io`,
`std::fs`, `std::net`, and `std::process` from the `pacta-contract` crate. It targets
the whole crate (`module("crate")`), as the sibling ambient-time tooth does, because
the guibiao module rule governs a file-based module and the entire core is sans-I/O,
not the inline `kernel` module alone. These are sysroot heads caught in the default
mode, so the reaction does not use `strict_external()` (which exists only to also
catch external-crate heads). The reaction is acknowledged to be inherently partial —
I/O entry points cannot be enumerated, and a call through a method on a value (such as
`write_all` on a writer) or macro-expanded I/O such as `println!` is not seen by a
source scan — and SHALL state that partiality in its reason, complementing rather than
replacing review. The reaction SHALL be proven to fire by a reaction test, so a
misconfigured or silently no-op boundary — a mistyped prefix, a wrong module target —
cannot pass forever behind a clean workspace.

#### Scenario: A synchronous I/O call in the core fails governance
- **WHEN** any code in the `pacta-contract` core crate, the kernel included, makes an inline call into a `std::io`, `std::fs`, `std::net`, or `std::process` path
- **THEN** the governance reaction fails, because the sans-I/O core performs no I/O

#### Scenario: Runtime I/O outside the core is allowed
- **WHEN** a runtime crate such as `pacta-driver` performs I/O
- **THEN** the governance reaction does not reject it, because the no-I/O prohibition scopes to the `pacta-contract` core crate

#### Scenario: The no-I/O reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts that the static check reports a `must not call inline` violation for a fixture whose `pacta-contract` calls into each of `std::io`, `std::fs`, `std::net`, and `std::process`, so the proof distinguishes each reacting boundary from one that silently never fires

### Requirement: Executor Orchestration-Vocabulary Reaction
Pacta SHALL enforce by an executable reaction that `pacta-executor` exposes no orchestration or
policy vocabulary as a public symbol, so the composition seam stays a blind mechanism and cannot
accrete sibling-owned or non-goal policy under a public name. A reaction SHALL reject a public
item of `pacta-executor` whose name denotes retry, timeout, backoff, circuit, quota, or
rate-limit. The forbidden list SHALL be generic orchestration vocabulary drawn from the stated
non-goals and SHALL name no sibling product, because sibling-blindness forbids the reaction from
naming what it checks against. The reaction is acknowledged to be inherently partial — a line
scan judges only a line that begins with `pub ` and defines a `fn`, `struct`, `enum`, `trait`,
`type`, `static`, `union`, `mod`, or `macro`, so it sees no macro-expanded item and skips `pub use`
re-exports, `pub const` values, trait method declarations, and `pub` fields — and so complements
review rather than replacing it. The reaction SHALL be proven to fire, so a
misconfigured or silently no-op boundary cannot pass forever.

#### Scenario: A public orchestration-named symbol in the executor fails governance
- **WHEN** `pacta-executor` source declares a public item definition (`fn`, `struct`, `enum`,
  `trait`, `type`, `static`, `union`, `mod`, or `macro`) whose name denotes retry, timeout,
  backoff, circuit, quota, or rate-limit
- **THEN** the governance reaction fails

#### Scenario: The reaction names no sibling
- **WHEN** the reaction is declared
- **THEN** its forbidden list is generic orchestration vocabulary from the stated non-goals and
  references no sibling product

#### Scenario: The orchestration-vocabulary reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts the reaction reports a violation for a fixture executor exposing such a
  public symbol, and reports none for a matching fixture without it, so the proof distinguishes a
  reacting boundary from one that always fires

#### Scenario: The reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the orchestration-vocabulary reaction runs as part of the governance check

### Requirement: Colorless Kernel Exposes No Return-Position Existentials
Pacta SHALL reject a written return-position `impl Trait` (RPIT) returned by a public fn or method
in the module itself of each sans-I/O kernel —
the step-driver kernel (`crate::kernel`) and the colorless lifecycle-state kernel
(`crate::lifecycle`) — because the async-exposure guard catches only a literal `async fn`, while a
`fn -> impl Future` desugars to the same runtime coloring and would otherwise escape. The kernels
return named types, not existentials, so a Tianheng `impl_trait_boundary` SHALL forbid a written RPIT
there, closing that runtime-coloring hole for the written form. A descendant module, a boxed
`dyn Future` return, and a macro-generated item are outside the reaction and stay review-governed.
The reaction SHALL be proven to fire.

#### Scenario: A returned impl Trait in a kernel is rejected
- **WHEN** a public fn or method in the `crate::kernel` or `crate::lifecycle` module itself returns a written `impl Trait` (for example `fn drive() -> impl core::future::Future`)
- **THEN** the governance reaction fails via the hunyi impl-trait dimension, because the colorless kernel must return named types rather than an existential that could carry runtime coloring

#### Scenario: The impl-trait reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts that the semantic check reports a `must not expose impl trait` violation for a fixture whose kernel returns `impl Future`, so the proof distinguishes a reacting boundary from one that always passes
