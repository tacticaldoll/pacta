//! The pure, zero-dependency contract for Pacta.
//!
//! Pacta operates on three axioms:
//! 1. Registry is Lifecycle (no business logic, no retry/delay logic).
//! 2. Execution is Middleware.
//! 3. This contract is zero-dependency.

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

/// An opaque token proving authority to settle a specific claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retainer(
    /// Opaque retainer identifier.
    pub Uuid,
);

/// A claimed pact and the retainer required to settle it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Pact claimed for execution.
    pub pact: Pact,
    /// Authority required to heartbeat, fulfill, or breach the claim.
    pub retainer: Retainer,
    // Retainer expiry information would go here.
}

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
