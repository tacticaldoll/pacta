## MODIFIED Requirements

### Requirement: Curated Public Entrypoint
Pacta SHALL provide a single facade crate `pacta` that is the curated public
entrypoint to the compose-level API. The facade SHALL re-export the public items a
downstream consumer needs to compose the lifecycle end to end — implement
`Registry`, implement `Executor` and `Middleware`, and run the `Driver` — drawing
them from `pacta-contract`, `pacta-executor`, and `pacta-driver`. The facade SHALL
depend only on those three crates and SHALL NOT depend on any backend crate.

#### Scenario: Facade re-exports the compose-level surface
- **WHEN** a downstream consumer depends only on `pacta`
- **THEN** it can name `Pact`, `Claim`, `Retainer`, `Timestamp`, `Outcome`, `Settlement`, and `Registry`; `Executor`, `Execution`, and `Middleware`; and `Driver`, `Step`, and `DriverError`, without depending on the individual core crates directly

#### Scenario: Facade depends on no backend
- **WHEN** `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` runs
- **THEN** the Tianheng constitution reports no violation, because `pacta` depends only on `pacta-contract`, `pacta-executor`, and `pacta-driver`
