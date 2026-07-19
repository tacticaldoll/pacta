## Why

The `Registry` contract acknowledges retainer expiry only as a placeholder
comment, while `docs/domain-language.md` already names `Lapse` as recovery of a
pact whose retainer expired without settlement. A durable backend cannot be
correct without a lease-and-lapse model — at-least-once recovery depends on it —
and that correctness rests on a reciprocal user obligation (idempotent
execution) that is currently implicit. This change pins the lifecycle-persistence
contract before any backend or conformance suite is built, so those inherit a
governed contract rather than inventing one.

## What Changes

- Introduce the `lifecycle-persistence` capability as the governed contract for
  durable claim leasing and recovery, on the BACKLOG "lifecycle persistence"
  surface.
- Define the **lease** model: a claim is held for a bounded validity window;
  expiry is a claim-lifecycle state, not orchestration.
- Define **Lapse**: recovering a pact whose retainer expired without settlement
  makes it claimable again — a mechanism, not a retry policy.
- Establish **injected time**: lease expiry is decided from time supplied to the
  registry, never an ambient wall-clock read inside the core. The enforcing
  governance check lands in the change that introduces the time-taking code, so
  no requirement outruns its enforcement.
- Establish the **reciprocal obligation**: Pacta guarantees at-least-once claim
  recovery; the user's `Executor` MUST be idempotent. Exactly-once delivery
  stays deferred per BACKLOG.
- Draw the **mechanism-versus-policy** line: the registry tracks lease expiry and
  offers lapse; it does not decide whether, when, or how many times to retry,
  back off, or route to Tribunal — those stay user-owned or deferred.
- Assign **ownership**: lease duration and heartbeat cadence are user- and
  deployment-owned inputs; the registry owns only the expiry-and-lapse mechanism.
- This change pins the contract in specs and vocabulary only. The `Registry`
  trait surface (a lapse operation, lease and time types), a durable backend, and
  the conformance suite are deferred to follow-on changes so every code
  commitment is exercised the moment it is introduced.

## Capabilities

### New Capabilities
- `lifecycle-persistence`: the governed contract for durable claim leasing, lapse
  recovery, injected time, the at-least-once-versus-idempotent obligation split,
  and the mechanism-versus-policy boundary.

### Modified Capabilities
- `domain-language`: canonicalize `Lease` (the bounded claim-validity window,
  distinct from the retired "lease token") and `Lapse` (the lifecycle recovery
  operation), and add the at-least-once-versus-idempotent reciprocal-obligation
  vocabulary as governed terms.

## Impact

- Specs: new `lifecycle-persistence`; delta to `domain-language`.
- Code: none in this change. Follow-on changes add the `Registry` lapse
  operation and lease/time seam with an in-memory backend and conformance suite,
  then the first durable backend and its governance boundary.
- Docs and BACKLOG: unchanged by this change. The specs now canonicalize `Lease`
  and `Lapse` and pin the persistence contract; the `docs/domain-language.md`
  glossary already lists `Lapse`. Propagating the vocabulary into `docs/` and
  advancing the BACKLOG "Registry Conformance" area happen when the follow-on
  code lands.
- No dependency changes and no new crates in this change.
