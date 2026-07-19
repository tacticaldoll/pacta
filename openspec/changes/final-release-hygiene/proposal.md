## Why

The final release-readiness sweep found the release docs are not single-sourced:
the `pacta` facade — the recommended published entrypoint — is missing from the
CHANGELOG, the README, and the `release-packaging` spec's publishable set; six spec
Purpose lines are stale `TBD ... Update Purpose after archive` placeholders; and
AGENTS.md's DoD still says "or archiving a change" after the archive was dissolved.
Separately, the `docs/adr/` records duplicate decisions already carried by the
living docs (AGENTS.md, PROJECT.md, the specs) and by git — the same
redundancy-with-git that retired the change archive. A decision record whose
current-state content also lives in a maintained doc is a second copy that falls out
of sync (exactly why we had to bolt a supersession note onto ADR-0002). This change
single-sources the docs and retires the ADR class before publishing 0.1.0.

## What Changes

- **Name the facade where it belongs**: add `pacta` to the CHANGELOG 0.1.0 "Added"
  list, to the README "Status (0.1.0)"/architecture framing as the entrypoint, and
  to the `release-packaging` spec's publishable crate set.
- **Fix stale placeholders**: give the six `TBD ... after archive` spec Purpose
  lines real one-line Purposes.
- **Fix AGENTS.md self-contradiction**: drop "or archiving a change" from the DoD.
- **BACKLOG baseline**: add the two facade governance reactions (kernel-exclusion,
  re-exports-only) to the shipped-governance bullet.
- **Release packaging hygiene**: ship `LICENSE-MIT`/`LICENSE-APACHE` in each
  publishable crate's tarball; add the release date to the CHANGELOG 0.1.0 header.
- **Retire ADRs (adopt git-as-provenance)**: remove `docs/adr/`; state the principle
  in AGENTS.md — decisions are recorded in git history + PR descriptions, with
  reconsiderations in BACKLOG, and there is no separate ADR file class; re-home the
  archive-dissolution and ADR-retirement rationale into BACKLOG; reword the two
  governance scenarios to cite git history only (dropping "superseded ADRs").

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `release-packaging`: the publishable crate set includes the `pacta` facade, and
  the honest-release-scope scenario names the facade as the curated entrypoint.
- `governance-prose`: the historical-prose scenario cites git history (not
  superseded ADRs), matching the retirement of the ADR class.
- `quality-governance`: same historical-prose scenario update.

## Impact

- No code change, no dependency change, no runtime or published-API change.
- Documentation/spec single-sourcing: CHANGELOG, README, AGENTS.md, BACKLOG, six
  spec Purposes, two governance scenarios, and `release-packaging` requirements.
- Removes `docs/adr/` (recoverable from git); adds LICENSE files to six crates.
- The prose-governance scan behavior is unchanged (it reads only the active-prose
  file list; `docs/adr/` was never scanned and was already absent from the Document
  Authority list).
