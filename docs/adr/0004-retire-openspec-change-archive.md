# ADR 0004: Retire the OpenSpec change archive

## Status

Accepted. Supersedes the archival clause of [ADR 0002](0002-adopt-openspec.md).

## Context

ADR 0002 adopted OpenSpec and kept every shipped change under
`openspec/changes/archive/`. In practice the archive is redundant: each archived
delta spec is a frozen copy of what was already synced into `openspec/specs/` (the
living truth), the deliberation (proposal/design) is preserved in git history, and
durable "why not / reconsidered" decisions live in `BACKLOG.md`. The archive is not
made redundant by `PROJECT.md`/`CHANGELOG.md` — those cover identity and release
notes — but it is redundant against git + `openspec/specs/` + BACKLOG. Keeping the
repository thin is the project's paramount value, so a second, browsable copy of
what git already holds does not earn its keep.

## Decision

Retire `openspec/changes/archive/`. The change lifecycle becomes:

```text
explore -> propose -> apply -> sync
```

**Sync** is terminal: promote the verified delta specs into `openspec/specs/`, then
remove the change directory. Its content now lives in `openspec/specs/` and git
history. Because this OpenSpec CLI has no `sync` command — its `archive` command is
the only built-in that syncs deltas, and it also moves the change to `archive/` —
sync is performed agent-driven, following the `openspec-sync-specs` method (read the
delta specs, edit `openspec/specs/` directly), gated by
`openspec validate --all --strict`.

Historical deliberation is retrieved from git history; reversed or reconsidered
decisions are recorded in ADRs and in `BACKLOG.md`.

## Consequences

- The repository stays thinner; no directory accumulates one entry per change.
- Retrieving a past proposal/design means reading git history rather than browsing
  an in-tree archive — a deliberate trade of browsability for thinness, since git
  retains every archived file verbatim.
- Spec sync is a manual, agent-driven edit under the strict validate gate rather
  than a CLI move; the validate gate (DoD and CI) catches a mis-synced spec.
- The `openspec update`-generated CLI shims (`.claude/`, `.agent/`) still offer an
  archive command; `AGENTS.md` is the authoritative lifecycle and no longer
  archives. Aligning the generated shims is an upstream concern, not a hand-edit.
