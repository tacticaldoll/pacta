## Why

Pacta now has guarded core boundaries, but the executor crate still exposes only
a leaf execution role. Defining the smallest Pacta-native middleware and policy
skeleton now lets future retry, timeout, tracing, and rate-limit work grow from
Pacta's own composition model instead of from Tower or HTTP-shaped APIs.

## What Changes

- Add a minimal Pacta-native middleware abstraction in `pacta-executor`.
- Add a minimal policy vocabulary that can name orchestration rules without
  implementing retry, timeout, or rate limiting yet.
- Provide tests proving middleware can wrap an executor without introducing
  Tower, request/response, service, or layer vocabulary.
- Keep the driver and registry behavior unchanged except where type signatures
  naturally consume the executor surface.
- Do not add external dependencies or adapter crates.

## Capabilities

### New Capabilities

### Modified Capabilities

- `composition-governance`: Define the first concrete Pacta-native composition
  skeleton under the existing composition boundary.
- `runtime-skeleton`: Extend the executor runtime skeleton with middleware and
  policy abstractions.

## Impact

- Affects `pacta-executor` public API and its tests.
- May affect `pacta-driver` compilation if executor trait bounds or type aliases
  need small adjustments.
- Affects OpenSpec runtime and composition requirements.
- Does not introduce Tower, HTTP, backend integrations, async execution, retry
  algorithms, timeout handling, or scheduler behavior.
