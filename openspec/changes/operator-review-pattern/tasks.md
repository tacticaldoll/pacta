## 1. Reference demonstration

- [ ] 1.1 In `crates/pacta-executor/src/lib.rs`'s existing `#[cfg(test)]` module, add an inspectable log (e.g. `Rc<RefCell<Vec<Uuid>>>`) alongside the existing `FixedThreshold`/`GiveUp`/`GiveUpExecutor` fixture.
- [ ] 1.2 Record the exhausted pact's id into the log at the `Verdict::Concede` arm, before settling `Outcome::Breached`.
- [ ] 1.3 Add a `#[test]` asserting the log contains exactly the exhausted pact's id once the threshold is reached, and stays empty before it.

## 2. Documentation

- [ ] 2.1 Add a paragraph to `crates/pacta/src/lib.rs`'s crate-level docs, next to the `Policy`/`Verdict` explanation, describing the Operator Review / Tribunal pattern and its non-goal boundary.

## 3. Housekeeping

- [ ] 3.1 Update `BACKLOG.md`'s Operator Review candidate entry to point at the shipped demonstration instead of describing it as undesigned.

## 4. Verification

- [ ] 4.1 Run the full Definition of Done (`AGENTS.md`) before committing the apply-phase milestone.

## 5. Spec sync

- [ ] 5.1 Promote the `composition-governance` delta spec's new requirement into `openspec/specs/composition-governance/spec.md`.
- [ ] 5.2 Remove `openspec/changes/operator-review-pattern/`.
