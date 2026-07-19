## Context

The README explains the lifecycle and argues thin-vs-heavy in prose ("Why Pacta")
but never draws the ownership split that is Pacta's actual pattern. With
`infra-failure-lapses` shipped, that split is now fully true in code (the kernel owns
mechanism only). Separately, the License section is verbose (bullets + URLs +
`### Contribution`) where the family's `rust-openspec-starter` uses one line.

## Goals / Non-Goals

**Goals:**
- A compact, honest, pacta-local diagram of "what Pacta owns vs what you compose."
- License section matching the starter's canonical one-liner.

**Non-Goals:**
- The family-level "pattern-as-product / agent-resilient architecture" thesis. It
  names the family's shape and is a *bet* not yet proven (only one brick exists), so
  it stays in the private planner roadmap, not a sibling-blind component README.
- No code, spec, or version change.

## Decisions

- **Diagram altitude: pacta-local, sibling-blind.** Show the mechanism/policy split
  for *pacta itself* — Pacta owns (lifecycle contract + sans-I/O kernel + lease/lapse
  + the reference `Driver`, a mechanical loop that decides no outcome) vs what the
  user composes (`Registry`, `Executor`, `Middleware` — the obligation triad). Name no
  sibling and make no family claim. *Rationale:* the roadmap's sibling-blind rule,
  and honest-surface — the "thin bricks compose without upstream churn" claim is
  unproven until a second brick lands.
- **The `Driver` is shipped mechanism, not a consumer obligation.** Pacta ships a
  reference `Driver` (facade re-exports it; `runtime-skeleton` requires it; it decides
  no outcome). The diagram places it on the "Pacta owns" side, framed as a reference
  loop you *run* over your triad (or replace with your own loop over the kernel) —
  keeping it off the "you must write this" side. The obligation triad the consumer
  implements is exactly `Registry`/`Executor`/`Middleware`, matching
  `composition-governance`'s User-Obligation Delivery Pattern.
- **Honest about deferred orchestration.** The diagram presents `Middleware` as the
  *seam* where policy (retry, timeout, fail-fast) composes; it does not claim Pacta
  ships those. The one-line caption says the core decides *what* happens and never
  *how much* to retry or *when* to give up — that is the user's.
- **License one-liner, matching the starter, drop the `### Contribution` subsection.**
  Unify to the family SSOT. *Trade-off:* the dropped inbound=outbound contribution
  note had minor legal-clarity value; the dual-license SPDX still declares the terms,
  and matching the SSOT wins. Root README keeps relative `LICENSE-*` links (they
  resolve on the repo page); crate READMEs already use the one-liner shape with
  absolute URLs (a justified crates.io divergence) and need no change.

## Risks / Trade-offs

- **Honest-surface drift** if the diagram overstates shipped capability. → Mitigated
  by wording `Middleware` as the seam and keeping retry/timeout as "composes here,"
  not "provided."
- **Scope creep toward positioning philosophy.** → Kept out by the sibling-blind /
  pacta-local decision; deeper positioning belongs in `PROJECT.md` or the private
  roadmap, not here.
