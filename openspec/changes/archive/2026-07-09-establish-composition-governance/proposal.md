## Why

Pacta now has a contract crate, executor crate, driver skeleton, and Tianheng CI,
but the enforceable boundary still only covers workspace dependency direction.
The next risk is architecture leakage: Tower or adapter vocabulary can enter core
crates before Pacta's own composition model is defined.

## What Changes

- Define Pacta-native composition governance for middleware, policies, and
  adapters.
- Tighten the distinction between core runtime crates and user/integration
  adapter scope.
- Clarify that `pacta-contract` is isolated from workspace crates, not literally
  free of all external dependencies.
- Extend Tianheng governance so core crates reject framework-owned dependencies
  and adapter/back-end leakage, rather than only rejecting invalid workspace
  dependencies.
- Preserve the current runtime skeleton while documenting the next composition
  boundary before adding larger middleware APIs.

## Capabilities

### New Capabilities

- `composition-governance`: Governs Pacta-native composition patterns, adapter
  scope, and framework leakage boundaries.

### Modified Capabilities

- `domain-language`: Clarify contract isolation wording and adapter vocabulary
  boundaries.
- `runtime-skeleton`: Refine executor/driver behavior around Pacta-native
  composition and infrastructure error semantics.
- `quality-governance`: Expand executable architecture reactions beyond
  workspace dependency direction.

## Impact

- Affects OpenSpec requirements, `PROJECT.md`, `BACKLOG.md`, domain language
  documentation, and crate-level rustdoc wording.
- Affects `pacta-governance` rules for core crate dependency boundaries.
- Does not introduce Tower, HTTP, backend crates, registry implementations,
  adapter crates, or a full middleware stack in this change.
