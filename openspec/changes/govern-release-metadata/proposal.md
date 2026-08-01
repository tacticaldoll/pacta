## Why

`release-packaging`'s Release Metadata requirement (each publishable crate carries a
non-empty `description`/`license`/`repository`/`readme`, a crate-local `readme` rather
than the shared workspace-root README, and discoverability `keywords`/`categories`) is
today verified only by prose review and `cargo publish --dry-run`. `pacta-governance`
reads no `Cargo.toml` metadata, so a crate silently regressing to the shared
workspace-root README, or losing a required field, would not fail CI. `BACKLOG.md`
already records this as a deferred governance gap, deferred only because checking one
field alone would be asymmetric against the rest of the requirement.

## What Changes

- Add a `pacta-governance` reaction that reads every publishable crate's `Cargo.toml`
  and fails if any crate resolves an empty `description`, `license`, `repository`, or
  `readme`; if a crate's `readme` resolves to the shared workspace-root README instead
  of a crate-local file; or if `keywords`/`categories` are empty.
- Wire the new reaction into `pacta-governance`'s `check` subcommand alongside the
  existing prose/semantic reactions, so CI (`cargo run -p pacta-governance -- check`)
  now fails on a release-metadata regression instead of relying on human review.
- No consumer-facing or public API change; this is additive governance tooling only.

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `release-packaging`: adds a governance-verified scenario to the existing Release
  Metadata requirement — `pacta-governance check` now enforces what was previously
  prose-review-only.

## Impact

- `crates/pacta-governance/src/main.rs`: new `check_release_metadata` reaction plus
  wiring into `main()`'s check sequence.
- `openspec/specs/release-packaging/spec.md`: new scenario under the Release Metadata
  requirement stating the governance check.
- No CI workflow change — `.github/workflows/ci.yml`'s `governance` job already runs
  `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`.
