## Why

Pacta is being published to crates.io at 0.1.0. Its public API is currently spread
across three crates (`pacta-contract`, `pacta-executor`, `pacta-driver`) with no
single entry point, and the boundary between the compose-level API and the advanced
sans-I/O kernel is enforced only by omission. Once the workspace publishes, giving
consumers one curated `cargo add pacta` front door is a publisher's responsibility —
and making the curated-vs-advanced split executable (rather than a documentation
promise) is something only code can do, because the kernel is legitimately public
in `pacta-contract` and no dependency rule or API snapshot can express "recommended".

## What Changes

- Add a new workspace member crate `pacta`: a pure facade that re-exports the
  compose-level public API of `pacta-contract`, `pacta-executor`, and
  `pacta-driver`, and carries no logic of its own. This is the published entrypoint.
- The facade EXCLUDES the sans-I/O `kernel` module
  (`Directive`/`Notice`/`Kernel`/`StepResult`) from its curated surface — the kernel
  stays advanced-only, reachable via `pacta-contract` directly — and depends on no
  backend (`pacta-memory`).
- Add a facade composition example at `crates/pacta/examples/compose.rs` that wires
  the lifecycle end to end through the `pacta::` front door. The existing
  `crates/pacta-driver/examples/compose.rs` is UNCHANGED: it remains the proof that
  the three raw core crates compose through their own public APIs.
- Add executable governance for the new surface, all reached through the existing
  `tianheng` dependency (no new governance dependency):
  - a Tianheng dependency boundary restricting `pacta` to
    `{pacta-contract, pacta-executor, pacta-driver}`;
  - a hunyi semantic boundary asserting the facade's public API must not expose the
    kernel module — enforcement, not mere omission;
  - a source-scan reaction asserting the facade's library is re-exports only (no
    functions, types, traits, or other logic), so the entrypoint cannot silently
    accrete behavior.
- Update the governance coverage test and the synthetic-workspace test so the new
  member is governed like every other crate.

## Capabilities

### New Capabilities
- `public-facade`: Pacta SHALL offer a single curated entrypoint crate that
  re-exports the compose-level public API, keeps the sans-I/O kernel out of that
  curated surface by an executable reaction, and stays a pure re-export crate by an
  executable reaction.

### Modified Capabilities
- `quality-governance`: the "Workspace Governance Coverage" requirement's
  thin-workspace enumeration is extended to include the published facade entrypoint,
  recording that once the workspace publishes, owning the curated entrypoint is a
  publisher concern the thin library legitimately holds — distinct from a composer's
  batteries-included convenience.

## Impact

- New crate `crates/pacta/` (workspace member; publishable — inherits the workspace
  `publish = true` that `prepare-release-hygiene` sets); new `specs/public-facade`.
  This change is ordered AFTER `prepare-release-hygiene`: the facade's legitimacy
  depends on the workspace actually publishing.
- New `crates/pacta/examples/compose.rs`; `crates/pacta-driver/examples/compose.rs`
  and the `composition-example` spec are UNCHANGED.
- `crates/pacta-governance/src/main.rs`: one new `CrateBoundary`, one new
  `signature_boundary` (hunyi kernel-exclusion), one new source-scan reaction
  (re-exports only), and coverage/synthetic-workspace test updates. No new
  governance dependency — `SemanticBoundary` is reached through `tianheng`.
- Root `Cargo.toml`: `pacta` added to `members`; `pacta-driver` added to
  `workspace.dependencies` (the facade needs it).
- No change to any core crate's code or public API; the facade is additive.
