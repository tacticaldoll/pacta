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

/// A durable command or contract, generated from a Signal, ready to be executed.
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

/// The Registry manages the lifecycle of Pacts. It is a pure state machine.
///
/// Time is injected: [`claim`](Registry::claim) and
/// [`heartbeat`](Registry::heartbeat) take the current time as a parameter, and the
/// registry reads no ambient clock. Settlement takes no time because a rotated
/// retainer already tells a stale holder apart from the current one.
pub trait Registry: Send + Sync {
    /// Error returned by the registry implementation.
    type Error: std::error::Error;

    /// Claim a pact for execution from one of the requested dockets, using `now`
    /// to set the new lease and to reclaim any pact whose lease already expired
    /// without settlement — a lapse, realized through this normal claim path.
    /// Reclaiming rotates the retainer, so the prior holder can no longer settle.
    fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error>;

    /// Extend the retainer's lease using `now`. A heartbeat presented after the
    /// lease already expired is rejected: the holder must claim again rather than
    /// revive a lapsed lease, so two holders never both hold settlement authority.
    fn heartbeat(&self, retainer: &Retainer, now: Timestamp) -> Result<(), Self::Error>;

    /// Mark the pact as successfully fulfilled. Rejected when the retainer is not
    /// the current holder.
    fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error>;

    /// Mark the pact as breached. Rejected when the retainer is not the current
    /// holder.
    fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error>;
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
