## Context

Pacta is already tianheng-native: `pacta-governance` declares crate boundaries
with Tianheng, and `AGENTS.md` defines an adversarial review flow. The repository
does not yet have CI, cargo-deny policy, or explicit style reactions, so those
rules currently rely on local discipline and PR memory.

Tianheng is the closest reference because it treats governance as reaction:
compiler gates, cargo-deny, self-governance tests, and projected law instead of
expanding prose. Worklane is useful for later business skeleton work, but its CI
and runtime surface are larger than Pacta needs now.

## Goals / Non-Goals

**Goals:**

- Add CI that runs Pacta's Definition of Done on push and pull request.
- Add a `pacta-governance` reaction gate to CI.
- Add cargo-deny supply-chain policy for resolved advisories, licenses, bans,
  and sources.
- Define coding style through enforceable checks: rustfmt, clippy warnings,
  rustdoc warnings, and crate attributes.
- Keep documentation concise by pointing to executable checks instead of
  duplicating their rules in prose.

**Non-Goals:**

- Add driver, registry backend, executor, conformance, or Worklane-derived
  runtime behavior.
- Add database, Redis, or service-container CI.
- Add broad style-guide prose unrelated to executable checks.
- Introduce Tianheng semantic/runtime dimensions before there is a concrete
  Pacta boundary that needs them.

## Decisions

1. Use one focused CI workflow for the current workspace.

   The workflow should run build, test, clippy, fmt, rustdoc, cargo-deny, and
   `pacta-governance`. It should not copy Tianheng's packaging/release jobs yet:
   Pacta crates are `publish = false`, and release packaging policy can be added
   when publishing becomes part of the contract.

2. Keep cargo-deny responsible for supply chain.

   Advisory, license, ban, and source policy is whole-graph resolved dependency
   governance. That belongs in `deny.toml` and CI, not in Tianheng's architecture
   constitution.

3. Keep `pacta-governance` responsible for architectural shape.

   The current constitution should continue to protect `pacta-contract` and
   `pacta-governance` isolation. It may name near-future crate boundaries in
   reason text, but should not declare boundaries for crates that do not exist.

4. Use crate attributes for Rust style where they are real compiler reactions.

   `#![forbid(unsafe_code)]` is appropriate for current Pacta crates.
   `#![warn(missing_docs)]` should be applied where public API documentation is
   meaningful, especially `pacta-contract`. Binary-only governance code may rely
   on rustdoc/clippy without forcing every private helper into public docs.

5. Keep style documentation short.

   `docs/development-flow.md` and README should list commands and point to the
   gates. They should not restate rustfmt/clippy/rustdoc rules.

## Risks / Trade-offs

- **Risk:** CI copied wholesale from Tianheng would overfit to publishable crate
  packaging and examples Pacta does not have -> Mitigation: include only gates
  that react to Pacta's current surface.
- **Risk:** `missing_docs` can slow early iteration -> Mitigation: apply it first
  to the contract crate where public API is already the project contract.
- **Risk:** Governance prose could drift from the executable constitution ->
  Mitigation: keep prose minimal and make CI run `pacta-governance`.
- **Risk:** cargo-deny may require network locally -> Mitigation: CI owns the
  gate; local contributors can run `cargo deny check` when available.

## Migration Plan

1. Add CI workflow.
2. Add `deny.toml`.
3. Strengthen crate attributes and governance checks.
4. Update concise developer documentation.
5. Run all new and existing gates locally where possible.
