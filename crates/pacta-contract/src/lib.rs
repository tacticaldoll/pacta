//! The pure, zero-dependency contract for Pacta.
//!
//! Pacta operates on three axioms:
//! 1. Store is Lifecycle (no business logic, no retry/delay logic).
//! 2. Execution is Middleware.
//! 3. This contract is zero-dependency.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A durable command or contract, generated from a Signal, ready to be executed.
/// Note the deliberate absence of `attempts`, `delay`, and `priority`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pact {
    pub id: Uuid,
    pub lane: String,
    pub kind: String,
    pub payload: Vec<u8>,
}

/// An opaque token proving authority to resolve a specific reservation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationReceipt(pub Uuid);

/// A reserved job and the receipt required to resolve it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub pact: Pact,
    pub receipt: ReservationReceipt,
    // Lease information would go here
}

/// The Store manages the lifecycle of Pacts. It is a pure state machine.
pub trait Store: Send + Sync {
    type Error;

    /// Lock a pact for execution.
    fn reserve(&self, lanes: &[&str]) -> Result<Option<Reservation>, Self::Error>;

    /// Extend the lease of an ongoing Reservation.
    fn heartbeat(&self, receipt: &ReservationReceipt) -> Result<(), Self::Error>;

    /// Mark the pact as successfully completed.
    fn ack(&self, receipt: &ReservationReceipt) -> Result<(), Self::Error>;

    /// Mark the pact as permanently failed (Dead).
    fn nack(&self, receipt: &ReservationReceipt) -> Result<(), Self::Error>;
}
