//! The isolated core contract for Pacta.
//!
//! Pacta operates on three axioms:
//! 1. Registry is Lifecycle (no business logic, no retry/delay logic).
//! 2. Execution is Middleware.
//! 3. This contract has no dependency on other workspace crates.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A durable obligation, generated from a Signal, ready to be executed.
/// Note the deliberate absence of `attempts`, `delay`, and `priority`.
///
/// Construct through [`Pact::new`]; the fields stay public for reading. The type is
/// `#[non_exhaustive]` so it can gain a field in a later minor release without a
/// breaking change.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Pact {
    /// Stable identifier for this pact.
    pub id: Uuid,
    /// Logical docket from which the pact can be claimed.
    pub docket: String,
    /// Application-defined pact kind.
    pub kind: String,
    /// Business data required to fulfill the pact.
    pub clause: Vec<u8>,
}

impl Pact {
    /// Build a pact from its identifier, docket, kind, and clause.
    #[must_use]
    pub fn new(id: Uuid, docket: String, kind: String, clause: Vec<u8>) -> Self {
        Self {
            id,
            docket,
            kind,
            clause,
        }
    }
}

/// A retainer: the authority token a registry issues with a claim and validates
/// when settling it. Authority is registry-validated — a forged identifier does not
/// match an issued claim — not proven by the type system. Construct via
/// [`Retainer::new`] and read the identifier via [`Retainer::id`]. Derives
/// `PartialEq`/`Eq`/`Hash` so a durable backend can index lease state by holder
/// identity — the orphan rule makes providing these the contract's responsibility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Retainer(Uuid);

impl Retainer {
    /// Mint a retainer from an identifier. A registry issues tokens through this.
    #[must_use]
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    /// The retainer's identifier, which a registry validates on settlement.
    #[must_use]
    pub fn id(&self) -> Uuid {
        self.0
    }
}

/// A point in time as milliseconds since an epoch the runtime chooses. This is a
/// pure value: the core names time but never reads it. There is deliberately no
/// `now` constructor — a runtime obtains the current time and injects it, keeping
/// lease decisions deterministic and testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Build a timestamp from milliseconds since the runtime's chosen epoch.
    #[must_use]
    pub fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    /// The milliseconds since the runtime's chosen epoch.
    #[must_use]
    pub fn as_millis(self) -> u64 {
        self.0
    }

    /// The timestamp `millis` milliseconds after this one, saturating at the maximum.
    #[must_use]
    pub fn plus_millis(self, millis: u64) -> Self {
        Self(self.0.saturating_add(millis))
    }
}

/// A claimed pact and the retainer required to settle it.
///
/// Construct through [`Claim::new`]; the fields stay public for reading. The type is
/// `#[non_exhaustive]` so it can gain a field in a later minor release without a
/// breaking change.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Claim {
    /// Pact claimed for execution.
    pub pact: Pact,
    /// Authority required to heartbeat, fulfill, or breach the claim.
    pub retainer: Retainer,
    /// When the claim's lease expires. After this the pact may be lapsed and
    /// reclaimed unless the holder heartbeats first.
    pub lease_expiry: Timestamp,
}

impl Claim {
    /// Build a claim from a pact, the settling retainer, and the lease expiry.
    #[must_use]
    pub fn new(pact: Pact, retainer: Retainer, lease_expiry: Timestamp) -> Self {
        Self {
            pact,
            retainer,
            lease_expiry,
        }
    }
}

// The lease identity must be usable as a durable-backend key; removing the derives
// fails this build rather than silently regressing the backend contract.
const _: fn() = || {
    fn assert_key<T: Eq + std::hash::Hash>() {}
    assert_key::<Retainer>();
};

/// The lifecycle outcome an execution produces for a claimed pact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The pact was fulfilled successfully.
    Fulfilled,
    /// The pact could not be fulfilled and must be breached.
    Breached,
}

/// The lifecycle conclusion applied to a claim, currently a fulfilled or breached
/// [`Outcome`].
pub type Settlement = Outcome;

/// The pure lifecycle state machine every `Registry` backend composes over.
///
/// This is the single source of the pact lifecycle *semantics* — the claim-eligibility
/// predicate, the state transitions, the current-holder authority check, and the lease
/// arithmetic. A backend owns its own storage and mints its own retainer (a fencing
/// value); it delegates every eligibility decision and transition here, so the semantics
/// are defined once and cannot drift between backends (or between a synchronous and a
/// future asynchronous binding).
///
/// It is colorless and sans-I/O: it reads no clock (time is an injected parameter),
/// performs no I/O, and mints nothing non-deterministic (the retainer is supplied by the
/// caller). Named `lifecycle` to distinguish it from the executor step-driver
/// [`kernel`], which is a different pure machine.
pub mod lifecycle {
    use crate::{Retainer, Timestamp};

    /// A pact's position in its claim lifecycle: the pure state a backend maps to its own storage.
    /// The backend owns where it lives; this owns what it means.
    ///
    /// For the 0.2 series this is a **closed** enumeration of exactly these four variants — it is
    /// deliberately not `#[non_exhaustive]` — so a backend author knows the complete set of states to
    /// represent and can match it exhaustively, distinct from the growing `#[non_exhaustive]`
    /// protocol enums (`Directive`/`Notice`/`StepResult`) elsewhere in this crate. (This is a
    /// stability statement for 0.2.x, not a promise never to evolve the model in a later minor.)
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum State {
        /// Never claimed, or freshly seeded: immediately claimable.
        Available,
        /// Held under a lease by `retainer` until `expiry`. Claimable again only once
        /// the lease has lapsed (`expiry < now`), which rotates authority away.
        Held {
            /// The current holder's authority token.
            retainer: Retainer,
            /// When the lease expires.
            expiry: Timestamp,
        },
        /// Released non-terminally: claimable again only at or after `reclaimable_at`.
        Deferred {
            /// The instant at or after which the pact may be reclaimed.
            reclaimable_at: Timestamp,
        },
        /// Concluded (fulfilled or breached): never claimable again.
        ///
        /// This is the *model* (and reference-backend) representation of a concluded obligation, not
        /// a required storage obligation. A durable backend MAY represent settled by **removing the
        /// row** — a load of the absent row returns no state, so the pact is trivially not claimable
        /// and the prior retainer can no longer transition it, which is the whole of what settlement
        /// guarantees. Do not assume a settled pact persists.
        Settled,
    }

    /// A transition was attempted by something that is not the state's current holder —
    /// a stale retainer, or a state (available, deferred, settled) with no holder at all.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct NotCurrentHolder;

    impl std::fmt::Display for NotCurrentHolder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "retainer is not the current holder of this pact")
        }
    }

    impl std::error::Error for NotCurrentHolder {}

    /// The lease expiry for a claim taken at `now` for `lease_millis` — the single
    /// source of the lease arithmetic.
    #[must_use]
    pub fn lease_expiry(now: Timestamp, lease_millis: u64) -> Timestamp {
        now.plus_millis(lease_millis)
    }

    /// Whether a pact in `state` may be claimed at `now`: the eligibility invariant.
    /// `Available` always; a `Held` lease that has lapsed; a `Deferred` pact at or past
    /// its instant; never a `Settled` one. Only positive, unambiguous eligibility.
    #[must_use]
    pub fn is_claimable(state: &State, now: Timestamp) -> bool {
        match state {
            State::Available => true,
            State::Held { expiry, .. } => *expiry < now,
            State::Deferred { reclaimable_at } => *reclaimable_at <= now,
            State::Settled => false,
        }
    }

    /// The state a successful claim produces: `Held` by `retainer` until the lease
    /// expiry for `now`/`lease_millis`. The backend mints `retainer` and passes it in.
    #[must_use]
    pub fn on_claim(retainer: &Retainer, now: Timestamp, lease_millis: u64) -> State {
        State::Held {
            retainer: retainer.clone(),
            expiry: lease_expiry(now, lease_millis),
        }
    }

    /// The state a heartbeat produces: the lease extended to the expiry for
    /// `now`/`lease_millis`, provided `retainer` currently holds `state` and the lease
    /// has not already lapsed. A lapsed lease is not revived — the holder must re-claim.
    pub fn on_heartbeat(
        state: &State,
        retainer: &Retainer,
        now: Timestamp,
        lease_millis: u64,
    ) -> Result<State, NotCurrentHolder> {
        match state {
            State::Held {
                retainer: held,
                expiry,
            } if held == retainer && *expiry >= now => Ok(State::Held {
                retainer: retainer.clone(),
                expiry: lease_expiry(now, lease_millis),
            }),
            _ => Err(NotCurrentHolder),
        }
    }

    /// The state a settlement produces: `Settled`, provided `retainer` currently holds
    /// `state`. Fulfill and breach share this — the lifecycle state records that the
    /// obligation concluded, not which outcome concluded it.
    pub fn on_settle(state: &State, retainer: &Retainer) -> Result<State, NotCurrentHolder> {
        if is_current_holder(state, retainer) {
            Ok(State::Settled)
        } else {
            Err(NotCurrentHolder)
        }
    }

    /// The state a release produces: `Deferred` until `reclaimable_at`, provided
    /// `retainer` currently holds `state`. Non-terminal; rotates authority away.
    pub fn on_release(
        state: &State,
        retainer: &Retainer,
        reclaimable_at: Timestamp,
    ) -> Result<State, NotCurrentHolder> {
        if is_current_holder(state, retainer) {
            Ok(State::Deferred { reclaimable_at })
        } else {
            Err(NotCurrentHolder)
        }
    }

    fn is_current_holder(state: &State, retainer: &Retainer) -> bool {
        matches!(state, State::Held { retainer: held, .. } if held == retainer)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use uuid::Uuid;

        fn retainer() -> Retainer {
            Retainer::new(Uuid::new_v4())
        }

        #[test]
        fn eligibility_covers_each_state() {
            let now = Timestamp::from_millis(100);
            assert!(is_claimable(&State::Available, now));
            // A held lease is claimable only once lapsed.
            assert!(!is_claimable(
                &State::Held {
                    retainer: retainer(),
                    expiry: Timestamp::from_millis(101)
                },
                now
            ));
            assert!(is_claimable(
                &State::Held {
                    retainer: retainer(),
                    expiry: Timestamp::from_millis(99)
                },
                now
            ));
            // A deferred pact is claimable at or past its instant.
            assert!(!is_claimable(
                &State::Deferred {
                    reclaimable_at: Timestamp::from_millis(101)
                },
                now
            ));
            assert!(is_claimable(
                &State::Deferred {
                    reclaimable_at: Timestamp::from_millis(100)
                },
                now
            ));
            assert!(!is_claimable(&State::Settled, now));
        }

        #[test]
        fn transitions_require_the_current_holder() {
            let holder = retainer();
            let held = State::Held {
                retainer: holder.clone(),
                expiry: Timestamp::from_millis(200),
            };
            let stranger = retainer();

            assert_eq!(on_settle(&held, &stranger), Err(NotCurrentHolder));
            assert_eq!(
                on_release(&held, &stranger, Timestamp::from_millis(0)),
                Err(NotCurrentHolder)
            );
            assert_eq!(on_settle(&State::Settled, &holder), Err(NotCurrentHolder));

            assert_eq!(on_settle(&held, &holder), Ok(State::Settled));
            assert_eq!(
                on_release(&held, &holder, Timestamp::from_millis(500)),
                Ok(State::Deferred {
                    reclaimable_at: Timestamp::from_millis(500)
                })
            );
        }

        #[test]
        fn heartbeat_refreshes_but_does_not_revive_a_lapsed_lease() {
            let holder = retainer();
            let held = State::Held {
                retainer: holder.clone(),
                expiry: Timestamp::from_millis(200),
            };
            // Live lease refreshes.
            assert_eq!(
                on_heartbeat(&held, &holder, Timestamp::from_millis(150), 100),
                Ok(State::Held {
                    retainer: holder.clone(),
                    expiry: Timestamp::from_millis(250)
                })
            );
            // Lapsed lease is not revived.
            assert_eq!(
                on_heartbeat(&held, &holder, Timestamp::from_millis(201), 100),
                Err(NotCurrentHolder)
            );
        }
    }
}

/// A pure kernel transition decision — a [`lifecycle`] `on_X` — passed to the transition port
/// [`Registry::apply`] (and its async twin). The same type is used by both bindings, so the port is
/// literally one shape.
///
/// The `Send + Sync` bound is on the transition **closure**: it lets a backend hold the decision
/// across its own atomic scope or hand it to a worker thread. It does **not** make the async
/// binding's `apply` *future* `Send` — future coloring stays the consumer's (the async binding is
/// deliberately `Send`-agnostic at its futures). A backend that needs a `Send` `apply` future
/// satisfies that at its own concrete call site, not from this bound.
pub type Transition<'a> = dyn Fn(&lifecycle::State) -> Result<lifecycle::State, lifecycle::NotCurrentHolder>
    + Send
    + Sync
    + 'a;

/// The asynchronous binding of the [`Registry`] contract, available behind the `async` feature.
/// [`AsyncRegistry`] is the same five-op contract over the same [`Transition`] port, made async;
/// [`apply_via_cas`] is the optional compare-and-set helper. A consumer that does not enable `async`
/// compiles none of it.
#[cfg(feature = "async")]
mod async_registry;
#[cfg(feature = "async")]
pub use async_registry::{AsyncRegistry, apply_via_cas};

/// The Registry is the durable lifecycle-authority **port**: it preserves pacts and decides claim,
/// lease, and settlement authority over them. It is *not itself* the pure state machine — the pure,
/// colorless machine is [`lifecycle`], which every backend composes over; a `Registry`
/// implementation is the I/O-owning port that persists that machine's states and enforces its
/// authority (the async twin is [`AsyncRegistry`]).
///
/// A backend implements three primitives — a native [`claim`](Registry::claim) selection, a
/// [`lease_millis`](Registry::lease_millis) accessor, and an atomic [`apply`](Registry::apply)
/// transition port — and inherits heartbeat, fulfill, breach, and release as defaults over `apply`.
/// The obligations mirror the async binding exactly:
///
/// - **`claim` selects atomically, admits only an eligible pact, and rotates the retainer.** It
///   returns only a pact [`lifecycle::is_claimable`] would admit and mints a fresh retainer, all in
///   one atomic step. A durable backend expresses this as a native, full-scan-free selection.
/// - **`apply` is `load → decide → store` in one atomic scope.** It loads the state held by the
///   retainer, computes the next state through the passed [`lifecycle`] decision, and stores it
///   atomically; a non-atomic load-to-store window lets two workers both write and breaks
///   exactly-once and retainer fencing.
/// - **Reclaim — not mere expiry — rotates settlement authority.** A holder whose lease lapsed but
///   whose pact no one reclaimed is still the current holder and can still settle; authority rotates
///   only when the pact is actually reclaimed (or released). A transition against a pact the retainer
///   no longer holds surfaces as a not-current-holder error through the backend's `Error`.
///
/// `pacta-conformance` proves the *behavioral* half of this (eligibility, transitions, lapse/reclaim
/// rotation, and — via its contention checks — at-most-once claim and settlement). It does **not**
/// prove the *query-shape* obligation that `claim` is full-scan-free; a sequential functional suite
/// cannot observe query cost, so that stays a backend obligation established by review.
///
/// Time is injected: [`claim`](Registry::claim) and [`heartbeat`](Registry::heartbeat) take the
/// current time as a parameter, and the registry reads no ambient clock. Settlement takes no time
/// because a rotated retainer already tells a stale holder apart from the current one.
pub trait Registry: Send + Sync {
    /// Error returned by the registry implementation.
    type Error: std::error::Error;

    /// Claim a pact for execution from one of the requested dockets, using `now`
    /// to set the new lease and to reclaim any pact whose lease already expired
    /// without settlement — a lapse, realized through this normal claim path.
    ///
    /// **Obligation:** select — atomically — only a pact [`lifecycle::is_claimable`] would admit
    /// (available, a lapsed hold, or a deferred pact past its instant; never a settled one) and mint
    /// a fresh retainer, so reclaiming rotates authority and the prior holder can no longer settle. A
    /// durable backend expresses this as a native, full-scan-free selection (for example SQL
    /// `SKIP LOCKED`), not by loading the whole docket to filter in memory.
    fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error>;

    /// The backend's lease duration in milliseconds, used by [`heartbeat`](Registry::heartbeat)
    /// to compute the extended lease. Lease sizing is the backend's; the contract supplies the
    /// mechanism, not a constant.
    fn lease_millis(&self) -> u64;

    /// Apply a lifecycle transition to the pact held by `retainer`, within the backend's own
    /// atomic scope. `transition` is the pure kernel decision (a [`lifecycle`] `on_X`): the
    /// backend loads the held state, computes the next state through `transition`, and applies
    /// it atomically — it never decides the transition itself, so the lifecycle semantics stay
    /// single-sourced in the kernel and cannot drift. A transition applied against a pact the
    /// retainer no longer holds resolves to a not-current-holder error (the kernel's
    /// [`NotCurrentHolder`](lifecycle::NotCurrentHolder) surfaces through `transition`). This is
    /// the one transition port; the four transition operations below are provided over it.
    ///
    /// The backend owns *how* the scope is made atomic (a lock, a transaction, a native
    /// conditional write, or compare-and-set); the contract mandates no concurrency-control
    /// mechanism.
    fn apply(&self, retainer: &Retainer, transition: &Transition<'_>) -> Result<(), Self::Error>;

    /// Extend the retainer's lease using `now`. A heartbeat presented after the
    /// lease already expired is rejected: the holder must claim again rather than
    /// revive a lapsed lease, so two holders never both hold settlement authority.
    fn heartbeat(&self, retainer: &Retainer, now: Timestamp) -> Result<(), Self::Error> {
        let lease = self.lease_millis();
        self.apply(retainer, &|state| {
            lifecycle::on_heartbeat(state, retainer, now, lease)
        })
    }

    /// Mark the pact as successfully fulfilled. Rejected when the retainer is not
    /// the current holder.
    fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.apply(retainer, &|state| lifecycle::on_settle(state, retainer))
    }

    /// Mark the pact as breached. Rejected when the retainer is not the current
    /// holder. Shares the settlement transition with [`fulfill`](Registry::fulfill) — the
    /// lifecycle records that the obligation concluded, not which outcome concluded it.
    fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.apply(retainer, &|state| lifecycle::on_settle(state, retainer))
    }

    /// Release the claim without concluding the obligation, making the pact
    /// reclaimable again only at or after `reclaimable_at`.
    ///
    /// This is **non-terminal**: unlike [`fulfill`](Registry::fulfill) and
    /// [`breach`](Registry::breach), it settles nothing — the pact is left to be
    /// attempted again. The registry computes no delay: `reclaimable_at` is a
    /// consumer-supplied instant, honored exactly as the injected `now` is honored
    /// (compared, never computed), so backoff policy stays with the caller and `Pact`
    /// carries no delay. A `reclaimable_at` at or before now makes the pact immediately
    /// claimable, as a voluntary lapse. Release rotates authority like a lapse, so the
    /// prior retainer can no longer settle or heartbeat. Rejected when the retainer is
    /// not the current holder.
    fn release(&self, retainer: &Retainer, reclaimable_at: Timestamp) -> Result<(), Self::Error> {
        self.apply(retainer, &|state| {
            lifecycle::on_release(state, retainer, reclaimable_at)
        })
    }
}

/// The sans-I/O lifecycle kernel.
///
/// The kernel is a pure state machine: it decides the next [`Directive`](kernel::Directive)
/// from its state and absorbs [`Notice`](kernel::Notice) reports a runtime feeds
/// back. It performs no I/O
/// and exposes no `async fn`, so it commits to no runtime shape. It encodes the
/// lifecycle decision table only — it adds no orchestration behavior.
///
/// # Advanced surface
///
/// This is the **advanced** tier of Pacta's public API: lower stability intent than
/// the recommended surface (its API may evolve as the runtime story settles), though
/// it stays a supported, governed core surface — not unsupported or slated for
/// removal. Most consumers should compose with the `Driver` runtime (or the `pacta`
/// facade) and never touch the kernel. Reach for it only to build a custom runtime;
/// it is reached through `pacta-contract` directly, never through the `pacta` facade.
///
/// # Driving the kernel
///
/// A runtime drives one lifecycle step by looping: ask [`poll`](kernel::Kernel::poll)
/// for the next [`Directive`](kernel::Directive), perform it, report the outcome back with
/// [`on_event`](kernel::Kernel::on_event), and repeat until [`result`](kernel::Kernel::result)
/// yields a terminal [`StepResult`](kernel::StepResult). The kernel decides *what*;
/// the runtime performs it and injects time — the kernel reads no clock.
///
/// ```
/// use pacta_contract::kernel::{Directive, Kernel, Notice, StepResult};
/// use pacta_contract::{Claim, Outcome, Pact, Retainer, Timestamp};
///
/// let mut kernel = Kernel::new();
/// let mut available = Some(Claim::new(
///     Pact::new(Default::default(), "demo".into(), "demo".into(), Vec::new()),
///     Retainer::new(Default::default()),
///     Timestamp::from_millis(0),
/// ));
///
/// let result = loop {
///     if let Some(result) = kernel.result() {
///         break result;
///     }
///     match kernel.poll() {
///         Directive::Claim => kernel.on_event(Notice::Claimed(available.take())),
///         Directive::Execute(_pact) => kernel.on_event(Notice::Executed(Outcome::Fulfilled)),
///         Directive::Settle(_retainer, _outcome) => kernel.on_event(Notice::Settled),
///         Directive::Idle => break StepResult::Idle,
///         _ => unreachable!("driver handles every current kernel directive"),
///     }
/// };
///
/// assert_eq!(result, StepResult::Settled(Outcome::Fulfilled));
/// ```
pub mod kernel {
    use crate::{Claim, Outcome, Pact, Retainer};

    /// An instruction the kernel issues for a runtime to perform.
    ///
    /// `#[non_exhaustive]`: this advanced-tier protocol may gain directives, so a
    /// runtime's match must carry a wildcard arm.
    #[derive(Debug, Clone)]
    #[non_exhaustive]
    pub enum Directive {
        /// Claim a pact from the runtime's configured dockets.
        Claim,
        /// Execute the given claimed pact.
        Execute(Pact),
        /// Settle the claim identified by the retainer with the decided outcome.
        Settle(Retainer, Outcome),
        /// Nothing remains to be performed for this step.
        Idle,
    }

    /// A report a runtime feeds back after performing a [`Directive`].
    ///
    /// `#[non_exhaustive]`: this advanced-tier protocol may gain notices, so a
    /// consumer's match must carry a wildcard arm.
    #[derive(Debug, Clone)]
    #[non_exhaustive]
    pub enum Notice {
        /// Result of a claim: a claim if one was available, else none.
        Claimed(Option<Claim>),
        /// An execution produced a lifecycle outcome.
        Executed(Outcome),
        /// The execution infrastructure failed to run the pact — the executor
        /// produced no outcome. The kernel fabricates no outcome for this notice: it
        /// settles nothing and reaches an unsettled terminal, leaving the claim to
        /// lapse and be reclaimed, while the runtime surfaces the error to its caller.
        ExecutionFailed,
        /// A settlement was persisted.
        Settled,
    }

    /// The terminal result of one lifecycle step.
    ///
    /// `#[non_exhaustive]`: this advanced-tier protocol may gain results, so a
    /// consumer's match must carry a wildcard arm.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum StepResult {
        /// No pact was available to claim.
        Idle,
        /// The claim was settled with this outcome.
        Settled(Outcome),
        /// Execution produced no outcome (an infrastructure failure), so nothing was
        /// settled. The claim is left held-but-unsettled to lapse and be reclaimed;
        /// the kernel fabricates no `Outcome` from the absence of one.
        Unsettled,
    }

    #[derive(Debug)]
    enum Phase {
        Claiming,
        Executing {
            pact: Pact,
            retainer: Retainer,
        },
        Settling {
            retainer: Retainer,
            outcome: Outcome,
        },
        DoneIdle,
        DoneSettled(Outcome),
        DoneUnsettled,
    }

    /// The pure lifecycle state machine for a single step.
    #[derive(Debug)]
    pub struct Kernel {
        phase: Phase,
    }

    impl Kernel {
        /// Start a fresh lifecycle step.
        #[must_use]
        pub fn new() -> Self {
            Self {
                phase: Phase::Claiming,
            }
        }

        /// Decide the next directive from the current state.
        #[must_use]
        pub fn poll(&self) -> Directive {
            match &self.phase {
                Phase::Claiming => Directive::Claim,
                Phase::Executing { pact, .. } => Directive::Execute(pact.clone()),
                Phase::Settling { retainer, outcome } => {
                    Directive::Settle(retainer.clone(), *outcome)
                }
                Phase::DoneIdle | Phase::DoneSettled(_) | Phase::DoneUnsettled => Directive::Idle,
            }
        }

        /// Absorb a runtime report, advancing the lifecycle.
        pub fn on_event(&mut self, notice: Notice) {
            let phase = std::mem::replace(&mut self.phase, Phase::DoneIdle);
            self.phase = match (phase, notice) {
                (Phase::Claiming, Notice::Claimed(Some(claim))) => Phase::Executing {
                    pact: claim.pact,
                    retainer: claim.retainer,
                },
                (Phase::Claiming, Notice::Claimed(None)) => Phase::DoneIdle,
                (Phase::Executing { retainer, .. }, Notice::Executed(outcome)) => {
                    Phase::Settling { retainer, outcome }
                }
                (Phase::Executing { .. }, Notice::ExecutionFailed) => Phase::DoneUnsettled,
                (Phase::Settling { outcome, .. }, Notice::Settled) => Phase::DoneSettled(outcome),
                (other, _) => other,
            };
        }

        /// The terminal result once the step reaches a terminal state, else `None`.
        #[must_use]
        pub fn result(&self) -> Option<StepResult> {
            match &self.phase {
                Phase::DoneIdle => Some(StepResult::Idle),
                Phase::DoneSettled(outcome) => Some(StepResult::Settled(*outcome)),
                Phase::DoneUnsettled => Some(StepResult::Unsettled),
                _ => None,
            }
        }
    }

    impl Default for Kernel {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::Timestamp;
        use uuid::Uuid;

        fn claim() -> Claim {
            Claim::new(
                Pact::new(
                    Uuid::new_v4(),
                    "default".to_string(),
                    "example".to_string(),
                    Vec::new(),
                ),
                Retainer::new(Uuid::new_v4()),
                Timestamp::from_millis(0),
            )
        }

        fn drive(
            kernel: &mut Kernel,
            execution: Result<Outcome, ()>,
            claimable: bool,
        ) -> StepResult {
            loop {
                if let Some(result) = kernel.result() {
                    return result;
                }
                match kernel.poll() {
                    Directive::Claim => {
                        let notice = if claimable {
                            Notice::Claimed(Some(claim()))
                        } else {
                            Notice::Claimed(None)
                        };
                        kernel.on_event(notice);
                    }
                    Directive::Execute(_) => kernel.on_event(match execution {
                        Ok(outcome) => Notice::Executed(outcome),
                        Err(()) => Notice::ExecutionFailed,
                    }),
                    Directive::Settle(_, _) => kernel.on_event(Notice::Settled),
                    Directive::Idle => return StepResult::Idle,
                }
            }
        }

        #[test]
        fn fulfilled_execution_settles_fulfilled() {
            let mut kernel = Kernel::new();
            assert_eq!(
                drive(&mut kernel, Ok(Outcome::Fulfilled), true),
                StepResult::Settled(Outcome::Fulfilled)
            );
        }

        #[test]
        fn breached_execution_settles_breached() {
            let mut kernel = Kernel::new();
            assert_eq!(
                drive(&mut kernel, Ok(Outcome::Breached), true),
                StepResult::Settled(Outcome::Breached)
            );
        }

        #[test]
        fn infrastructure_error_is_unsettled() {
            let mut kernel = Kernel::new();
            assert_eq!(drive(&mut kernel, Err(()), true), StepResult::Unsettled);
        }

        #[test]
        fn empty_claim_is_idle() {
            let mut kernel = Kernel::new();
            assert_eq!(
                drive(&mut kernel, Ok(Outcome::Fulfilled), false),
                StepResult::Idle
            );
        }
    }
}
