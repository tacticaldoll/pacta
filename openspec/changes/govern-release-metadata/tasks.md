## 1. Reaction implementation

- [ ] 1.1 Add `check_release_metadata(root: &Path) -> Result<(), Vec<ReleaseMetadataViolation>>` to `crates/pacta-governance/src/main.rs`, scanning `crates/*/Cargo.toml`'s `[package]` table only.
- [ ] 1.2 Implement publishable-crate detection (skip a manifest containing a literal `publish = false` line).
- [ ] 1.3 Implement the `description`/`readme` non-empty-literal check and the `license`/`repository`/`keywords`/`categories` literal-or-`.workspace = true` check.
- [ ] 1.4 Implement the readme-resolves-to-a-crate-local-file check (canonicalized path comparison against the workspace-root `README.md`).
- [ ] 1.5 Add the non-vacuous guard (no crates found under `crates/` fails loudly, matching sibling reactions).

## 2. Wiring

- [ ] 2.1 Add a `RELEASE_METADATA_REASON` constant describing the reaction's intent.
- [ ] 2.2 Wire `check_release_metadata` into `main()`'s `should_check_prose` sequence alongside the existing reactions, with the same eprintln!-and-exit-1 shape.

## 3. Tests

- [ ] 3.1 `current_release_metadata_satisfies_governance`: the real repo passes today.
- [ ] 3.2 A synthetic crate missing `description` fails with the expected violation.
- [ ] 3.3 A synthetic crate whose `readme` resolves to the shared workspace-root README fails.
- [ ] 3.4 A synthetic crate missing `keywords`/`categories` fails.
- [ ] 3.5 A synthetic crate with a literal `publish = false` is skipped entirely (no violation even with other fields missing).
- [ ] 3.6 A root with no `crates/` directory fails loudly (non-vacuous guard).

## 4. Verification

- [ ] 4.1 Run the full Definition of Done (`AGENTS.md`) locally before committing the apply-phase milestone.

## 5. Spec sync

- [ ] 5.1 Promote the `release-packaging` delta spec's Release Metadata requirement into `openspec/specs/release-packaging/spec.md`.
- [ ] 5.2 Remove `openspec/changes/govern-release-metadata/`.
