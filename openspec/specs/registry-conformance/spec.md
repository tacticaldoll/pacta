# registry-conformance Specification

## Purpose
Define Pacta's backend-agnostic conformance suite that verifies any `Registry` backend against the same lease lifecycle — claim, settlement, lapse, and heartbeat, including the at-least-once safety property — through the public trait with injected time and a single seeding hook.
## Requirements
### Requirement: Backend-Agnostic Conformance Suite
Pacta SHALL provide a conformance suite that verifies `Registry` lifecycle
behavior independent of any particular backend, so every backend is held to the
same correctness contract.

#### Scenario: The same suite runs against any backend
- **WHEN** a `Registry` backend is subjected to the conformance suite
- **THEN** the suite exercises it through the public `Registry` trait without
  depending on backend-specific internals

#### Scenario: Time is driven through the trait
- **WHEN** the conformance suite exercises time-dependent behavior such as lease
  expiry
- **THEN** it advances time by passing controlled `Timestamp` values into the
  trait rather than waiting on a wall clock

#### Scenario: Claims are scoped to requested dockets
- **WHEN** the conformance suite claims from a docket while a pact sits on a
  different docket
- **THEN** the pact is not claimed, and it is claimable only when its own docket
  is requested

### Requirement: Conformance Seeding Hook
The conformance suite SHALL populate a backend under test through a single defined
seeding hook, so the suite stays generic while each backend supplies its own way
of holding pacts.

#### Scenario: The suite populates a backend under test
- **WHEN** the conformance suite needs a backend containing known pacts
- **THEN** it constructs the backend through the defined seeding hook rather than
  reaching into backend-specific storage

### Requirement: Conformance Covers Lease Lifecycle
The conformance suite SHALL verify claim, settlement, lapse, and heartbeat
behavior, including the at-least-once safety property that a reclaimed pact
rejects the prior holder's authority.

#### Scenario: A claimed pact is settled and no longer claimable
- **WHEN** the suite claims a pact and then fulfills or breaches it
- **THEN** the pact is settled and a further claim returns nothing

#### Scenario: An expired lease is reclaimed
- **WHEN** the suite lets a lease expire and then claims again at a later time
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: A reclaimed pact rejects the prior holder
- **WHEN** the suite reclaims a lapsed pact and the original holder then settles
  with its prior retainer
- **THEN** the settlement is rejected

#### Scenario: Heartbeat guards the lease
- **WHEN** the suite heartbeats within the lease window and again after expiry
- **THEN** the in-window heartbeat extends the lease and the post-expiry heartbeat
  is rejected

### Requirement: Conformance Covers Deferred Reclaim
The conformance suite SHALL verify release and deferred reclaim for any backend: that a
released pact is not claimable before its reclaimable instant, is claimable at or after it,
that an immediate reclaimable instant behaves as a lapse, and that release rotates authority
away from the prior retainer.

#### Scenario: A released pact is withheld until its reclaimable instant
- **WHEN** the suite releases a claim with a future reclaimable instant and claims again before it
- **THEN** the claim returns nothing

#### Scenario: A released pact is reclaimable at its instant
- **WHEN** the suite claims again at or after the reclaimable instant
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: An immediate reclaimable instant reclaims like a lapse
- **WHEN** the suite releases a claim with a reclaimable instant at or before now and claims again
- **THEN** the pact is reclaimed through the normal claim path

#### Scenario: Release rotates authority away from the prior holder
- **WHEN** the suite releases a claim and the prior holder then settles with its retainer
- **THEN** the settlement is rejected

### Requirement: Conformance Is Offered For The Async Binding
The conformance suite SHALL verify an `AsyncRegistry` backend against the same lifecycle scenarios it
verifies sync `Registry` backends against, so the async binding is held to the same correctness
contract as the sync binding. The async runner SHALL reuse the sync suite's scenarios rather than a
duplicated scenario set, so sync and async coverage cannot drift. The async runner SHALL be gated
behind the conformance suite's `async` feature, so a sync-only consumer that does not enable it
compiles no async binding code and no async runtime.

The suite SHALL offer the async scenario set through two entries over the one shared scenario
definition, so a real durable backend can run it on its own runtime while the reference backend
keeps a zero-dependency path:

- A **runtime-compatible** entry that drives the shared scenarios through a caller-supplied driver,
  so a backend whose futures require an external reactor runs the same scenarios on its own runtime.
  The entry SHALL NOT force a runtime or an executor coloring on the backend: the driver abstraction
  SHALL impose no `Send` bound on the futures it drives and SHALL add no async-runtime *normal*
  dependency to the conformance crate.
- A **ready-future convenience** entry (the existing runner) that drives the shared scenarios with
  the suite's own poll-based `block_on`, correct only for backends whose futures make progress
  without an external reactor. Its documentation SHALL state that limitation rather than implying
  any async backend can run it.

Both entries SHALL drive the one shared scenario definition, so a real backend does not re-declare
the scenario set and coverage cannot drift between the two entries.

#### Scenario: The async binding runs the same scenarios
- **WHEN** an `AsyncRegistry` backend is subjected to the async conformance runner
- **THEN** it is exercised against the same lifecycle scenarios (claim, settlement, lapse, heartbeat,
  deferred reclaim, at-least-once safety) the sync suite runs, through the public `AsyncRegistry`
  trait

#### Scenario: Coverage is single-sourced, not duplicated
- **WHEN** either async entry exercises a scenario
- **THEN** it drives the same scenario definition the sync suite uses rather than a parallel copy, so
  a change to a scenario applies to both bindings and both entries at once

#### Scenario: A real-reactor backend runs the suite on its own runtime
- **WHEN** a backend whose futures require an external reactor is subjected to the runtime-compatible entry with a driver wrapping its own runtime
- **THEN** the shared scenarios run to completion on that runtime, and the entry imposes no `Send` bound on the backend's futures and adds no async-runtime normal dependency to the conformance crate

#### Scenario: The ready-future entry states its limitation
- **WHEN** a consumer reads the ready-future convenience entry's documentation
- **THEN** it states the entry is correct only for backends whose futures make progress without an external reactor, and points a real-reactor backend at the runtime-compatible entry

#### Scenario: Sync-only consumers compile no async
- **WHEN** a sync-only consumer builds the conformance suite without enabling its `async` feature
- **THEN** the build compiles no async binding code and pulls no async runtime, because the async runner is behind the feature

### Requirement: Conformance Covers Concurrent Transition Contention
The conformance suite SHALL provide contention checks that verify concurrent transitions contending
on a single claimed pact preserve at-most-once application — the invariant every backend's `apply`
must uphold regardless of whether it is implemented with a lock, a transaction, or compare-and-set.
The check SHALL assert the outcome through the `apply` transition port without assuming a particular
concurrency-control mechanism. The suite SHALL provide this contention check for **both** the sync
`Registry` binding and the async `AsyncRegistry` binding. It SHALL drive concurrency with OS threads
and (for the async binding) the suite's own poll-based `block_on`, pulling no async runtime, so it
stays within the suite's dependency budget and forces no executor choice on a backend.

The contention check MAY use a repeated probabilistic stress to surface an interleaving, but its
documentation SHALL describe that honestly as a probabilistic stress rather than a deterministic
proof. The suite's non-vacuity SHALL instead be established by a deterministic guard (see *The
Contention Harness Is Proven Non-Vacuous*).

#### Scenario: Contending transitions apply at most once
- **WHEN** two workers concurrently attempt a transition on the same claimed pact
- **THEN** exactly one transition succeeds and the other resolves to a not-current-holder, so the transition is applied at most once

#### Scenario: The sync binding has a transition-contention check
- **WHEN** a sync `Registry` backend is subjected to the suite's contention check
- **THEN** two OS threads contend a settlement on one claimed pact and exactly one succeeds, so the sync binding proves the at-most-once invariant, not only the async binding

#### Scenario: The invariant is backend-mechanism-agnostic
- **WHEN** the concurrent-contention check runs against a backend
- **THEN** it asserts the at-most-once outcome through the `apply` port, not by inspecting whether the backend used a lock, a transaction, or compare-and-set

#### Scenario: The contention is exercised under real parallelism
- **WHEN** the concurrent-contention check runs
- **THEN** it runs under genuine multi-threaded parallelism (OS threads), because a backend's ready futures do not interleave when driven to completion on a single thread

#### Scenario: The contention check pulls no async runtime
- **WHEN** a backend runs the contention check
- **THEN** the check drives its concurrency with OS threads and the suite's poll-based `block_on`, depending on no async runtime, so the suite's dependency budget is unchanged and the backend is not forced onto a particular executor

#### Scenario: The probabilistic stress is described honestly
- **WHEN** the contention check repeats many rounds to surface an interleaving
- **THEN** its documentation describes that repetition as a probabilistic stress, not as a deterministic proof that a non-atomic backend is caught on every run

### Requirement: Conformance Covers Concurrent Claim Contention
The conformance suite SHALL provide a contention check that verifies concurrent claims on a single
available pact issue at most one claim — the invariant a backend's native `claim` selection must
uphold. Two workers SHALL contend to claim the one available pact through the public claim
operation; exactly one SHALL receive a `Claim` and the other SHALL receive `None`, and two valid
retainers SHALL NOT be issued for the same pact. The suite SHALL provide this check for **both** the
sync `Registry` binding and the async `AsyncRegistry` binding, driving concurrency with OS threads
and (for the async binding) the suite's own poll-based `block_on`, pulling no async runtime.

#### Scenario: Two contending claims issue at most one claim
- **WHEN** two workers concurrently claim from a docket holding exactly one available pact
- **THEN** exactly one worker receives a `Claim` and the other receives `None`, so no two workers ever both hold authority over the same pact

#### Scenario: The claim-contention check covers both bindings
- **WHEN** the concurrent-claim check is offered
- **THEN** it is runnable against both a sync `Registry` backend and an async `AsyncRegistry` backend, through each binding's public claim operation

#### Scenario: The claim-contention check asserts through the public API
- **WHEN** the concurrent-claim check runs against a backend
- **THEN** it decides the outcome through the public claim operation only, not by inspecting the backend's locking, transaction, or compare-and-set mechanism

### Requirement: The Contention Harness Is Proven Non-Vacuous
The suite SHALL prove that its contention checks actually catch a non-atomic backend, so a
contention gate cannot pass forever against a broken backend and read as coverage it does not
provide. A deterministic, barrier-synchronized non-atomic fixture — whose contended operation loads
the state, waits until both contending workers have loaded the same pre-state, then stores, so a
double application is forced rather than left to chance — SHALL make the corresponding contention
check fail. A matching atomic fixture SHALL pass. Both the settlement-contention and the
claim-contention branches SHALL be covered by such a guard. This mirrors the project's "reactions are
proven to fire" discipline for governance.

#### Scenario: A non-atomic apply fails the settlement-contention check
- **WHEN** the settlement-contention check runs against a deterministically non-atomic `apply` fixture that lets both workers observe the same pre-state before either stores
- **THEN** the check fails, because the fixture double-applies the transition and violates at-most-once

#### Scenario: A non-atomic claim fails the claim-contention check
- **WHEN** the claim-contention check runs against a deterministically non-atomic `claim` fixture that lets both workers observe the pact available before either marks it held
- **THEN** the check fails, because the fixture issues two claims for the same pact

#### Scenario: A matching atomic fixture passes
- **WHEN** the contention checks run against a matching fully atomic fixture
- **THEN** they pass, so each guard distinguishes a harness with teeth from one that always passes

#### Scenario: The guard is deterministic
- **WHEN** the non-vacuity guard forces the interleaving
- **THEN** it uses a barrier so both workers load the same pre-state before either stores, making the double application deterministic rather than reliant on a lucky interleaving

### Requirement: Conformance Covers The Heartbeat Expiry Boundary
The conformance suite SHALL verify the `heartbeat` lease boundary at `now == expiry`: a heartbeat
presented exactly at the lease-expiry instant SHALL succeed and extend the lease, because the lease
is still valid at its expiry instant and only a strictly later heartbeat is rejected. This pins the
`now == expiry` edge of the reclaim-fence, complementing the existing after-expiry rejection.

#### Scenario: A heartbeat at the expiry instant extends the lease
- **WHEN** the suite heartbeats a claim at `now` equal to its lease expiry
- **THEN** the heartbeat succeeds and extends the lease, so a subsequent claim before the new expiry finds the pact still held

#### Scenario: The boundary is single-sourced across bindings
- **WHEN** the heartbeat-boundary scenario runs
- **THEN** it is part of the shared scenario set, so both the sync and async bindings inherit it without a duplicated definition

### Requirement: Conformance Proves Behavior, Not Query Shape
The suite's documentation SHALL distinguish the properties the current sequential scenarios prove
from the properties they do not, so a backend author does not read more assurance into a passing run
than it provides. The suite proves *behavioral* equivalence to the shared lifecycle — eligibility
admission, transition outcomes, lapse and reclaim rotation, and (through the contention checks)
at-most-once claim and transition. It SHALL NOT be claimed to prove a backend's *query shape* — that
claim selection is full-scan-free — because a sequential functional suite cannot observe query cost.
The full-scan-free selection is a separate obligation on the backend, established by review, not by
this suite.

#### Scenario: Behavioral properties are attributed to the suite
- **WHEN** documentation states what a passing conformance run proves
- **THEN** it attributes eligibility, transition, lapse/reclaim, and at-most-once contention outcomes to the suite

#### Scenario: Query-shape obligation is not attributed to the suite
- **WHEN** documentation describes the full-scan-free claim obligation
- **THEN** it states that the sequential conformance suite does not prove it, keeping the behavior proof separate from the query-shape obligation

