## ADDED Requirements

### Requirement: Kernel Is Not Serializable
Pacta SHALL enforce by an executable reaction that the sans-I/O kernel does not
acquire serialization, so the split "durable state serializes, transient driving
protocol does not" cannot silently drift. A Tianheng forbidden-marker reaction SHALL
reject the `pacta-contract` `crate::kernel` subtree acquiring `Serialize` or
`Deserialize` — whether by `#[derive]` or a hand-written `impl` — because a
`Directive` or `Notice` is a decision to be performed now, not durable state. The
reaction SHALL be proven to fire.

#### Scenario: A kernel serde derive fails governance
- **WHEN** a type in the `pacta-contract` `crate::kernel` subtree acquires `Serialize` or `Deserialize`
- **THEN** the governance reaction fails via the hunyi forbidden-marker dimension

#### Scenario: The no-serde reaction is proven to fire
- **WHEN** the governance test suite runs
- **THEN** it asserts that the forbidden-marker check reports a violation for a fixture whose kernel derives `Serialize`, and reports none for a matching fixture that does not, so the proof distinguishes a reacting boundary from one that always fires

#### Scenario: No-serde reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the kernel no-serde reaction runs as part of the governance check

### Requirement: The Core Contract Performs No Synchronous I/O
Pacta SHALL extend the sans-I/O guarantee beyond its async-only coverage with an
executable reaction that rejects synchronous standard-library I/O anywhere in the
core contract crate, so the core's I/O-free nature — the kernel included — is
enforced and not only documented. A Tianheng `must_not_call_inline` reaction SHALL
reject calls into `std::io`, `std::fs`, `std::net`, and `std::process` from the
`pacta-contract` crate. It targets the whole crate (`module("crate")`), as the
sibling ambient-time tooth does, because the guibiao module rule governs a
file-based module and the entire core is sans-I/O, not the inline `kernel` module
alone. These are sysroot heads caught in the default mode, so the reaction does not
use `strict_external()` (which exists only to also catch external-crate heads). The
reaction is acknowledged to be inherently partial — I/O entry points cannot be
enumerated, and macro-expanded I/O such as `println!` is not seen by a source scan —
and SHALL state that partiality in its reason, complementing rather than replacing
review.

#### Scenario: A synchronous I/O call in the core fails governance
- **WHEN** any code in the `pacta-contract` core crate, the kernel included, calls into `std::io`, `std::fs`, `std::net`, or `std::process`
- **THEN** the governance reaction fails, because the sans-I/O core performs no I/O

#### Scenario: Runtime I/O outside the core is allowed
- **WHEN** a runtime crate such as `pacta-driver` performs I/O
- **THEN** the governance reaction does not reject it, because the no-I/O prohibition scopes to the `pacta-contract` core crate
