//! Pacta-native execution abstractions.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use pacta_contract::Pact;

/// A single attempt to fulfill a claimed pact.
#[derive(Debug, Clone)]
pub struct Execution {
    /// Pact being executed.
    pub pact: Pact,
}

impl Execution {
    /// Build an execution from a claimed pact.
    #[must_use]
    pub fn new(pact: Pact) -> Self {
        Self { pact }
    }
}

/// Result reported by an executor after handling an execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The pact was fulfilled successfully.
    Fulfilled,
    /// The pact could not be fulfilled and must be breached.
    Breached,
}

/// Lifecycle settlement requested by execution.
pub type Settlement = Outcome;

/// Public role responsible for executing claimed pacts through middleware.
pub trait Executor {
    /// Error returned when the execution infrastructure fails.
    type Error;

    /// Execute a claimed pact.
    fn execute(&mut self, execution: Execution) -> Result<Outcome, Self::Error>;
}
