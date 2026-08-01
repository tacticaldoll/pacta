## 1. Add `#[must_use]` to decision/result-shaped types

- [ ] 1.1 `crates/pacta-contract/src/lib.rs`: add `#[must_use]` to `Outcome`.
- [ ] 1.2 `crates/pacta-contract/src/lib.rs` (`lifecycle` module): add `#[must_use]` to `State`.
- [ ] 1.3 `crates/pacta-contract/src/lib.rs` (`kernel` module): add `#[must_use]` to `Directive`, `Notice`, and `StepResult`.
- [ ] 1.4 `crates/pacta-executor/src/lib.rs`: add `#[must_use]` to `Verdict`.
- [ ] 1.5 `crates/pacta-driver/src/lib.rs`: add `#[must_use]` to `Step` and `DriverError`.

## 2. Verification

- [ ] 2.1 Run the full Definition of Done (`AGENTS.md`) and confirm no in-workspace call site newly fails under `-D warnings`.

## 3. Spec sync

- [ ] 3.1 Confirm no capability delta to promote (per proposal.md's Modified Capabilities: none).
- [ ] 3.2 Remove `openspec/changes/must-use-decision-types/`.
