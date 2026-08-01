## Context

`release-packaging`'s Release Metadata requirement already states three scenarios
(required metadata present, crate-local readme, discoverability metadata) as SHALL,
but `pacta-governance` reads no `Cargo.toml` today — enforcement is prose review plus
`cargo publish --dry-run`. `BACKLOG.md` recorded this gap but deferred it as
"asymmetric to add for one field alone," implying any fix should cover the whole
Release Metadata requirement, not just `readme` in isolation.

`pacta-governance` may depend only on `tianheng` (`GOVERNANCE_REASON` in
`crates/pacta-governance/src/main.rs`), so this cannot pull in a TOML-parsing crate.
Every existing reaction (`check_changelog_footer_links`, `check_facade_reexports_only`,
etc.) is a deliberate line scan for the same reason — this change follows that
established shape rather than introducing a new parsing approach.

Inspecting the current manifests: `description` and `readme` are the only two
`[package]` fields every publishable crate declares as a crate-local literal (there is
no `workspace.package.readme`, so a crate cannot inherit a shared readme — it must
name its own file). `license`, `repository`, `keywords`, and `categories` are all
declared today as `<field>.workspace = true`. Both forms must count as satisfying the
requirement.

## Goals / Non-Goals

**Goals:**
- Verify, via `pacta-governance check`, that every publishable crate's `[package]`
  table resolves a non-empty `description`, `license`, `repository`, `readme`,
  `keywords`, and `categories` (literal or `.workspace = true`).
- Verify `readme` resolves to a file inside the crate's own directory, not the shared
  workspace-root `README.md`.
- Cover the requirement symmetrically (all listed fields), not just `readme`, per the
  asymmetry BACKLOG.md already flagged.

**Non-Goals:**
- Does not validate the Publishable Crate Set or Publishable Dependency Graph
  requirements (already a separate, unchecked concern — out of scope here).
- Does not replace `cargo publish --dry-run`, which also validates packaging concerns
  this reaction cannot (e.g. whether the graph actually resolves on crates.io).
- Does not add a TOML-parsing dependency; stays a line scan like every sibling
  reaction in this file.

## Decisions

- **Discover publishable crates by walking `crates/*/Cargo.toml`, not the workspace
  `members` array.** Every crate directory under `crates/` today has a matching
  workspace member; walking the directory avoids parsing the `members` TOML array and
  matches the existing repo layout convention.
- **Treat a crate as publishable unless its manifest contains a literal `publish =
  false` line.** The workspace declares `publish = true` at `[workspace.package]`, and
  only `pacta-governance` opts out with a literal `publish = false` — checking for that
  literal avoids parsing `publish.workspace = true` inheritance.
- **Scan only the `[package]` table.** Stop once a `[`-prefixed line other than
  `[package]` is seen, so a `description`-like key that might appear in a later table
  is never misread as the package's own field.
- **A field counts as present if the table has either `<field> = "..."` (non-empty) or
  `<field>.workspace = true`.** `description` and `readme` are additionally required to
  be genuinely non-empty literals, since neither has a `workspace.package` default to
  inherit from in this workspace — a bare `.workspace = true` for either would be
  meaningless today and is treated as absent.
- **`readme` violation reported by resolving the path.** Join the crate directory with
  the literal string, and compare its canonicalized form against the workspace root's
  `README.md`, canonicalized — this directly encodes the "not the shared workspace-root
  README" scenario rather than a string heuristic (e.g. rejecting `"../README.md"` by
  pattern would also need to reject `"../../README.md"`, symlinks, etc.).

## Risks / Trade-offs

- [Line scan cannot see a value hidden behind a build script or macro] → accepted,
  same limitation every sibling reaction in this file already carries; the file's own
  precedent states these reactions "complement review rather than replace it."
- [A future publishable crate added under `crates/` is checked automatically with no
  registration step] → this is the intended behavior (no allowlist to maintain), but
  means a new *internal, unpublished* tool crate must remember its own literal
  `publish = false`, mirroring what `pacta-governance` already does.

## Migration Plan

Purely additive: the reaction is wired into the existing `check` subcommand path
alongside the other prose/semantic reactions. The current repository state is expected
to already pass (all manifests already declare the required fields), verified in the
apply phase before this change is committed.

## Open Questions

None blocking. Extending this reaction (or a sibling one) to cover the Publishable
Dependency Graph requirement (`cargo publish --dry-run --workspace`) is a candidate for
a future, separately-scoped change — not decided here.
