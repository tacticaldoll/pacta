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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// A retainer: the authority token a registry issues with a claim and validates
/// when settling it. Authority is registry-validated — a forged identifier does not
/// match an issued claim — not proven by the type system. Construct via
/// [`Retainer::new`] and read the identifier via [`Retainer::id`].
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// A claimed pact and the retainer required to settle it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Pact claimed for execution.
    pub pact: Pact,
    /// Authority required to heartbeat, fulfill, or breach the claim.
    pub retainer: Retainer,
    // Retainer expiry information would go here.
}

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
pub trait Registry: Send + Sync {
    /// Error returned by the registry implementation.
    type Error;

    /// Claim a pact for execution from one of the requested dockets.
    fn claim(&self, dockets: &[&str]) -> Result<Option<Claim>, Self::Error>;

    /// Extend the retainer of an ongoing claim.
    fn heartbeat(&self, retainer: &Retainer) -> Result<(), Self::Error>;

    /// Mark the pact as successfully fulfilled.
    fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error>;

    /// Mark the pact as breached.
    fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error>;
}

/// The sans-I/O lifecycle kernel.
///
/// The kernel is a pure state machine: it decides the next [`Directive`](kernel::Directive)
/// from its state and absorbs [`Notice`](kernel::Notice) reports a runtime feeds
/// back. It performs no I/O
/// and exposes no `async fn`, so it commits to no runtime shape. It encodes the
/// lifecycle decision table only — it adds no orchestration behavior.
pub mod kernel {
    use crate::{Claim, Outcome, Pact, Retainer};

    /// An instruction the kernel issues for a runtime to perform.
    #[derive(Debug, Clone)]
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
    #[derive(Debug, Clone)]
    pub enum Notice {
        /// Result of a claim: a claim if one was available, else none.
        Claimed(Option<Claim>),
        /// An execution produced a lifecycle outcome.
        Executed(Outcome),
        /// An execution failed at the infrastructure level, distinct from a
        /// deliberate breach.
        ExecutionFailed,
        /// A settlement was persisted.
        Settled,
    }

    /// The terminal result of one lifecycle step.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StepResult {
        /// No pact was available to claim.
        Idle,
        /// The claim was settled with this outcome.
        Settled(Outcome),
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
                Phase::DoneIdle | Phase::DoneSettled(_) => Directive::Idle,
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
                (Phase::Executing { retainer, .. }, Notice::ExecutionFailed) => Phase::Settling {
                    retainer,
                    outcome: Outcome::Breached,
                },
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
        use uuid::Uuid;

        fn claim() -> Claim {
            Claim {
                pact: Pact {
                    id: Uuid::new_v4(),
                    docket: "default".to_string(),
                    kind: "example".to_string(),
                    clause: Vec::new(),
                },
                retainer: Retainer::new(Uuid::new_v4()),
            }
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
        fn infrastructure_error_settles_breached() {
            let mut kernel = Kernel::new();
            assert_eq!(
                drive(&mut kernel, Err(()), true),
                StepResult::Settled(Outcome::Breached)
            );
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
