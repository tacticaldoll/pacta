## Why

`openspec/changes/archive/` holds 22 dated change directories whose content is
already redundant: each archived delta spec is a frozen copy of what was synced
into `openspec/specs/` (the living truth), and the deliberation (proposal/design)
is preserved in git history and in `BACKLOG.md`'s Recorded Reconsiderations. The
archive is not made redundant by `PROJECT.md`/`CHANGELOG.md` (different axes), but
it is redundant against git + `openspec/specs/` + BACKLOG. Retiring it keeps the
repo thin — the project's paramount value — without losing any decision record.

## What Changes

- Remove `openspec/changes/archive/` and its 22 archived change directories.
- Switch the change lifecycle from `explore -> propose -> apply -> sync -> archive`
  to `explore -> propose -> apply -> sync`, where **sync** promotes verified delta
  specs into `openspec/specs/` (agent-driven, via the `openspec-sync-specs` method,
  since this CLI has no `sync` command and its `archive` is the only built-in
  sync-and-move) and then removes the now-redundant change directory. Historical
  deliberation lives in git history and BACKLOG.
- Record the reversal as **ADR-0004**, superseding the archival clause of ADR-0002.
- Update the flow prose in `AGENTS.md`, `docs/development-flow.md`, and
  `docs/blueprint.md` to drop the archive step.
- Update the two governance scenarios that illustrate ignored historical prose so
  they cite git history and superseded ADRs rather than "archived OpenSpec changes."

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `governance-prose`: the "ignores historical prose" guarantee is unchanged; its
  illustration no longer cites archived OpenSpec changes (which will not exist),
  citing git history and superseded ADRs instead.
- `quality-governance`: same illustration update to the "Historical prose is not
  governed" scenario.

## Impact

- Deletes `openspec/changes/archive/` (recoverable from git history).
- No code change, no dependency change, no runtime or published-surface change.
- Updates `docs/adr/` (new ADR-0004 + a status note on ADR-0002), `AGENTS.md`,
  `docs/development-flow.md`, `docs/blueprint.md`, and two spec scenarios.
- The prose-governance scan is unaffected in behavior — it already reads only a
  fixed active-prose file list, never the archive.
- Out of scope (see design): the `openspec update`-generated CLI shims under
  `.claude/`/`.agent/` are left as-is with `AGENTS.md` as the authoritative
  lifecycle; six stale `TBD ... after archive` spec Purpose placeholders are handed
  to the release-readiness sweep.
