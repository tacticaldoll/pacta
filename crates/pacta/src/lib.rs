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
//! # What is deliberately not here
//!
//! The sans-I/O lifecycle kernel (`pacta_contract::kernel`) is advanced machinery
//! and is intentionally absent from this curated surface. Reach for it through
//! [`pacta-contract`](pacta_contract) directly if you are building a custom runtime;
//! most consumers compose with [`Driver`] instead.
//!
//! Durable registry backends live outside this workspace and prove themselves
//! against `pacta-conformance`; the in-memory reference backend is `pacta-memory`.
//!
//! # Composing the lifecycle
//!
//! See `examples/compose.rs` for an end-to-end claim → execute → settle step wired
//! entirely through this entrypoint.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use pacta_contract::{Claim, Outcome, Pact, Registry, Retainer, Settlement, Timestamp};
pub use pacta_driver::{Driver, DriverError, Step};
pub use pacta_executor::{Execution, Executor, Middleware, Policy};
