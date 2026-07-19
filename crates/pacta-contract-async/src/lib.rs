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
//!
//! # Both halves of the contract
//!
//! This binding does not re-specify lifecycle behavior — it references the governed truth in
//! [`pacta_contract::lifecycle`] and `pacta-conformance`. But an implementer and a consumer each
//! owe the contract obligations, made explicit here so reading only this crate shows both halves.
//!
//! ## What a backend must satisfy (implementer half)
//!
//! - **`cas` must be atomic.** It must compare-and-set the pact's state in one indivisible step
//!   (e.g. a conditional `UPDATE`, or a transaction). If `cas` is not atomic, two workers can
//!   both observe `expected` and both write — and exactly-once and retainer fencing break
//!   silently. This is the load-bearing obligation of the transition port.
//! - **`claim` must honor the eligibility invariant and rotate the retainer.** It must select
//!   only a pact [`lifecycle::is_claimable`] would admit (available, a lapsed hold, or a
//!   deferred pact past its instant — never a settled one) and mint a fresh retainer, all
//!   atomically. It must be a **native, full-scan-free** selection (e.g. SQL `SKIP LOCKED`); it
//!   must not load the whole docket to filter in memory. Eligibility is re-expressed natively
//!   per backend, so `pacta-conformance` is the executable proof it matches the invariant.
//! - **`load` returns the state of the pact the retainer currently holds**, or `None`.
//!
//! ## What a consumer owes (user-obligation half)
//!
//! - **An idempotent unit of work.** Recovery is **at-least-once**, not exactly-once: a lapsed,
//!   reclaimed pact is executed again. The work a consumer performs between claim and settle
//!   must be safe to repeat (compose idempotency with `shaahid`, or make the effect naturally
//!   idempotent).
//! - **User-owned lease sizing.** The lease duration ([`lease_millis`](AsyncRegistry::lease_millis))
//!   is the consumer's to size for its workload; the contract supplies the mechanism, not a
//!   constant.
//! - **Runtime-owned heartbeat cadence.** Long work must [`heartbeat`](AsyncRegistry::heartbeat)
//!   before its lease lapses; when and how often is the runtime's policy.
//!
//! Note the fence rule this binding inherits from the frozen contract: a holder whose lease has
//! lapsed but whose pact **no one has reclaimed** can still settle (its retainer is still the
//! current holder) — reclaim, not expiry, rotates authority.

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
    /// selection primitive: a backend performs it natively (a full-scan-free selection, e.g. SQL
    /// `SKIP LOCKED`), because selection cannot be built from `load`/`cas`.
    ///
    /// **Obligation:** select only a pact [`lifecycle::is_claimable`] would admit, and mint a
    /// fresh retainer, atomically. Eligibility is re-expressed natively per backend, so
    /// `pacta-conformance` is the executable proof it matches the invariant.
    async fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error>;

    /// Load the current lifecycle [`State`] of the pact held by `retainer`, or `None` if the
    /// retainer holds no pact.
    async fn load(&self, retainer: &Retainer) -> Result<Option<State>, Self::Error>;

    /// Atomically set the state of the pact held by `retainer` to `next` iff its current state
    /// still equals `expected`; return whether it applied. A `false` means the state changed
    /// under the caller (contention, or a lapse and reclaim).
    ///
    /// **Obligation:** this must be a single atomic compare-and-set (a conditional `UPDATE` or a
    /// transaction). A non-atomic implementation — read-then-write with a gap — lets two workers
    /// both observe `expected` and both write, silently breaking exactly-once and retainer
    /// fencing. This is the load-bearing obligation of the transition port.
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
