## Context

This is the last hygiene pass before the paused 0.1.0 publishes. A three-part
adversarial sweep found no publish blocker but surfaced doc/spec drift: the `pacta`
facade is absent from the CHANGELOG, README, and `release-packaging` spec; six spec
Purpose lines are stale `TBD ... after archive` placeholders; AGENTS.md's DoD still
says "or archiving a change." Separately, the user's discipline argument: `docs/adr/`
duplicates decisions already carried by the living docs and git, the same
redundancy-with-git that retired the change archive — and the supersession note we
had to add to ADR-0002 is exactly the second-copy maintenance tax. This change
single-sources the docs and retires the ADR class.

## Goals / Non-Goals

**Goals:**
- Name the facade in every authoritative place a consumer or the spec looks.
- Remove stale placeholders and the AGENTS.md self-contradiction.
- Retire ADRs by adopting git-as-provenance, losing no recoverable decision.
- Ship license text in the published tarballs.

**Non-Goals:**
- No code, dependency, runtime, or public-API change.
- Not the API-freeze decisions (`#[non_exhaustive]`, `Policy`, serde/derives) — those
  are settled separately.
- No new capability specs.

## Decisions

### git-as-provenance replaces the ADR class
The decision content of every ADR already lives in a living doc — AGENTS.md (process:
adopt OpenSpec, sync-not-archive), the domain-language spec + `docs/domain-language.md`
Legacy Mapping (the vocabulary), PROJECT.md (identity). What an ADR uniquely held was
the dated rationale + supersession chain, which git holds verbatim. ADRs are already
orphaned: they are absent from AGENTS.md's Document Authority list and PROJECT.md's
References. So the consistent, disciplined move — the one already applied to the
archive — is: git commit bodies + PR descriptions are the decision-rationale store,
BACKLOG holds reconsiderations, living docs are the SSOT, and there is no separate
ADR file class.

- Re-homing: nothing is irrecoverable. ADR-0003's vocabulary + migration is fully in
  `docs/domain-language.md`; ADR-0002's OpenSpec adoption is in AGENTS.md's workflow;
  ADR-0001's practice is replaced by the stated principle; ADR-0004's
  archive-dissolution rationale and the ADR-retirement decision are recorded in
  BACKLOG (and git).
- The self-reference (retiring ADRs would "want" an ADR) is resolved cleanly: the
  principle is stated in AGENTS.md and the decision is recorded in git + BACKLOG — no
  new ADR.
- The two governance scenarios that cite "superseded ADRs" are reworded to cite git
  history only. (They were set to that phrasing one change ago; this simplifies them,
  the expected double-touch of deciding ADR retirement after archive retirement.)

### Facade single-sourcing
The facade was added late and never propagated to the consumer-facing docs. Add it to
CHANGELOG "Added", README "Status (0.1.0)" + architecture framing, and the
`release-packaging` publishable-set requirement + honest-release-scope scenario, so
the recommended entrypoint is named everywhere authority lives.

### The six Purpose placeholders
`TBD - created by archiving change X. Update Purpose after archive.` is stale on two
counts: "after archive" is now impossible, and the provenance is false (specs are now
produced by sync, not archive). Replace each with a real one-line Purpose (drop the
provenance sentence; git holds provenance). Purpose is spec top-matter with no delta
mechanism, so these are direct edits during sync.

### License text in tarballs
`readme` is workspace-inherited and cargo copies it into each package, but the root
`LICENSE-*` files are not. Add `LICENSE-MIT` and `LICENSE-APACHE` to each publishable
crate (symlinks to the root files; cargo packages the resolved content), so a
downloaded crate carries its license text, not only the SPDX field.

## Risks / Trade-offs

- [A decision's rationale lost with the ADRs] → Each ADR's rationale is in git
  verbatim, and the durable/forward-looking parts are re-homed to AGENTS.md/BACKLOG.
  The adversarial review checks nothing enforced-or-referenced-only-in-ADRs is dropped.
- [Losing browsable in-tree decision provenance] → Accepted, consistent with the
  archive decision: git is the provenance store.
- [Governance scenarios double-touched] → Cosmetic; the net wording is simpler
  (git-history-only) and validate gates it.
- [Symlinked license files behave oddly on some checkouts] → cargo resolves symlinks
  into the package at publish; the DoD `cargo publish --workspace --dry-run` confirms
  each tarball carries the license text.

## Migration Plan

Apply the doc/spec edits, add the license files, remove `docs/adr/`, hand-sync the
three delta specs + six Purposes into `openspec/specs/`, `openspec validate --all
--strict`, run the DoD, land on `release/0.1.0`, then sync + retire the change dir
(no archive).

## Open Questions

None. The API-freeze decisions are tracked separately.
