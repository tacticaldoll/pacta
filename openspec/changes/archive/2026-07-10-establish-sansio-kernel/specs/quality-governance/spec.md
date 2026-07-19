## ADDED Requirements

### Requirement: Kernel Async-Exposure Reaction
Pacta SHALL run an executable semantic reaction that keeps the lifecycle kernel
free of exposed async, so the kernel's runtime-agnosticism cannot silently drift.

#### Scenario: Kernel async fn is rejected
- **WHEN** the lifecycle kernel's public API exposes an `async fn`
- **THEN** the governance reaction fails via the hunyi semantic dimension

#### Scenario: Async-exposure reaction runs in CI
- **WHEN** a push or pull request runs CI
- **THEN** the async-exposure reaction runs as part of the governance check
