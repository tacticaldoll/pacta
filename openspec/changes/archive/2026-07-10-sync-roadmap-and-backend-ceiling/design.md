## Context

An independent adversarial fidelity audit judged the repo FAITHFUL to its thin
"lifecycle-contract, compose the rest" intent, but named one prospective risk as
most important: no spec *states* how many backends the workspace owns.
`pacta-memory` is defensible as a dependency-free reference and conformance anchor;
adding `pacta-sqlite` to the workspace would push toward a backend-provider
framework. A second adversarial review then corrected a premise: the enforcement
partly exists already — the coverage gate (`every_workspace_crate_has_a_boundary`)
fails CI for any crate without a boundary, and a `CrateBoundary` cannot be built
without a `.because(reason)` justification, so a new crate cannot enter silently or
unjustified. What is missing is the stated principle, not a new gate. The audit
also found `BACKLOG.md` stale on every count (missing a session of shipped work,
referencing an archived change as active, listing shipped conformance work as a
candidate) and surfaced a lifecycle reconsideration (infra-failure could
lapse-and-recover instead of terminally breaching).

## Goals / Non-Goals

**Goals:**
- Make `BACKLOG.md` reflect shipped truth.
- Convert the backend question from a judgment call into a governed one by stating
  the ceiling principle a new crate's justification must satisfy, backed by the
  coverage gate that already blocks an ungoverned addition.
- Record, without deciding, the breach-vs-lapse reconsideration.

**Non-Goals:**
- No change to lifecycle behavior; the breach-vs-lapse question is recorded only.
- No new backend, no `pacta-sqlite`.
- No change to the core contract, the driver, or the two backend/conformance
  crates.

## Decisions

### Decision: Document the existing teeth, add no redundant gate
Do not add a workspace-membership allowlist. State the ceiling as a
`quality-governance` requirement that documents the enforcement that already
exists — the coverage gate that fails on any ungoverned crate, plus the mandatory
`.because` justification on every boundary — and pins the backend ceiling as the
intent that a new crate's justification must address.

Rationale: an earlier draft proposed an allowlist test, but adversarial review
showed it would be a redundant and *weaker* second gate. The coverage gate already
fails CI for any crate without a boundary, and `CrateBoundary` cannot be built
without a `.because(reason)`, so a new crate already cannot enter silently or
unjustified — a bare `&[&str]` allowlist carries no justification field and adds
nothing. "Is this crate a durable backend" is not machine-detectable, so the honest
ceiling of mechanical enforcement is "every crate is governed by a justified
boundary." The ceiling principle rides on that justification: a backend crate's
`.because` must address why the thin library owns it, checked at review, not by a
fake count. The requirement is worded to claim only what the check proves.

Rationale for the home: the ceiling is a governance-coverage concern with an
executable check, which is exactly what `quality-governance` governs; keeping it in
one enforceable place avoids spreading a toothless principle across identity specs.

### Decision: Record breach-vs-lapse, do not decide it
Add the reconsideration to BACKLOG as a candidate: now that lease and lapse exist,
an infrastructure failure during execution could be left unsettled to lapse and
recover, rather than terminally breached.

Rationale: it is a genuine lifecycle-semantics change with its own spec and review;
folding it into a roadmap-and-governance change would smuggle a behavior change
into a hygiene change. Recording it keeps the finding honest and visible without
deciding it prematurely.

### Decision: BACKLOG edits carry no spec delta
The BACKLOG rewrite is prose governed by the existing `domain-language` "Roadmap
Uses Pacta Terms" requirement and the prose scanner; it changes no requirement, so
it needs no spec delta. The edits must avoid the scanner's stale phrases (for
example, write "dependency-free," not the banned capitalized slogan).

## Risks / Trade-offs

- [The ceiling is only as strong as the human review of a boundary's `.because`] →
  True, and stated honestly: the mechanical guarantee is "every crate is governed
  by a justified boundary," and the ceiling is the intent that justification must
  satisfy. No fake machine count claims more than it proves.
- [The ceiling is a positioning claim that could belong in identity specs] → It is
  reflected in BACKLOG for readers and governed in quality-governance; if a future
  change wants it in product-positioning too, that is additive.
- [Recording breach-vs-lapse without deciding could look like indecision] → It is
  deliberately deferred with a stated reason, consistent with how the backlog
  records other deferrals.

## Open Questions

- Whether the breach-vs-lapse reconsideration becomes a change depends on whether
  transient infra failures are common enough to warrant recovery over terminal
  breach; that is for a later, dedicated proposal.
