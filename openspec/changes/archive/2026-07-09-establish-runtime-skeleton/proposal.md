## Why

Pacta has a contract vocabulary, quality gates, and the isolated
`pacta-contract` crate, but it still lacks the first executable runtime shape
that proves "Execution is Middleware" in code. Establishing the branded runtime
language before the skeleton keeps Pacta-native names at the center while still
leaving room for Tower as an adapter target instead of the core API identity.

## What Changes

- Define the runtime skeleton's public naming style before adding new crates:
  Pacta domain terms first, clear mechanical terms only where they improve
  implementation clarity.
- Add a minimal Pacta-native executor surface that treats execution as
  middleware-oriented pact fulfillment, not as a mandatory Tower `Service`.
- Add a `pacta-driver` skeleton for the mechanical loop that claims pacts from a
  `Registry` by docket and dispatches them to a Pacta executor.
- Keep Tower integration out of the core runtime skeleton unless it appears as
  an optional adapter or a separately scoped future change.
- Add enough tests or examples to prove the driver settles successful execution
  with `fulfill` and failed execution with `breach`.
- Update workspace governance so new runtime crates have explicit Tianheng
  boundaries.
- Update README, roadmap, and development docs only where the new crate layout
  changes contributor-facing truth.
- Do not add registry backends, persistence, retry/backoff, scheduling,
  rate-limiting, Tribunal handling, a conformance suite, or a Tower-first public
  API in this change.

## Capabilities

### New Capabilities

- `runtime-skeleton`: Defines Pacta's initial executor and driver skeleton,
  including how the driver composes `Registry` with Pacta-native execution.

### Modified Capabilities

- `quality-governance`: Current workspace crates remain covered by executable
  safety, docs, CI, and Tianheng governance reactions as runtime crates are
  added.
- `domain-language`: Project positioning and runtime skeleton names stop
  describing Pacta as Tower-native and instead describe Tower as an optional
  middleware adapter target.

## Impact

- Adds `crates/pacta-executor` and `crates/pacta-driver`.
- Adds only the minimal async/test dependencies required for the skeleton.
- Updates `crates/pacta-governance` with boundaries for the new crates.
- Updates workspace manifests, `PROJECT.md`, README, and backlog/development
  docs as needed.
