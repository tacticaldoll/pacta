# Spec Delta

## MODIFIED Requirements

### Requirement: Facade Excludes The Kernel
The facade's curated surface SHALL exclude the sans-I/O lifecycle **step-driver** kernel. The
step-driver kernel (`Directive`, `Notice`, `Kernel`, `StepResult`, and the `kernel` module)
SHALL remain reachable only through `pacta-contract` directly, so the facade names
the compose-level and backend-author API and not the advanced state-machine machinery. This
exclusion SHALL be enforced by an executable reaction, not by omission alone; the reaction observes
a written `pacta_contract::kernel` path in a re-export or a public signature, and a kernel item
reached through
another module's re-export, through a glob whose leaves the scan cannot enumerate, or through a
macro-generated item is review-governed. The exclusion SHALL
scope to the `kernel` module only: the colorless `lifecycle` module is the backend-author surface
and is re-exported (see *Curated Public Entrypoint*), distinct from the excluded step-driver kernel.

#### Scenario: Kernel is absent from the facade surface
- **WHEN** a downstream consumer depends only on `pacta`
- **THEN** it cannot reach the step-driver kernel types through `pacta`, and must depend on `pacta-contract` directly to use them

#### Scenario: A facade kernel re-export fails governance
- **WHEN** the facade's public API re-exports an item through a written `pacta_contract::kernel` path
- **THEN** the governance reaction fails via the hunyi semantic dimension

#### Scenario: Re-exporting the lifecycle module does not trip the kernel exclusion
- **WHEN** the facade re-exports the `pacta_contract::lifecycle` module
- **THEN** the governance reaction reports no violation, because the exclusion targets the `kernel` module and `lifecycle` is a distinct colorless module that is the backend-author surface
