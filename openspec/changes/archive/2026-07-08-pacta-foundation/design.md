# Design: Pacta Foundation

## Overview

This document outlines the initial design of the `pacta-contract` and `pacta-governance` crates, establishing the fundamental boundaries of the system.

## 1. The Contract (`pacta-contract`)

The `pacta-contract` crate is the sole source of truth. It defines what a `Pact` is and how the `Store` transitions it.

### The `Pact` Envelope
Unlike traditional job queues, the `Pact` drops execution policies:
```rust
pub struct Pact {
    pub id: Uuid,
    pub lane: String,
    pub kind: String,
    pub payload: Vec<u8>,
}
```
*Rejected alternatives*: We rejected including `attempts`, `delay`, `trace_context`, or `priority` because they couple the storage layer to business execution rules and scheduling.

### The `Store` Trait
A pure state machine interface:
```rust
pub trait Store {
    /// Lock a pact for execution, returning a Reservation with a Lease.
    fn reserve(&self, lanes: &[&str]) -> Result<Option<Reservation>, StoreError>;
    
    /// Extend the lease of an ongoing Reservation.
    fn heartbeat(&self, lease_id: &Uuid) -> Result<(), StoreError>;
    
    /// Mark the pact as successfully completed.
    fn ack(&self, lease_id: &Uuid) -> Result<(), StoreError>;
    
    /// Mark the pact as permanently failed (Dead).
    fn nack(&self, lease_id: &Uuid) -> Result<(), StoreError>;
}
```

## 2. Middleware & Execution (Conceptual Design for Phase 2)

While the `Handler` trait will be implemented in a future change (`pacta-driver`), the design anticipates a `&self` based trait to enable frictionless concurrency, avoiding `tower::Service`'s `&mut self` and `poll_ready` for background tasks:

```rust
// Anticipated design for future Driver integration
pub trait Handler: Send + Sync {
    fn handle(&self, pact: Pact) -> impl Future<Output = Outcome> + Send;
}
```
This enables the "Replay Ingress" capability without needing complex Buffers or service cloning.

## 3. Executable Governance (`pacta-governance`)

To ensure the "Zero-Dependency Contract" axiom is maintained forever, we configure `tianheng` to violently reject any dependency drift in `pacta-contract`. 

The `tianheng` ruleset will enforce:
- `pacta-contract` may only depend on standard or explicitly allowed foundational crates (e.g., `serde`, `uuid`).
- No future `pacta-store-*` crate can leak into the contract.
