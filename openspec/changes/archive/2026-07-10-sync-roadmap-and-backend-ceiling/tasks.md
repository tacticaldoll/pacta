## 1. Consolidate the roadmap

- [x] 1.1 Update `BACKLOG.md` "Current Baseline" to include the shipped
  lifecycle-persistence contract, sans-I/O kernel, retainer encapsulation,
  `pacta-memory`, `pacta-conformance`, the ambient-time scan, and the coverage gate
- [x] 1.2 Fix the stale line asserting product positioning is governed by an active
  change; that change is archived and its specs are synced
- [x] 1.3 Graduate the shipped parts of the "Registry Conformance" area to baseline
  and narrow the remaining candidate to durable backends and an async `Registry`
- [x] 1.4 Expand "Explicitly Deferred" with async `Registry`, durable backends
  outside the workspace, a public pact-ingress API, an operator-triggered lapse
  sweep, and runtime heartbeat driving
- [x] 1.5 Record the breach-vs-lapse reconsideration as a candidate, not a decision
- [x] 1.6 State the workspace backend ceiling principle in `BACKLOG.md`, avoiding
  the prose scanner's stale phrases

## 2. Govern the backend ceiling

- [x] 2.1 Author the `quality-governance` "Workspace Governance Coverage"
  requirement documenting the existing coverage gate and mandatory boundary
  justification, and pinning the backend ceiling as the intent a new crate's
  justification must address
- [x] 2.2 Confirm the existing coverage gate already enforces it — a crate without
  a boundary fails the gate and a `CrateBoundary` cannot be built without
  `.because` — so no new code is added

## 3. Validate

- [x] 3.1 Run `openspec validate sync-roadmap-and-backend-ceiling --strict`
- [x] 3.2 Confirm the prose scanner passes on the rewritten `BACKLOG.md`
- [x] 3.3 Confirm no code changed (the change is BACKLOG prose plus a spec delta)

## 4. Definition of Done

- [x] 4.1 `cargo build --workspace`
- [x] 4.2 `cargo test --workspace`
- [x] 4.3 `cargo clippy --all-targets -- -D warnings`
- [x] 4.4 `cargo fmt --all --check`
- [x] 4.5 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 4.6 `cargo deny check`
- [x] 4.7 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml`
- [x] 4.8 Adversarial review passes before commit
