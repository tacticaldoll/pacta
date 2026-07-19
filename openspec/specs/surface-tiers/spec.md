# surface-tiers Specification

## Purpose
TBD - created by archiving change tier-the-public-surface. Update Purpose after archive.
## Requirements
### Requirement: Stability Tiers Are Declared
Pacta SHALL declare stability tiers over its public surface, so a consumer knows
which faces are converging toward a long-term contract and which are advanced. The
declaration SHALL name a recommended tier (the `pacta` facade and the backend-author
path) and an advanced tier (`pacta_contract::kernel`, lower stability intent — its
API may evolve — but a governed, supported core surface, not a throwaway), and SHALL
state the declaration where a consumer looks: the facade documentation and the
`kernel` module. The tiers express stability *intent*; per-release guarantees remain
governed by SemVer. The recommended/advanced boundary is already partly enforced by
the existing kernel-exclusion reaction (the facade must not re-export the kernel,
governed by `public-facade`), which this requirement references rather than restates.

#### Scenario: The facade declares the tiers
- **WHEN** a consumer reads the `pacta` crate-root documentation
- **THEN** it states that the facade and the backend-author path are the recommended surface, and that `pacta_contract::kernel` is an advanced surface (lower stability intent, API may evolve) reached through `pacta-contract` directly

#### Scenario: The advanced surface names its tier
- **WHEN** a consumer reads the `pacta_contract::kernel` module documentation
- **THEN** it states that the kernel is an advanced surface with lower stability intent than the recommended tier — its API may evolve — while remaining a supported, governed core surface

### Requirement: The Advanced Tier's Driving Contract Is Manifested
Pacta SHALL document the kernel's driving contract and prove it with a doctest, so
composing a custom runtime over the advanced tier is a legible, verified extension
point rather than an undocumented consequence of the crate split.

#### Scenario: The kernel documents its driving protocol
- **WHEN** a consumer reads the `pacta_contract::kernel` documentation
- **THEN** it describes the driving loop: obtain the next `Directive` from `poll`, perform it, report back a `Notice` via `on_event`, and repeat until `result` yields a terminal `StepResult`

#### Scenario: A doctest drives the kernel to a terminal result
- **WHEN** `cargo test --workspace` runs
- **THEN** a `kernel` doctest drives one step manually — poll, perform, `on_event` — and reaches a terminal `StepResult`, failing if the driving protocol stops compiling

