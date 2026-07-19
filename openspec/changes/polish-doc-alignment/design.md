## Context

An audit of the `release/0.1.1` doc surface found the workspace fundamentally tight
(vision one-liner is intentional SSOT-identity alignment; exhaustiveness is cleanly
partitioned by owning capability; retired items left no dangling references) with one
real defect and two cheap consistency items. The real defect is a three-way Definition
of Done divergence: `README.md` and `AGENTS.md` list five gates; `docs/development-flow.md`
and CI enforce seven (adding `rustdoc -D warnings` and `cargo deny`). The drift itself is
the proof-of-cost of duplicating the list across three living docs.

## Goals / Non-Goals

**Goals:**
- One authoritative, complete Definition of Done; other docs point to it.
- Govern the single-sourcing so the drift cannot recur.
- Align the `pacta-contract` description and the crate-README license form.

**Non-Goals:**
- No version bump / changelog (finalization owns that). No code or public-API change.
- No churn of the audit's LEAVE items (BACKLOG↔spec digest, cross-spec doctest overlap,
  README domain-language curated subset). Cutting them is churn, not progress.

## Decisions

- **`AGENTS.md` owns the complete Definition of Done.** `governance-prose`'s Document
  Authority already makes `AGENTS.md` the operating-protocol authority, so the DoD (a
  protocol artifact) belongs there: the seven local gates (clippy as
  `cargo clippy --workspace --all-targets -- -D warnings`, matching CI exactly), plus
  the per-gate ownership note currently stranded in the flow doc, plus a line noting CI
  additionally verifies the MSRV build (`cargo +1.88 build --workspace`) — a CI-only
  gate because it needs a pinned toolchain, so it is documented rather than listed as a
  routine local step. *Rationale:* one SSOT the others reference, and honest about the
  full CI gate set (the audit found the MSRV job listed nowhere).
- **Single-sourcing governs prose, not CI parity.** The new scenario requires prose
  consistency (one DoD, others point to it). It does NOT claim a mechanical
  doc-vs-CI guarantee — nothing derives the docs from `.github/workflows/ci.yml` — so
  keeping the two aligned stays a review responsibility, stated honestly.
- **No divergent subset anywhere.** `README.md` Contributing and
  `docs/development-flow.md` point to `AGENTS.md` for the DoD rather than restating a
  partial list — a subset is exactly what misled a contributor into a green-local /
  red-CI state. README keeps the `openspec new change` quickstart; the flow doc keeps
  its lifecycle checklist but defers the gate list.
- **Govern it (preventive, not speculative).** The drift just happened, so the
  single-sourcing requirement earns its place. It is review-governed, matching Document
  Authority's sibling scenarios (not every prose rule has a tianheng tooth).
- **`pacta-contract` description conforms to positioning.** The defect is that the
  description labels Pacta *as* a "task runtime" — a product-identity claim — where the
  README opening and `product-positioning` call it a "durable contract fabric". Change
  it to match the README opening. The `runtime`/`task` workspace keywords are a
  deliberate discoverability choice and stay (not in scope). This is the crates.io card
  for the release, so the one-line fix is worth it.
- **License reflow is form-only.** Reflow the seven crate license blocks to one line to
  match the root; keep the **absolute** URLs — the LICENSE files are not in each crate's
  tarball, so relative links would 404 on crates.io. Do not convert them to relative.

## Risks / Trade-offs

- **DoD change could itself be wrong** → dogfood it: run the actual seven-gate DoD as
  part of verification, proving the documented list matches what passes.
- **README loses its inline gate block** → acceptable; the block was the divergent copy.
  README still points to `AGENTS.md` (as it already did) and keeps the workflow quickstart.
