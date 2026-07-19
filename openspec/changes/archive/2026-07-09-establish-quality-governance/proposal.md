## Why

Pacta now has a public contract vocabulary and a minimal governance crate, but
quality gates still depend on local discipline instead of non-bypassable
reactions. Establishing CI and style reactions before the next business skeleton
keeps Pacta tianheng-native: architectural shape, supply-chain policy, docs, and
Rust style fail loudly instead of drifting into prose.

## What Changes

- Add a GitHub Actions CI workflow modeled on Tianheng's reaction-first approach
  and scaled to Pacta's current crate set.
- Add supply-chain policy with `cargo-deny`, keeping resolved dependency
  advisories, licenses, bans, and sources as cargo-deny's lane.
- Strengthen `pacta-governance` as the tianheng-native architectural reaction
  gate for current and near-future crate boundaries.
- Define coding-style expectations through enforceable Rust gates rather than a
  long prose style guide:
  - formatting via `cargo fmt --all --check`
  - linting via `cargo clippy --all-targets -- -D warnings`
  - public documentation health via `RUSTDOCFLAGS="-D warnings"`
  - crate-level safety/docs attributes where appropriate
- Update development documentation and README references only where needed to
  point contributors at the checks.
- Do not introduce driver, registry backend, conformance, or Worklane-derived
  runtime behavior in this change.

## Capabilities

### New Capabilities

- `quality-governance`: Defines Pacta's CI, style, supply-chain, documentation,
  and tianheng architectural reaction requirements.

### Modified Capabilities

- None. Existing `domain-language` requirements are not changing.

## Impact

- Adds `.github/workflows/ci.yml`.
- Adds or updates root governance files such as `deny.toml` and development
  documentation.
- Updates `pacta-governance` if needed to expose a stronger architecture
  constitution.
- May add crate-level attributes to existing crates to make style/documentation
  expectations compile-time reactions.
