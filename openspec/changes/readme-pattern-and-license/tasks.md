## 1. Spec delta

- [x] 1.1 `product-positioning` delta: ADD "The Composition Pattern Is Documented"
- [x] 1.2 `openspec validate readme-pattern-and-license --strict` passes

## 2. README composition-pattern diagram

- [x] 2.1 Add a "what Pacta owns vs what you compose" section (after "Why Pacta") with a compact monospace diagram of the mechanism/policy split
- [x] 2.2 Keep it honest-surface: `Middleware` shown as the composition seam (retry/timeout/fail-fast compose there, not shipped); core decides *what*, never *how much*/*when*
- [x] 2.3 Keep it sibling-blind: no sibling/family named; no unproven "bricks compose" claim
- [x] 2.4 The reference `Driver` sits on the "Pacta owns" (mechanism) side, framed as a loop you run over your triad or replace with your own; the consumer-composed triad is exactly `Registry`/`Executor`/`Middleware`
- [x] 2.5 Complement, not restate, "Why Pacta": the diagram draws the ownership boundary rather than re-listing the same items; avoid the governance-banned phrase "middleware ecosystem"

## 3. README license one-liner

- [x] 3.1 Replace the verbose License section with the starter one-liner: `Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.`
- [x] 3.2 Drop the separate `### Contribution` subsection (unify to the `rust-openspec-starter` SSOT)

## 4. Verify

- [x] 4.1 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (active-prose governance parses the README)
- [x] 4.2 Diagram renders as intended in monospace; no broken links; License links resolve on the repo page
- [x] 4.3 The diagram claims only shipped behavior (no shipped retry/timeout implied); no sibling named
- [ ] 4.4 At sync, extend the `product-positioning` spec Purpose line to include the composition-pattern documentation (so the synced Purpose does not go stale)
