## Context

The workspace is a virtual manifest with six crates, all currently `publish = false`
and expressed inconsistently (four explicit, two inherited). Shared dependencies
live in `workspace.dependencies` as path-only entries. There is no MSRV, changelog,
or release-scope statement. This change makes the workspace publishable to crates.io
at 0.1.0 and is the predecessor for `establish-pacta-facade`.

## Goals / Non-Goals

**Goals:**
- The five library crates publish; the governance gate does not.
- `cargo publish --dry-run` succeeds for the publishable crates.
- MSRV declared and CI-verified; changelog present; README honest about scope.

**Non-Goals:**
- No source or public-API change.
- No facade crate (that is the follow-on change).
- No new runtime features, backends, ingress, or adapters.

## Decisions

### Decision: Flip the workspace default, not per-crate flags
Set `workspace.package.publish = true` and keep a single explicit
`publish = false` on `pacta-governance`. Alternative (explicit `publish = true` on
each of five crates) is noisier and drifts; one default + one documented exception
is the smaller surface. `pacta-contract`, `pacta-memory`, and `pacta-conformance`
switch their explicit `publish = false` to `publish.workspace = true`, matching how
`pacta-executor`/`pacta-driver` already opt in.

### Decision: Publish memory and conformance, not only the core three
`pacta-conformance` MUST publish — external durable backends dev-depend on it to
prove conformance, which is its entire reason to exist. `pacta-memory` publishes as
a usable, dependency-free reference backend. `pacta-governance` stays unpublished:
it is an internal CI gate and would drag `tianheng` onto the published graph.

### Decision: Version the shared path dependencies
`cargo publish` refuses a path dependency without a version. Add
`version = "0.1.0"` beside the `path` in each `workspace.dependencies` entry that a
publishable crate consumes. `cargo publish --dry-run` (a release-time gate) is the
proof; this change establishes the precondition.

### Decision: MSRV = 1.88, verified in CI
The governance crate uses let-chains (stabilized in 1.88); edition 2024 alone needs
only 1.85, so 1.88 is the binding floor. Declaring `rust-version` without checking
it is aspirational, so CI gains a step that explicitly installs and selects a 1.88
toolchain (there is no `rust-toolchain` pin file) and builds the workspace on it.

### Decision: README honesty is scope, not category
`product-positioning` blesses "durable contract fabric" as Pacta's product
category, so that framing stays. The honesty fix is additive: a release-scope
statement (what 0.1.0 ships; durable backends live outside) and de-emphasizing the
`Signal -> Pact` ingress as user-provided, not a shipped API. All edits stay green
against the prose-governance stale-phrase gate.

## Risks / Trade-offs

- [crates.io names unavailable / metadata rejected] → Caught by
  `cargo publish --dry-run` at release time; this change adds the metadata that
  makes the dry-run meaningful.
- [MSRV 1.88 is wrong (features need newer)] → The CI MSRV step fails loudly rather
  than shipping a false floor; adjust the declared version to match.
- [Inherited `readme`/`keywords` path resolution] → Verified by `cargo package`
  during the DoD; inherited `readme` resolves relative to the workspace root.
- [Publishing is irreversible] → This change only makes the workspace *able* to
  publish; the actual `cargo publish` happens in the `release 0.1.0` step behind the
  dry-run gate.

## Migration Plan

Manifest-and-docs only; no code. Rollback is reverting the manifest edits and
removing `CHANGELOG.md`. The actual publish is a later, separate, gated step.

## Open Questions

None blocking. Whether to also publish a `pacta` facade as the front door is decided
and handled in the follow-on `establish-pacta-facade` change, which depends on this
one.
