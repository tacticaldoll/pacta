## Why

Every publishable crate inherits `readme.workspace = true`, so all six resolve to
`../../README.md`: on crates.io each crate page renders the same generic workspace
README instead of documenting that crate. The workspace is dual-licensed
(`MIT OR Apache-2.0`, with both `LICENSE-MIT` and `LICENSE-APACHE` present) yet the
root README carries no License section. And the root README still lists `Policy` as
shipped execution-composition vocabulary, a type removed in the 0.1.0 freeze,
misrepresenting the shipped surface against `release-packaging`'s Honest Release
Scope. Note the split this change creates: once each crate carries its own README,
the root README becomes the GitHub landing page (no longer any crate's `readme`),
and the new crate-local READMEs — above all `crates/pacta/README.md`, the facade —
become the crates.io pages. So both must be honest: the root README's `Policy` is
corrected, and the crate-local READMEs are authored honest from the start (no
removed vocabulary, no overstated ingress). This is documentation hygiene to settle
before family work begins. It is the first content on the `release/0.1.1` track;
the version bump, changelog, and publish are a separate, purely mechanical
release-finalization PR run once all `0.1.1` content (these docs plus the code-base
fixes that follow) has landed. The immutable `0.1.0` cannot be edited in place, so
the corrections reach crates.io only at that eventual `0.1.1` publish.

## What Changes

- Add a crate-specific `README.md` to each workspace crate, and switch each crate's
  `readme.workspace = true` to `readme = "README.md"` so it resolves crate-local
  rather than to the shared workspace root.
- Add a `## License` section to the root README stating the `MIT OR Apache-2.0`
  dual license and the standard Rust contribution terms.
- Correct the stale `Policy` references in the root README to match the frozen
  0.1.0 surface (already reflected in `CHANGELOG.md`).
- Record a BACKLOG thread for future packaging-metadata governance teeth (out of
  scope here — see design).
- **Deferred to the release-finalization PR (not this change):** the workspace
  version bump `0.1.0` → `0.1.1`, the `[workspace.dependencies]` requirement bump,
  the `CHANGELOG.md` `0.1.1` entry, and the publish. Manifests stay at `0.1.0` here.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `release-packaging`: the Release Metadata requirement is tightened so each
  publishable crate's `readme` resolves to a crate-specific file rather than the
  shared workspace root.

## Impact

- Docs/packaging only — no source code, public API, behavior, or governance-code
  change. Manifests stay at version `0.1.0`.
- Files: 7 new `crates/*/README.md`; `readme` field in 7 `crates/*/Cargo.toml`; root
  `README.md` (License section + Policy correction); `BACKLOG.md` (deferral thread).
  `CHANGELOG.md` and the version bump are **not** touched here.
- crates.io: the corrected READMEs and honest surface reach crates.io only at the
  eventual `0.1.1` publish; `0.1.0` stays as published (immutable).
