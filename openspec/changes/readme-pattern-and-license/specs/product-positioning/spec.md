## ADDED Requirements

### Requirement: The Composition Pattern Is Documented
Pacta SHALL document, where consumers look, its mechanism-versus-policy composition
pattern: the core — including the reference `Driver`, a mechanical loop that decides
no outcome — owns the lifecycle mechanism and no policy, and the consumer composes
the policy at the `Registry`, `Executor`, and `Middleware` seams. The documentation
SHALL depict only shipped behavior — presenting `Middleware` as the composition seam
rather than as shipped retry, timeout, or backoff — and SHALL name no sibling product.

#### Scenario: The README distinguishes what the core owns from what the consumer composes
- **WHEN** a consumer reads the README
- **THEN** it distinguishes what Pacta owns (the lifecycle contract, the sans-I/O kernel, lease/lapse recovery, and the reference `Driver` that decides no outcome) from what the consumer composes (a `Registry` backend, an `Executor`, and `Middleware`), and states that the core decides what happens to a pact but not how much to retry or when to give up

#### Scenario: The documented pattern claims only shipped behavior
- **WHEN** the composition pattern is documented
- **THEN** it presents `Middleware` as the seam where policy such as retry, timeout, or fail-fast composes, without presenting those as behavior Pacta ships
