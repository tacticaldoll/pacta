## Context

The workspace publishes six crates plus the unpublished `pacta-governance`. Every
publishable crate sets `readme.workspace = true`, which `cargo metadata` confirms
resolves to `../../README.md` — the workspace-root README — for all of them. So on
crates.io the six crates render one generic page each. Separately, the root README
lacks a License section and still names `Policy` (removed at the 0.1.0 freeze) as
shipped vocabulary. `release-packaging` already requires a non-empty `readme` and an
Honest Release Scope; today the letter is met (a readme resolves) while the intent
is not (it is not the crate's own, and it is no longer honest).

## Goals / Non-Goals

**Goals:**
- Each publishable crate documents itself on crates.io via a crate-local README.
- The root README states the dual license and stops naming a removed type.
- Corrections reach crates.io through a docs-only `0.1.1`.

**Non-Goals:**
- No source, public-API, behavior, or dependency change.
- No version bump, changelog entry, or publish — deferred to the mechanical
  release-finalization PR (see Decisions). Manifests stay at `0.1.0`.
- No new governance code. pacta-governance checks no packaging metadata today;
  adding a one-off readme check would be asymmetric with the rest of
  `release-packaging` (MSRV, keywords, dependency graph), which is prose- and
  tooling-verified (`cargo publish --dry-run`, CI). Recorded as a BACKLOG thread.

## Decisions

- **Crate-local readme via explicit field, not workspace inheritance.** Replace
  `readme.workspace = true` with `readme = "README.md"` in each crate. A
  workspace-inherited `readme` resolves its path relative to the workspace root
  (hence `../../README.md`); an explicit package-level `readme` resolves relative to
  the crate directory. Verified post-edit with `cargo metadata` (each `readme` must
  resolve to a path inside its own crate). *Alternative — point the workspace readme
  elsewhere:* rejected; the root README must remain the repo landing page.
- **Short, role-focused per-crate READMEs.** Each README states the crate's name,
  its one-line role, where it sits relative to the `pacta` facade, and the license —
  deliberately not duplicating the `lib.rs` rustdoc (docs.rs renders that; crates.io
  renders the README). This keeps the two from drifting.
- **Governance crate gets a README too.** `pacta-governance` (publish = false) gains
  a crate-local README and `readme = "README.md"` for repo-browsing consistency;
  because it is unpublished this does not affect crates.io.
- **The root README becomes the GitHub landing page; the crate READMEs become the
  crates.io pages.** After the switch, no crate inherits the root README, so
  crates.io honesty at 0.1.1 rides on the crate-local READMEs — above all
  `crates/pacta/README.md` (the facade, the most-viewed page). They are therefore
  authored honest from the start (no removed `Policy` vocabulary, no `Signal->Pact`
  overstated as shipped ingress), and the root README's `Policy` is corrected for
  the GitHub page. The now-orphaned `[workspace.package] readme` key is removed so it
  cannot silently re-inherit the wrong README into a future crate.
- **Policy correction aligns to the CHANGELOG.** `CHANGELOG.md` already dropped
  `Policy` from the 0.1.0 execution bullet; the root README is edited to match, so
  the two release-facing documents agree.
- **No version bump or changelog in this change.** Manifests stay at `0.1.0`. This
  change is the first content on the `release/0.1.1` track, which accumulates the
  docs plus the code-base fixes that follow. The version bump
  (`workspace.package.version` and the `[workspace.dependencies]` requirements,
  `0.1.0` → `0.1.1`), the `CHANGELOG.md` `0.1.1` entry, and the publish are a
  single, purely mechanical release-finalization PR run once all `0.1.1` content has
  landed — keeping every content PR free of release bookkeeping and the changelog a
  faithful one-shot record of the whole release.

## Risks / Trade-offs

- **A readme field resolves wrong (still root, or a missing file).** → Gate on
  `cargo metadata` showing every publishable crate's `readme` inside its own crate,
  and on `cargo publish --dry-run --workspace` packaging cleanly.
- **A docs-only version adds an immutable crates.io release.** → Accepted;
  justified by correcting a live honesty misstatement (`Policy`) that would
  otherwise persist on crates.io indefinitely, since pacta is at a deliberate
  resting point with no substantive 0.1.x work queued to carry the docs.
- **Per-crate README drifts from `lib.rs` docs over time.** → Mitigated by keeping
  READMEs short and role-scoped rather than restating the API.

## Migration Plan

This change lands via the normal ritual onto the `release/0.1.1` integration branch
(change → release/0.1.1). It does not touch `main` or tag on its own. Later
code-base fixes land onto the same `release/0.1.1`. The mechanical
release-finalization PR then does the version bump + changelog, and only after that
is `release/0.1.1` squashed to `main`, tagged `v0.1.1`, and published — each an
explicitly authorized step. Nothing on crates.io changes until then.
