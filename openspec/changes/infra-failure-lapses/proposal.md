## Why

The kernel is a pure state machine everywhere but one transition: on
`Notice::ExecutionFailed` it decides `Settling { outcome: Breached }` — it
**fabricates** an `Outcome` it was never given. An infrastructure failure (the
executor could not run: a downstream blip, an OOM, a dropped connection) is the
*absence* of a lifecycle outcome, not a business breach. Collapsing the two makes
the lifecycle punish honesty: a holder that silently dies is recovered by
lease/lapse (at-least-once), but a holder that honestly reports `Err` is
**terminally breached** and never retried. The more truthfully the executor
reports, the worse the outcome — an inverted incentive, and a violation of the
at-least-once recovery `lifecycle-persistence` promises.

This is the kernel's sole impurity. The fix is subtraction: the kernel stops
fabricating an outcome from an absence, leaving the claim unsettled so it lapses
and is reclaimed. Failure *disposition* (retry, give up, fail fast) becomes a
composed concern at the existing `Middleware` seam — the core owns the mechanism,
the edge owns the policy — rather than a decision baked into the state machine.

## What Changes

- The kernel SHALL NOT fabricate an `Outcome` from `Notice::ExecutionFailed`. The
  transition becomes a neutral unsettled terminal (`StepResult::Unsettled`), not a
  breach settlement. The kernel settles only outcomes it was actually given.
- The reference driver SHALL stop settling a breach on an executor error: it
  surfaces the executor error to the caller (unchanged) and settles nothing, so the
  claim is left held-but-unsettled and lapses for reclaim at lease expiry.
- Failure disposition is documented as a composed concern: a user wanting fail-fast
  wraps the executor in a `Middleware` that converts `Err` into `Ok(Outcome::Breached)`;
  a user wanting retry composes a retry `Middleware` (deferred). The core ships the
  zero-policy default (surface + lapse) and no new trait or middleware type.
- The BACKLOG "Infrastructure-failure handling during execution" reconsideration is
  resolved by this change.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `runtime-skeleton`: the kernel treats an executor infrastructure error as an
  **unsettled** terminal (fabricating no outcome), and the driver settles nothing
  and surfaces the error, leaving the claim to lapse.
- `lifecycle-persistence`: at-least-once recovery explicitly covers an
  infrastructure failure — it is recovered through lapse, not terminally breached.

## Impact

- Code: `pacta-contract` kernel (`Phase`, `on_event`, `poll`, `result`, a new
  `StepResult::Unsettled`, the `Notice::ExecutionFailed` doc, the kernel doctest);
  `pacta-driver` `step()` (remove the breach-on-error settlement; map the kernel's
  unsettled terminal to returning the executor error).
- `StepResult` is already `#[non_exhaustive]`, so adding `Unsettled` compiles
  cleanly downstream; but the reference driver's and doctest's wildcards are
  `unreachable!()`, so they gain an explicit arm here (a custom runtime whose
  wildcard panics/no-ops must add one too — compile-compatible, not behavior-free).
- **Behavioral change (pre-1.0):** infrastructure failure now lapses instead of
  breaching. A consumer relying on breach-on-infra-failure observes different
  behavior. Under 0.x SemVer this likely warrants a **minor** bump (0.2.0), not a
  patch — the version-line decision belongs to the deferred release-finalization
  step, not this change (manifests stay untouched here).
- No new dependency, no new public trait, no new middleware type.
