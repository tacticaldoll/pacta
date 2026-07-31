//! Pacta: a thin, durable lifecycle contract you compose.
//!
//! This crate is the curated public entrypoint. It re-exports the compose-level
//! API of the Pacta workspace so a consumer can depend on one crate:
//!
//! - the lifecycle contract — [`Pact`], [`Claim`], [`Retainer`], [`Timestamp`],
//!   [`Outcome`], [`Settlement`], and the [`Registry`] trait;
//! - the backend-author surface — the [`Transition`] port, the colorless
//!   [`lifecycle`] kernel ([`State`](lifecycle::State), the `on_X` transition
//!   decisions, [`is_claimable`](lifecycle::is_claimable), and the lease arithmetic),
//!   and the [`Uuid`] identifier type the constructors require (to build a [`Pact`] and
//!   mint a fresh [`Retainer`]), so a legal `Registry` is implementable from `pacta` alone;
//! - execution composition — [`Executor`], [`Execution`], [`Middleware`], and the
//!   composition mechanism [`Identity`], [`Stack`], and [`Composition`];
//! - the runtime driver — [`Driver`], [`Step`], [`DriverError`].
//!
//! It carries no logic of its own: every item here is a re-export.
//!
//! # The contract
//!
//! Pacta *is* a lifecycle contract; everything else you compose. The contract has
//! two halves.
//!
//! **What a backend must do (the implementer half).** A [`Registry`] provides the
//! `claim` → `heartbeat` → `fulfill` / `breach` lifecycle over a bounded lease, plus a
//! non-terminal `release` that makes a pact reclaimable again only at or after a
//! consumer-supplied instant: it leases a [`Claim`], reclaims a lapsed lease through the
//! normal claim path with a rotated [`Retainer`], rejects a heartbeat presented after
//! expiry, and honors a reclaimable instant exactly as it honors injected time. Those five
//! caller ops rest on three native primitives a backend actually implements: an atomic
//! `claim` selection that admits only an eligible pact and rotates the retainer, a
//! `lease_millis` accessor, and an atomic [`apply`](Registry::apply) transition port that
//! loads the held state, runs the shared [`lifecycle`] decision, and stores the next state
//! in one indivisible step — `heartbeat`, `fulfill`, `breach`, and `release` come as
//! defaults over `apply`. This half is not merely described — it is *executably proven*: a
//! backend runs the `pacta-conformance` suite (a dev-dependency) and passing it is what it
//! means to satisfy the contract — the backend author's two-crate journey is implement
//! `Registry` from `pacta`, then prove it with `pacta-conformance`. Durable backends
//! live outside this workspace and prove themselves the same way.
//!
//! **What you owe in return (the obligation half).** Recovery is *at-least-once*,
//! not exactly-once: a pact whose holder stops without settling is reclaimed and may
//! run again, so your [`Executor`] **must be idempotent**. Lease *duration* is your
//! input (size it against your work); heartbeat *cadence* is the runtime's — how
//! often a live holder extends its lease is decided by the loop that drives it, not
//! by the core.
//!
//! # Reference pieces, named as such
//!
//! [`Driver`] and the `pacta-memory` backend are *reference* implementations, not
//! production components. `pacta-memory` is in-memory (not durable). [`Driver`] drives
//! synchronously and does not heartbeat a claim in flight, so it is safe for tasks
//! shorter than the lease and for single-worker use; a long-running or multi-worker
//! durable workload composes its own loop over the [`Registry`] contract. See
//! [`Driver`]'s own documentation for the boundary in full.
//!
//! # Stability tiers
//!
//! This facade and the backend-author path are the **recommended** tier — the faces
//! converging toward Pacta's long-term contract. The colorless [`lifecycle`] kernel is
//! part of that recommended backend-author surface and is re-exported here. The sans-I/O
//! *step-driver* kernel (`pacta_contract::kernel`) is the **advanced** tier: lower
//! stability intent (its API may evolve as the runtime story settles), though still a
//! supported, governed core surface — not unsupported or slated for removal. Only the
//! step-driver kernel is intentionally absent from this curated surface; reach for it
//! through [`pacta-contract`](pacta_contract) directly only to build a custom runtime.
//! Most consumers compose with [`Driver`] instead. (Tiers state *intent*; through the
//! 0.2.x patch line SemVer still holds the compose-level and backend-author faces
//! additively stable — a patch adds no breaking change.)
//!
//! # Composing the lifecycle
//!
//! One mechanical step — claim, execute through a pass-through [`Middleware`], and
//! settle — wired entirely through this entrypoint over a *legal, stateful* backend that
//! holds real [`lifecycle`] state and applies the transition atomically (nothing here is
//! imported from outside `pacta`; run `cargo test` to see it execute):
//!
//! ```
//! use std::sync::Mutex;
//! use pacta::lifecycle::{self, State};
//! use pacta::{
//!     Claim, Composition, Driver, Execution, Executor, Identity, Middleware, Outcome, Pact,
//!     Registry, Retainer, Step, Timestamp, Transition, Uuid,
//! };
//!
//! // A complete legal single-pact `Registry`, implemented with only the facade surface: it stores
//! // real lifecycle state, mints a *distinct* retainer per claim (so authority rotates on reclaim),
//! // and applies the transition within one atomic (`Mutex`) scope.
//! struct Ledger { lease_millis: u64, record: Mutex<(Pact, State, u128)> }
//!
//! fn new_ledger() -> Ledger {
//!     let pact = Pact::new(Uuid::from_u128(1), "default".into(), "demo".into(), Vec::new());
//!     Ledger { lease_millis: 30_000, record: Mutex::new((pact, State::Available, 0)) }
//! }
//!
//! #[derive(Debug)]
//! struct NotHeld;
//! impl std::fmt::Display for NotHeld {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("not held") }
//! }
//! impl std::error::Error for NotHeld {}
//! impl From<lifecycle::NotCurrentHolder> for NotHeld {
//!     fn from(_: lifecycle::NotCurrentHolder) -> Self { NotHeld }
//! }
//!
//! impl Registry for Ledger {
//!     type Error = NotHeld;
//!     fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, NotHeld> {
//!         let mut record = self.record.lock().unwrap();
//!         if !dockets.contains(&record.0.docket.as_str()) || !lifecycle::is_claimable(&record.1, now) {
//!             return Ok(None);
//!         }
//!         let retainer = Retainer::new(Uuid::from_u128(record.2)); // a fresh, distinct token per claim
//!         record.2 += 1;
//!         record.1 = lifecycle::on_claim(&retainer, now, self.lease_millis);
//!         Ok(Some(Claim::new(record.0.clone(), retainer, lifecycle::lease_expiry(now, self.lease_millis))))
//!     }
//!     fn lease_millis(&self) -> u64 { self.lease_millis }
//!     // The one transition port: load the held state, run the shared decision, store it — atomically.
//!     // heartbeat / fulfill / breach / release come free as defaults over this.
//!     fn apply(&self, retainer: &Retainer, transition: &Transition<'_>) -> Result<(), NotHeld> {
//!         let mut record = self.record.lock().unwrap();
//!         match &record.1 {
//!             State::Held { retainer: held, .. } if held == retainer => {
//!                 record.1 = transition(&record.1)?;
//!                 Ok(())
//!             }
//!             _ => Err(NotHeld),
//!         }
//!     }
//! }
//!
//! struct Performer;
//! impl Executor for Performer {
//!     type Error = std::convert::Infallible;
//!     fn execute(&mut self, _e: Execution) -> Result<Outcome, Self::Error> { Ok(Outcome::Fulfilled) }
//! }
//!
//! // Compose and settle: claim → execute through a pass-through middleware → settle.
//! let performer = Composition::new().then(Identity).wrap(Performer); // compose, then wrap
//! let mut driver = Driver::new(new_ledger(), performer, ["default".to_string()]);
//! assert_eq!(driver.step().unwrap(), Step::Fulfilled);
//! // The transition was really applied and persisted: the settled pact is no longer claimable.
//! assert!(driver.registry().claim(&["default"], Timestamp::from_millis(0)).unwrap().is_none());
//!
//! // A complete legal backend also rotates authority on reclaim: claim, let the lease lapse, then
//! // reclaim at a later injected time — the new retainer differs, so the stale holder cannot settle.
//! let ledger = new_ledger();
//! let first = ledger.claim(&["default"], Timestamp::from_millis(0)).unwrap().unwrap();
//! let second = ledger.claim(&["default"], Timestamp::from_millis(1_000_000)).unwrap().unwrap();
//! assert_ne!(first.retainer.id(), second.retainer.id());
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use pacta_contract::{
    Claim, Outcome, Pact, Registry, Retainer, Settlement, Timestamp, Transition, Uuid, lifecycle,
};
pub use pacta_driver::{Driver, DriverError, Step};
pub use pacta_executor::{Composition, Execution, Executor, Identity, Middleware, Stack};
