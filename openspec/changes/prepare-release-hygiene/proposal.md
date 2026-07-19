## Why

Pacta is being published to crates.io at 0.1.0, but the workspace is not
release-ready: every crate is `publish = false`, the shared `workspace.dependencies`
path entries carry no version (so `cargo publish` would refuse them), there is no
declared MSRV, no CHANGELOG, and the README states no release scope for a
first-time consumer. This change makes the workspace publishable and honest, and it
is the predecessor that legitimizes the `pacta` facade (whose value depends on the
workspace actually publishing).

## What Changes

- Enable publishing: flip the workspace default to `publish = true`; keep
  `pacta-governance` explicitly `publish = false` (an internal governance gate that
  depends on `tianheng`). The publishable set is `pacta-contract`,
  `pacta-executor`, `pacta-driver`, `pacta-memory`, and `pacta-conformance`.
- Make path dependencies publishable: add `version = "0.1.0"` to the
  `workspace.dependencies` path entries, so a published crate's dependencies resolve
  on crates.io.
- Add release metadata inherited from `workspace.package`: `repository`,
  `keywords`, `categories`, and `readme`, opted into by each publishable crate.
- Declare an MSRV: `rust-version = "1.88"` (the governance crate uses let-chains),
  and verify it in CI on the pinned toolchain.
- Add `CHANGELOG.md` (Keep a Changelog) with a single honest `0.1.0` entry.
- README honesty pass: state what 0.1.0 ships and what lives outside the workspace,
  and mark the `Signal -> Pact` ingress as user-provided rather than a shipped API.
- Make the CI Definition-of-Done gates explicitly `--workspace` for clarity (the
  virtual workspace already builds all members; this removes ambiguity).

## Capabilities

### New Capabilities
- `release-packaging`: Pacta SHALL be packaged for a crates.io release — a defined
  publishable crate set with required metadata and versioned dependencies, a
  declared and CI-verified MSRV, a changelog, and a README that states the release
  scope honestly.

### Modified Capabilities
(none — no existing requirement changes behavior; the CI `--workspace` edit is a
clarification of the already-required workspace-root gates.)

## Impact

- Root `Cargo.toml`: `workspace.package` gains `publish = true`, `rust-version`,
  `repository`, `keywords`, `categories`, `readme`; `workspace.dependencies` path
  entries gain `version = "0.1.0"`.
- `crates/pacta-contract`, `-memory`, `-conformance`: drop explicit
  `publish = false` (inherit `true`); all five publishable crates opt into the
  inherited metadata. `crates/pacta-governance` keeps explicit `publish = false`.
- New `CHANGELOG.md`; `README.md` release-scope + ingress honesty edits (kept green
  against the prose-governance gate).
- `.github/workflows/ci.yml`: explicit `--workspace`; new MSRV verification step.
- No source-code or public-API changes.
