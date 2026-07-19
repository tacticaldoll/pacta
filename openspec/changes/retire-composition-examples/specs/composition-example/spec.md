## REMOVED Requirements

### Requirement: Public-API composition example
**Reason**: The standing `pacta-driver` `examples/compose.rs` this requirement governs is deleted. The public-API composition demonstration is re-homed to the facade doctest (see the `public-facade` capability's Facade Composition Example requirement), which runs and asserts the same claim -> execute -> settle end to end; the core crates' composition through their own public APIs is proven by `pacta-driver`'s `#[cfg(test)]` unit tests.
**Migration**: None for consumers. No public API changes. The composability guarantee and its constraints move to the facade doctest requirement in `public-facade`.

### Requirement: Example carries no orchestration behavior
**Reason**: The requirement is phrased about the deleted example. The pass-through, no-orchestration guarantee is preserved as a scenario on the facade doctest in `public-facade`, and orchestration remains explicitly deferred in `BACKLOG.md`.
**Migration**: None. The pass-through middleware guarantee is now asserted against the facade doctest.

### Requirement: Example preserves Registry purity
**Reason**: The requirement is phrased about the deleted example. Registry purity is a core axiom carried by the `Registry` contract and the domain-language spec, and is preserved as a scenario on the facade doctest in `public-facade`.
**Migration**: None. Registry purity in the composition proof is now asserted against the facade doctest.

### Requirement: Example adds no core dependency
**Reason**: The requirement is phrased about the deleted example. With the example gone, `pacta`'s `uuid` dev-dependency is removed and no example-only dependency remains; core-crate dependency boundaries stay enforced by the tianheng constitution.
**Migration**: None. The no-core-dependency-creep guarantee stays enforced executably by `pacta-governance`.
