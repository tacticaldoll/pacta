//! Pacta: a thin, durable lifecycle contract you compose.
//!
//! This crate is the curated public entrypoint. It re-exports the compose-level
//! API of the Pacta workspace so a consumer can depend on one crate:
//!
//! - the lifecycle contract — [`Pact`], [`Claim`], [`Retainer`], [`Timestamp`],
//!   [`Outcome`], [`Settlement`], and the [`Registry`] trait;
//! - execution composition — [`Executor`], [`Execution`], [`Middleware`], [`Policy`];
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
//! `claim` → `heartbeat` → `fulfill` / `breach` lifecycle over a bounded lease: it
//! leases a [`Claim`], reclaims a lapsed lease through the normal claim path with a
//! rotated [`Retainer`], and rejects a heartbeat presented after expiry. This half
//! is not merely described — it is *executably proven*: a backend runs the
//! `pacta-conformance` suite (a dev-dependency) and passing it is what it means to
//! satisfy the contract — the backend author's two-crate journey is implement
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
//! converging toward Pacta's long-term contract. The sans-I/O lifecycle kernel
//! (`pacta_contract::kernel`) is the **advanced** tier: lower stability intent (its
//! API may evolve as the runtime story settles), though still a supported, governed
//! core surface — not unsupported or slated for removal. It is intentionally absent
//! from this curated surface; reach for it through [`pacta-contract`](pacta_contract)
//! directly only to build a custom runtime. Most consumers compose with [`Driver`]
//! instead. (Tiers state *intent*; at 0.1.x SemVer holds every face unstable.)
//!
//! # Composing the lifecycle
//!
//! One mechanical step — claim, execute through a pass-through [`Middleware`], and
//! settle — wired entirely through this entrypoint (backend and executor boilerplate
//! hidden; run `cargo test` to see it execute):
//!
//! ```
//! # use std::convert::Infallible;
//! # use std::sync::Mutex;
//! # use pacta::{Claim, Execution, Executor, Middleware, Outcome, Pact, Registry, Retainer, Timestamp};
//! # struct Ledger { pending: Mutex<Option<Claim>> }
//! # impl Registry for Ledger {
//! #     type Error = Infallible;
//! #     fn claim(&self, _d: &[&str], _n: Timestamp) -> Result<Option<Claim>, Infallible> {
//! #         Ok(self.pending.lock().unwrap().take())
//! #     }
//! #     fn heartbeat(&self, _r: &Retainer, _n: Timestamp) -> Result<(), Infallible> { Ok(()) }
//! #     fn fulfill(&self, _r: &Retainer) -> Result<(), Infallible> { Ok(()) }
//! #     fn breach(&self, _r: &Retainer) -> Result<(), Infallible> { Ok(()) }
//! # }
//! # struct Performer;
//! # impl Executor for Performer {
//! #     type Error = Infallible;
//! #     fn execute(&mut self, _e: Execution) -> Result<Outcome, Infallible> { Ok(Outcome::Fulfilled) }
//! # }
//! # struct Witnessed<E> { inner: E }
//! # impl<E: Executor> Executor for Witnessed<E> {
//! #     type Error = E::Error;
//! #     fn execute(&mut self, e: Execution) -> Result<Outcome, Self::Error> { self.inner.execute(e) }
//! # }
//! # struct Witness;
//! # impl<E: Executor> Middleware<E> for Witness {
//! #     type Executor = Witnessed<E>;
//! #     fn wrap(&self, inner: E) -> Witnessed<E> { Witnessed { inner } }
//! # }
//! # let claim = Claim {
//! #     pact: Pact { id: Default::default(), docket: "default".into(), kind: "demo".into(), clause: Vec::new() },
//! #     retainer: Retainer::new(Default::default()),
//! #     lease_expiry: Timestamp::from_millis(0),
//! # };
//! use pacta::{Driver, Step};
//!
//! let ledger = Ledger { pending: Mutex::new(Some(claim)) };
//! let performer = Witness.wrap(Performer);          // middleware wraps the executor
//! let mut driver = Driver::new(ledger, performer, ["default".to_string()]);
//!
//! assert_eq!(driver.step().unwrap(), Step::Fulfilled); // claim → execute → settle
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use pacta_contract::{Claim, Outcome, Pact, Registry, Retainer, Settlement, Timestamp};
pub use pacta_driver::{Driver, DriverError, Step};
pub use pacta_executor::{Execution, Executor, Middleware, Policy};
