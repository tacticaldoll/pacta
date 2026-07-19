//! An in-memory [`AsyncRegistry`] backend with real lease and lapse semantics.
//!
//! The asynchronous counterpart of `pacta-memory`, and a **reference** backend, not a durable
//! one: it holds pacts in memory, so nothing survives the process. It exists to demonstrate the
//! lifecycle semantics through the async binding and to calibrate real async durable backends
//! against, which prove themselves the same way.
//!
//! It implements only the three [`AsyncRegistry`] primitives — a native [`claim`] selection and
//! the [`load`] + [`cas`] transition port — plus [`lease_millis`]; the four transition
//! operations come from the trait's default methods, which compose over the shared
//! [`pacta_contract::lifecycle`] kernel. Its I/O is trivial (a `Mutex`), so its `async fn`s are
//! ready futures — but it exercises the exact same async surface a durable backend implements.
//!
//! [`claim`]: AsyncRegistry::claim
//! [`load`]: AsyncRegistry::load
//! [`cas`]: AsyncRegistry::cas
//! [`lease_millis`]: AsyncRegistry::lease_millis

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Mutex;

use async_trait::async_trait;
use pacta_contract::lifecycle::{self, State};
use pacta_contract::{Claim, Pact, Retainer, Timestamp};
use pacta_contract_async::AsyncRegistry;
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

    async fn load(&self, retainer: &Retainer) -> Result<Option<State>, NotHeld> {
        let records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        Ok(records
            .iter()
            .find(|(_, state)| {
                matches!(state, State::Held { retainer: held, .. } if held == retainer)
            })
            .map(|(_, state)| state.clone()))
    }

    async fn cas(
        &self,
        _retainer: &Retainer,
        expected: &State,
        next: &State,
    ) -> Result<bool, NotHeld> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        // `expected` uniquely identifies the record (its retainer is part of the state), so a
        // global state-equality match is the atomic compare-and-set the transition needs.
        if let Some((_, state)) = records.iter_mut().find(|(_, state)| state == expected) {
            *state = next.clone();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn lease_millis(&self) -> u64 {
        self.lease_millis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEASE: u64 = 1000;

    fn at(ms: u64) -> Timestamp {
        Timestamp::from_millis(ms)
    }

    fn a_pact() -> Pact {
        Pact::new(Uuid::new_v4(), "d".to_string(), "k".to_string(), Vec::new())
    }

    fn seeded() -> MemoryRegistryAsync {
        MemoryRegistryAsync::seeded(vec![a_pact()], LEASE)
    }

    #[tokio::test]
    async fn claim_sets_a_fresh_lease() {
        let reg = seeded();
        let claim = reg
            .claim(&["d"], at(100))
            .await
            .unwrap()
            .expect("claimable");
        assert_eq!(claim.lease_expiry, at(100 + LEASE));
    }

    #[tokio::test]
    async fn held_pact_not_reclaimable_before_expiry() {
        let reg = seeded();
        reg.claim(&["d"], at(0)).await.unwrap().expect("claimable");
        assert!(reg.claim(&["d"], at(500)).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn expired_lease_lapses_and_rotates_retainer() {
        let reg = seeded();
        let first = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        let second = reg
            .claim(&["d"], at(1500))
            .await
            .unwrap()
            .expect("reclaimable after lapse");
        assert_ne!(first.retainer.id(), second.retainer.id());
        assert_eq!(second.lease_expiry, at(1500 + LEASE));
    }

    #[tokio::test]
    async fn fulfill_settles_terminally() {
        let reg = seeded();
        let claim = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        reg.fulfill(&claim.retainer).await.expect("fulfill settles");
        assert!(reg.claim(&["d"], at(0)).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn breach_settles_terminally() {
        let reg = seeded();
        let claim = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        reg.breach(&claim.retainer).await.expect("breach settles");
        assert!(reg.claim(&["d"], at(0)).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn released_pact_withheld_then_reclaimable_at_instant() {
        let reg = seeded();
        let first = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        reg.release(&first.retainer, at(1000))
            .await
            .expect("release");
        // Before the instant: withheld.
        assert!(reg.claim(&["d"], at(999)).await.unwrap().is_none());
        // At the instant: reclaimable, and authority rotated.
        let second = reg
            .claim(&["d"], at(1000))
            .await
            .unwrap()
            .expect("reclaimable");
        assert_ne!(first.retainer.id(), second.retainer.id());
        assert_eq!(reg.fulfill(&first.retainer).await, Err(NotHeld));
    }

    #[tokio::test]
    async fn heartbeat_extends_lease_preventing_lapse() {
        let reg = seeded();
        let claim = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        // Heartbeat before expiry pushes the lease out.
        reg.heartbeat(&claim.retainer, at(900))
            .await
            .expect("live lease refreshes");
        // At 1500 the original lease would have lapsed; the refreshed one (to 1900) has not.
        assert!(reg.claim(&["d"], at(1500)).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn heartbeat_on_lapsed_lease_is_rejected() {
        let reg = seeded();
        let claim = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        assert_eq!(reg.heartbeat(&claim.retainer, at(1500)).await, Err(NotHeld));
    }

    #[tokio::test]
    async fn stale_retainer_after_reclaim_is_rejected() {
        let reg = seeded();
        let first = reg.claim(&["d"], at(0)).await.unwrap().unwrap();
        let _second = reg
            .claim(&["d"], at(1500))
            .await
            .unwrap()
            .expect("reclaimed");
        assert_eq!(reg.fulfill(&first.retainer).await, Err(NotHeld));
    }
}
