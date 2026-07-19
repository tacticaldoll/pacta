## Why

The `lifecycle-persistence` contract is pinned but unexercised: no type models a
lease, no `Registry` method carries time, and no backend proves the contract can
be implemented. This change makes the contract real by adding the injected-time
seam, a first in-memory backend, and a backend-agnostic conformance suite that
any future backend must pass — so the lease, lapse, and at-least-once behaviors
are exercised the moment they are introduced, and the governance backstop that
keeps the core clock-free lands with the code it guards.

## What Changes

- Add a pure `Timestamp` value type to `pacta-contract` with no `now()`
  constructor: the core names time but never produces "the current time".
- Add the **injected-time seam** to `Registry`: `claim` and `heartbeat` accept a
  `now: Timestamp` parameter; `fulfill` and `breach` take no time, because a
  rotated retainer already invalidates a stale holder.
- Realize **lapse through the claim path**: `claim` reclaims a pact whose lease
  expired without settlement and rotates its retainer, invalidating the prior
  holder's authority. Lapse is emergent behavior of `claim`, not a separate
  method.
- Reject **heartbeat on a lapsed lease**: a holder whose lease already expired
  must re-claim rather than revive, so two holders never both hold valid
  settlement authority. This is a settlement guarantee, not a concurrency one — a
  lapsed holder may still be executing, which is why the user's executor must be
  idempotent.
- Add `pacta-memory`: the first `Registry` backend, a formal in-memory
  implementation with real lease and lapse semantics and zero external
  dependencies beyond `uuid`.
- Add `pacta-conformance`: a backend-agnostic suite of behavior tests that drives
  time through the trait and seeds a backend through one defined hook, so every
  backend is held to the same lifecycle correctness.
- Teach the `Driver` to obtain the current time once per step and inject it into
  the time-dependent registry operations. The sans-I/O kernel is unchanged: it
  still issues time-free directives; only the runtime reads the clock.
- Add the **injected-time backstop**: a governance check that rejects ambient
  wall-clock reads (`*::now()`) inside `pacta-contract`, plus Tianheng dependency
  boundaries for the two new crates.

## Capabilities

### New Capabilities
- `registry-conformance`: the backend-agnostic conformance suite and the seeding
  hook it requires; the correctness contract every `Registry` backend must pass.

### Modified Capabilities
- `lifecycle-persistence`: pin the concrete decisions the implementation makes —
  injected time as a call parameter, lapse rotating authority through the claim
  path, and heartbeat refusing to revive a lapsed lease.
- `quality-governance`: require that `pacta-contract` reads no ambient time,
  enforced by an executable governance check.
- `runtime-skeleton`: the runtime supplies the current time to time-dependent
  registry operations while the kernel stays time-free.

## Impact

- Code: `pacta-contract` gains `Timestamp` and the `Registry` time seam;
  `pacta-driver` injects time; new crates `pacta-memory` and `pacta-conformance`.
- Dependencies: `pacta-memory` depends on `pacta-contract` and `uuid`;
  `pacta-conformance` depends on `pacta-contract` and `uuid` (it builds seed pacts,
  whose ids are uuids); both governed by new Tianheng boundaries. No new
  dependency enters the existing core crates.
- Governance: `pacta-governance` gains the ambient-time source scan. Tianheng
  coverage is advisory (it warns but never fails CI), so a new governance test
  asserts every workspace crate has a dependency boundary, making a forgotten
  boundary fail rather than merely warn.
- Breaking: `Registry::claim` and `Registry::heartbeat` signatures gain a
  `now: Timestamp` parameter. Existing in-crate impls (driver tests, the
  composition example) are updated in this change.
