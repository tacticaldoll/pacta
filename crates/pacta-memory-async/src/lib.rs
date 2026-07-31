//! An in-memory [`AsyncRegistry`] backend with real lease and lapse semantics.
//!
//! The asynchronous counterpart of `pacta-memory`, and a **reference** backend, not a durable
//! one: it holds pacts in memory, so nothing survives the process. It exists to demonstrate the
//! lifecycle semantics through the async binding and to calibrate real async durable backends
//! against, which prove themselves the same way.
//!
//! It implements only the two [`AsyncRegistry`] primitives — a native [`claim`] selection and
//! the [`apply`] transition port — plus [`lease_millis`]; the four transition operations come from
//! the trait's default methods, which compose over the shared [`pacta_contract::lifecycle`]
//! kernel. Its atomic scope is one `Mutex` hold (load, decide, and store without releasing the
//! lock), so its `async fn`s are ready futures — but it exercises the exact same async surface a
//! durable backend implements.
//!
//! [`claim`]: AsyncRegistry::claim
//! [`apply`]: AsyncRegistry::apply
//! [`lease_millis`]: AsyncRegistry::lease_millis

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Mutex;

use async_trait::async_trait;
use pacta_contract::lifecycle::{self, State};
use pacta_contract::{Claim, Pact, Retainer, Timestamp};
use pacta_contract_async::{AsyncRegistry, Transition};
use uuid::Uuid;

/// The error returned when a retainer is not the current holder of any claim (a stale, settled,
/// or lapsed-and-reclaimed retainer, or a heartbeat after the lease already lapsed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotHeld;

impl std::fmt::Display for NotHeld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "retainer is not the current holder of any claim")
    }
}

impl std::error::Error for NotHeld {}

impl From<lifecycle::NotCurrentHolder> for NotHeld {
    fn from(_: lifecycle::NotCurrentHolder) -> Self {
        NotHeld
    }
}

/// An in-memory async registry seeded with a fixed set of pacts.
pub struct MemoryRegistryAsync {
    records: Mutex<Vec<(Pact, State)>>,
    lease_millis: u64,
}

impl MemoryRegistryAsync {
    /// Create an empty registry that leases claims for `lease_millis`.
    #[must_use]
    pub fn new(lease_millis: u64) -> Self {
        Self::seeded(Vec::new(), lease_millis)
    }

    /// Create a registry holding `pacts`, each available to claim, leasing claims for
    /// `lease_millis`.
    #[must_use]
    pub fn seeded(pacts: Vec<Pact>, lease_millis: u64) -> Self {
        Self {
            records: Mutex::new(pacts.into_iter().map(|p| (p, State::Available)).collect()),
            lease_millis,
        }
    }
}

#[async_trait]
impl AsyncRegistry for MemoryRegistryAsync {
    type Error = NotHeld;

    async fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, NotHeld> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        let index = records.iter().position(|(pact, state)| {
            dockets.contains(&pact.docket.as_str()) && lifecycle::is_claimable(state, now)
        });
        let Some(index) = index else { return Ok(None) };
        let retainer = Retainer::new(Uuid::new_v4());
        records[index].1 = lifecycle::on_claim(&retainer, now, self.lease_millis);
        let expiry = lifecycle::lease_expiry(now, self.lease_millis);
        Ok(Some(Claim::new(records[index].0.clone(), retainer, expiry)))
    }

    fn lease_millis(&self) -> u64 {
        self.lease_millis
    }

    async fn apply(
        &self,
        _retainer: &Retainer,
        transition: &Transition<'_>,
    ) -> Result<(), NotHeld> {
        // One `Mutex` hold is the atomic scope: load, decide, and store without releasing the
        // lock, so there is no load-then-store window to race. The transition carries the
        // authority check, so the first record it accepts is the pact the retainer holds; a
        // durable backend would instead load by `retainer`, and this in-memory scan is equivalent.
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        for (_, state) in records.iter_mut() {
            if let Ok(next) = transition(state) {
                *state = next;
                return Ok(());
            }
        }
        Err(NotHeld)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    const LEASE: u64 = 1000;

    fn at(ms: u64) -> Timestamp {
        Timestamp::from_millis(ms)
    }

    fn a_pact() -> Pact {
        Pact::new(Uuid::new_v4(), "d".to_string(), "k".to_string(), Vec::new())
    }

    /// The reference async backend is held to the same scenarios as every sync backend, through the
    /// shared conformance suite — the async binding proving itself, not a bespoke test set. This
    /// runs synchronously because the runner drives the ready futures to completion; it proves
    /// state-machine parity, not concurrency (see below).
    #[test]
    fn passes_async_conformance() {
        pacta_conformance::run_async(|pacts, lease_millis| {
            MemoryRegistryAsync::seeded(pacts, lease_millis)
        });
    }

    /// The one property the parity runner cannot reach: the at-most-once invariant every backend's
    /// `apply` must uphold under concurrent contention, regardless of the concurrency-control
    /// mechanism it uses (here, one `Mutex` scope). Asserted through the public `fulfill` op — not
    /// by inspecting the mechanism — so it holds for any backend. Needs genuine multi-threaded
    /// parallelism, because the reference backend's ready futures do not interleave single-threaded.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_settle_applies_at_most_once() {
        // Enough iterations that an interleaving loading the same state in both tasks is hit, and
        // that a broken (non-atomic) fence would overwhelmingly double-apply on some iteration.
        for _ in 0..2000 {
            let reg = Arc::new(MemoryRegistryAsync::seeded(vec![a_pact()], LEASE));
            let retainer = reg
                .claim(&["d"], at(0))
                .await
                .unwrap()
                .expect("claimable")
                .retainer;

            let a = {
                let reg = Arc::clone(&reg);
                let retainer = retainer.clone();
                tokio::spawn(async move { reg.fulfill(&retainer).await })
            };
            let b = {
                let reg = Arc::clone(&reg);
                let retainer = retainer.clone();
                tokio::spawn(async move { reg.fulfill(&retainer).await })
            };
            let (a, b) = (a.await.unwrap(), b.await.unwrap());

            let winners = [a.is_ok(), b.is_ok()].into_iter().filter(|&ok| ok).count();
            assert_eq!(
                winners, 1,
                "settlement must apply exactly once: a={a:?} b={b:?}"
            );
            // The winner settled the pact; a stranger's authority is gone and it is not claimable.
            assert!(reg.claim(&["d"], at(0)).await.unwrap().is_none());
        }
    }
}
