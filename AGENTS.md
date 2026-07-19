# AGENTS.md

Meta-guideline for AI coding agents working in this repository. Read this first,
then let `openspec/specs/` and active change specs be the source of durable
architecture truth.

## Pacta In One Sentence

Pacta is a thin, elegant durable contract fabric and governed pattern framework
for Rust user-defined obligations.

This repository is intentionally narrow. Pacta is not a broker, workflow engine,
or queue feature platform. Framework integrations are boundary patterns, not the
identity of the core.

## Architectural Axioms

Before proposing or writing code, protect these axioms:

1. **Lifecycle kernel stays thin**: `pacta-contract` owns the durable pact
   envelope and `Registry` lifecycle contract. It does not own orchestration,
   scheduling, routing, adapters, or backend business behavior.
2. **Execution grows by composition**: execution behavior belongs around
   `Executor` through Pacta-native middleware, policies, and future governed
   patterns.
3. **Adapters stay outside the core**: integrations with external frameworks,
   transports, or storage systems must not define first-layer Pacta APIs.
4. **Vocabulary is governance**: names such as `Pact`, `Docket`, `Clause`,
   `Brief`, `Registry`, `Claim`, `Retainer`, `Fulfill`, `Breach`, and
   `Tribunal` protect the contract/arbitration worldview.

## Document Authority

- `openspec/specs/` is shipped architecture truth.
- `openspec/changes/` contains active proposed truth until it is synced.
- `PROJECT.md` states product vision, positioning, and non-goals.
- `docs/blueprint.md` names extension surfaces and boundary rules.
- `BACKLOG.md` records deferred decisions and candidate patterns, not mandatory
  phases.
- `AGENTS.md` is operating protocol for agents and contributors.

Decision provenance lives in git — the commit body and pull request that made a
change record its rationale. Forward-looking or reversed decisions are noted in
`BACKLOG.md`. There is no separate architecture-decision-record file class; the
living documents above are the single source of truth for current state, and git is
the source of truth for why it changed.

If these documents conflict, fix the conflict through an OpenSpec change before
implementing feature code.

## Adversarial Review Stance

When reading proposals or reviewing code, actively challenge the design:

- **Propose phase**: Does the change make Pacta heavier than the thin kernel
  requires? Can it be expressed as a governed pattern on an extension surface?
  Does it treat a benchmark or adapter as core identity?
- **Apply phase**: Does the implementation leak orchestration, integration, or
  product prose drift into core crates or active guidance? Does Tianheng still
  bite the boundary that the prose claims?

Reject or redesign changes that pull Pacta toward broad queue-runtime behavior.

## Pattern Admission Guardrail

Pacta leads with patterns — consumers ignite its work, they do not gate it — so a
composition pattern earns its place by its own soundness, not by a consumer's demand.
That license is bounded: admit a pattern into a core crate only when all four hold.

1. **Native**: expressed in purely Pacta-native vocabulary.
2. **Sibling-clear**: steps on no sibling product's domain.
3. **Non-goal-clear**: pulls toward no stated non-goal.
4. **Mechanism-only**: touches optional composition mechanism, not the durable contract.

A pattern failing any question is rejected from core or relocated to an extension
surface, a sibling, or the consumer. This is what keeps "lead with patterns" from
becoming a feature catalog: concrete orchestration such as retry, timeout, or
circuit posture fails questions 2 or 3 no matter how reasonable it looks.

A judgment is prose; its enforceable content projects onto two executable shadows,
and part of it cannot project at all:

- **Governance** gives static/structural teeth (a Tianheng reaction over the code's
  shape). Non-goal vocabulary can be forbidden here without naming a sibling.
- **Conformance** gives dynamic/behavioral teeth (a suite that runs the code).
- **Adversarial review** holds the irreducible residue. The sibling-clear question
  cannot become a reaction — a reaction would have to name the siblings it checks
  against, which sibling-blindness forbids — so it stays a review obligation by
  design. State which prose has teeth and which does not, rather than pretending all
  of it does.

## OpenSpec Workflow

This repository uses OpenSpec. The lifecycle is:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: investigate and shape intent. Do not write feature code outside
   a change.
2. **Propose**: create `proposal.md`, `design.md`, `tasks.md`, and delta specs.
   Commit as `docs(<change>): propose <summary>`.
3. **Apply**: implement against the active delta specs. Check off tasks only
   after verification. Commit coherent compiling milestones as `feat(...)` or
   `fix(...)`.
4. **Sync**: merge verified delta specs into `openspec/specs/` (agent-driven —
   this CLI has no `sync` command), then remove the completed change directory; its
   content now lives in `openspec/specs/` and git history. There is no archive.
   Commit as `docs(specs): sync <change>`.

## Commits

Use Conventional Commits: `type(scope): summary`.

- Use lowercase imperative mood.
- Keep the subject at 72 characters or fewer.
- Write commit messages in English.
- Do not include AI signatures, tool signatures, PR numbers, or issue numbers.
- `release: X.Y.Z` is reserved for release commits on `main`.

### Branch Commits

- Use the Conventional Commit format above for development commits.
- Use the body to record motivation, important decisions, constraints, and
  verification when that context exists. Do not merely enumerate changed files.
- Do not append pull request or issue numbers to the subject or body.
- Development branches may contain multiple coherent commits because the pull
  request is squash-merged.

### Pull Requests

- Branch from `main` and open every change directly against `main`.
- Make the pull request title the intended squash commit subject.
- Give every pull request a non-empty body that explains why the change is
  needed, what changed, consequential decisions or tradeoffs, and verification.
- Rebase the branch onto the current `main` before final verification.
- Do not introduce a release integration branch between a change and `main`.

### Squash Merges

- Squash-merge every verified pull request into `main`.
- Make the squash commit subject exactly the approved pull request title.
- Give every squash commit a non-empty body distilled from the approved pull
  request body. Preserve durable rationale, decisions, constraints, and
  verification; omit transient checklists and generated commit lists.
- Do not append a pull request number, issue number, or URL to the squash
  subject or body.
- Every content-changing commit on `main`, including release preparation, must
  come from a squash-merged pull request.
- Keep `main` releasable after every merge.

### Attribution

- Do not include AI, agent, model, tool, automation, or generation attribution
  in commits, pull requests, tags, changelogs, or release notes.
- Prohibited forms include AI `Co-authored-by` trailers, `generated by`,
  `written with`, model or agent names used as signatures, and tool signatures.
- A `Co-authored-by` trailer is allowed only for a real human contributor.

### Release Finalization

- Prepare version metadata, the changelog, and other release content in a pull
  request whose squash subject is exactly `chore(release): prepare X.Y.Z`.
- Give the release preparation squash commit a non-empty body describing the
  release scope, compatibility, metadata changes, and verification.
- Run the complete Definition of Done against that commit after it reaches
  `main`.
- Finalize the release by creating annotated tag `vX.Y.Z` directly on that
  verified squash commit. The tag message must be exactly `release: X.Y.Z`.
- Push the annotated tag without creating another commit. A release branch and
  an empty release commit are not part of the release flow.

## Definition Of Done

Run these from the workspace root before checking off implementation tasks or
syncing specs. This is the single source for the gate list — `README.md` and
`docs/development-flow.md` point here rather than restating it.

```bash
cargo build --workspace --all-features
cargo build --workspace --no-default-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --no-default-features -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
cargo deny check
cargo run -p pacta-governance -- check --manifest-path Cargo.toml
```

Both ends of the feature space are gated. `--all-features` is required specifically so
`pacta-memory`'s async reference backend and the async conformance runners (`run_async` parity and
`run_async_contention`), gated on `pacta-memory`'s `async` feature, are exercised by build, test,
clippy, and doc. (A plain workspace *test* build already compiles `pacta-contract`'s async binding
and runs its async unit tests — dev-dependency feature unification pulls in `pacta-contract/async` —
but those reference-backend and conformance checks stay gated off without `--all-features`.)
`--no-default-features` keeps the **sync-only** surface covered: it compiles the workspace with
`async` off, so an async item accidentally left un-gated, or an async-only symbol referenced from the
sync path, fails to compile — protecting the promise that a sync-only consumer pulls no async.
(`fmt` and `cargo deny` are feature-independent.)

CI runs the same gates on push and pull request, and additionally verifies the
declared MSRV builds (`cargo +1.88 build --workspace --all-features`). Rust style lives in these
checks: rustfmt formats, clippy denies warnings, rustdoc denies documentation
warnings, cargo-deny owns resolved supply-chain policy, and `pacta-governance` owns
Tianheng architecture boundaries.
