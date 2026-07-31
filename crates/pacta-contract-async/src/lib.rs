//! An asynchronous binding of the Pacta `Registry` contract.
//!
//! Real durable backends do async I/O and cannot implement the synchronous
//! [`pacta_contract::Registry`]. [`AsyncRegistry`] is the same frozen five-op contract, made
//! asynchronous — a *second binding*, not new semantics. The lifecycle semantics are
//! single-sourced in [`pacta_contract::lifecycle`], which this binding composes over, so the
//! sync and async bindings cannot drift.
//!
//! A backend implements only three primitives — [`claim`](AsyncRegistry::claim) (a native
//! selection carrying the eligibility invariant), and the transition port
//! [`load`](AsyncRegistry::load) + [`cas`](AsyncRegistry::cas) — plus a
//! [`lease_millis`](AsyncRegistry::lease_millis) accessor. The four transition operations
//! (heartbeat, fulfill, breach, release) are default methods that `load -> lifecycle::on_X ->
//! cas`, so their semantics (including heartbeat's lapsed check) come from the shared kernel.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use async_trait::async_trait;
use pacta_contract::lifecycle::{self, State};
use pacta_contract::{Claim, Retainer, Timestamp};

/// An asynchronous [`pacta_contract::Registry`]: the same frozen five-op contract, made async.
///
/// Backends implement the three primitives ([`claim`](Self::claim), [`load`](Self::load),
/// [`cas`](Self::cas)) and the [`lease_millis`](Self::lease_millis) accessor; the four transition
/// operations are provided as default methods composing over [`pacta_contract::lifecycle`].
#[async_trait]
pub trait AsyncRegistry: Send + Sync {
    /// Error returned by the backend. It must be able to represent a lost/absent authority, so
    /// the shared kernel's [`lifecycle::NotCurrentHolder`] converts into it.
    type Error: std::error::Error + Send + Sync + 'static + From<lifecycle::NotCurrentHolder>;

    // --- required primitives ---

    /// Claim an eligible pact from one of `dockets` at `now`, rotating the retainer. This is the
    /// selection primitive: a backend performs it natively (a full-scan-free selection that
    /// re-expresses the eligibility invariant, e.g. SQL `SKIP LOCKED`), because selection cannot
    /// be built from `load`/`cas`.
    async fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error>;

    /// Load the current lifecycle [`State`] of the pact held by `retainer`, or `None` if the
    /// retainer holds no pact.
    async fn load(&self, retainer: &Retainer) -> Result<Option<State>, Self::Error>;

    /// Atomically set the state of the pact held by `retainer` to `next` iff its current state
    /// still equals `expected`; return whether it applied. A `false` means the state changed
    /// under the caller (contention, or a lapse and reclaim).
    async fn cas(
        &self,
        retainer: &Retainer,
        expected: &State,
        next: &State,
    ) -> Result<bool, Self::Error>;

    /// The backend's lease duration in milliseconds — used by the default
    /// [`heartbeat`](Self::heartbeat) to refresh the lease while keeping the faithful
    /// `heartbeat(retainer, now)` signature (the sync contract takes no lease parameter).
    fn lease_millis(&self) -> u64;

    // --- default transition methods: load -> lifecycle::on_X -> cas (single-sourced) ---

    /// Extend the lease of the pact held by `retainer` to `now + lease`, provided the retainer
    /// currently holds it and the lease has not lapsed. Composes over
    /// [`lifecycle::on_heartbeat`].
    async fn heartbeat(&self, retainer: &Retainer, now: Timestamp) -> Result<(), Self::Error> {
        let lease = self.lease_millis();
        loop {
            let state = self
                .load(retainer)
                .await?
                .ok_or(lifecycle::NotCurrentHolder)?;
            let next = lifecycle::on_heartbeat(&state, retainer, now, lease)?;
            if self.cas(retainer, &state, &next).await? {
                return Ok(());
            }
        }
    }

    /// Conclude the pact held by `retainer` (fulfilled). Composes over [`lifecycle::on_settle`].
    async fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.settle(retainer).await
    }

    /// Conclude the pact held by `retainer` (breached). Shares the settlement transition with
    /// [`fulfill`](Self::fulfill) — the lifecycle state records that the obligation concluded,
    /// not which outcome concluded it.
    async fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.settle(retainer).await
    }

    /// Relinquish the claim held by `retainer` non-terminally, making the pact reclaimable at or
    /// after `reclaimable_at`. Composes over [`lifecycle::on_release`].
    async fn release(
        &self,
        retainer: &Retainer,
        reclaimable_at: Timestamp,
    ) -> Result<(), Self::Error> {
        loop {
            let state = self
                .load(retainer)
                .await?
                .ok_or(lifecycle::NotCurrentHolder)?;
            let next = lifecycle::on_release(&state, retainer, reclaimable_at)?;
            if self.cas(retainer, &state, &next).await? {
                return Ok(());
            }
        }
    }

    /// The shared settlement transition for [`fulfill`](Self::fulfill) and
    /// [`breach`](Self::breach).
    async fn settle(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        loop {
            let state = self
                .load(retainer)
                .await?
                .ok_or(lifecycle::NotCurrentHolder)?;
            let next = lifecycle::on_settle(&state, retainer)?;
            if self.cas(retainer, &state, &next).await? {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pacta_contract::Pact;
    use std::sync::Mutex;
    use uuid::Uuid;

    /// A trivial in-memory async backend implementing only the three primitives, to prove the
    /// five ops emerge through the defaults. Not a reference backend — that is a later change.
    struct MemAsync {
        records: Mutex<Vec<(Pact, State)>>,
        lease_millis: u64,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct NotHeld;

    impl std::fmt::Display for NotHeld {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "not the current holder")
        }
    }
    impl std::error::Error for NotHeld {}
    impl From<lifecycle::NotCurrentHolder> for NotHeld {
        fn from(_: lifecycle::NotCurrentHolder) -> Self {
            NotHeld
        }
    }

    impl MemAsync {
        fn seeded(pacts: Vec<Pact>, lease_millis: u64) -> Self {
            Self {
                records: Mutex::new(pacts.into_iter().map(|p| (p, State::Available)).collect()),
                lease_millis,
            }
        }
    }

    #[async_trait]
    impl AsyncRegistry for MemAsync {
        type Error = NotHeld;

        async fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, NotHeld> {
            let mut records = self.records.lock().unwrap();
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
            let records = self.records.lock().unwrap();
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
            let mut records = self.records.lock().unwrap();
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

    fn a_pact() -> Pact {
        Pact::new(Uuid::new_v4(), "d".to_string(), "k".to_string(), Vec::new())
    }

    #[tokio::test]
    async fn claim_then_fulfill_round_trips_via_defaults() {
        let reg = MemAsync::seeded(vec![a_pact()], 1000);
        let claim = reg
            .claim(&["d"], Timestamp::from_millis(0))
            .await
            .unwrap()
            .expect("a pact is claimable");
        // fulfill is a default method built on load + cas.
        reg.fulfill(&claim.retainer).await.expect("fulfill settles");
        // Settled: not claimable again.
        assert!(
            reg.claim(&["d"], Timestamp::from_millis(0))
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn heartbeat_refreshes_and_a_stranger_is_rejected() {
        let reg = MemAsync::seeded(vec![a_pact()], 1000);
        let claim = reg
            .claim(&["d"], Timestamp::from_millis(0))
            .await
            .unwrap()
            .unwrap();
        reg.heartbeat(&claim.retainer, Timestamp::from_millis(500))
            .await
            .expect("live lease refreshes");
        let stranger = Retainer::new(Uuid::new_v4());
        assert_eq!(
            reg.heartbeat(&stranger, Timestamp::from_millis(600)).await,
            Err(NotHeld)
        );
    }

    #[tokio::test]
    async fn release_then_reclaim_and_the_prior_holder_is_rejected() {
        let reg = MemAsync::seeded(vec![a_pact()], 1000);
        let first = reg
            .claim(&["d"], Timestamp::from_millis(0))
            .await
            .unwrap()
            .unwrap();
        // Release with an immediate reclaimable instant, then reclaim.
        reg.release(&first.retainer, Timestamp::from_millis(0))
            .await
            .expect("release");
        let second = reg
            .claim(&["d"], Timestamp::from_millis(10))
            .await
            .unwrap()
            .expect("reclaimable after release");
        // The prior holder can no longer settle — authority rotated (load finds it no longer held).
        assert_eq!(reg.fulfill(&first.retainer).await, Err(NotHeld));
        reg.fulfill(&second.retainer)
            .await
            .expect("new holder settles");
    }
}
