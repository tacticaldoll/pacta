//! In-memory reference [`Registry`] backends with real lease and lapse semantics.
//!
//! These are **reference** backends, not durable or production ones: they hold pacts in memory, so
//! nothing survives the process. They exist to demonstrate correct lifecycle semantics and to
//! calibrate against — durable backends live outside this workspace and prove themselves against
//! `pacta-conformance` just as these do.
//!
//! [`MemoryRegistry`] implements the synchronous [`Registry`]. Behind the `async` feature,
//! [`MemoryRegistryAsync`] implements [`pacta_contract::AsyncRegistry`] over the **same** private
//! store, so the two bindings share one storage and cannot drift. Every eligibility decision and
//! state transition is delegated to the shared, pure [`pacta_contract::lifecycle`] kernel; the store
//! reads no clock — time is injected into `claim` and `heartbeat`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Mutex;

use pacta_contract::lifecycle::{self, State};
use pacta_contract::{Claim, Pact, Registry, Retainer, Timestamp, Transition};
use uuid::Uuid;

/// The error a memory backend returns when a retainer is not the current holder,
/// or when a heartbeat arrives after its lease has already lapsed.
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

struct Record {
    pact: Pact,
    state: State,
}

/// The shared in-memory store: storage, retainer minting, and the claim-select / transition-apply
/// logic. Both the sync and async backends wrap one of these, so their behavior is single-sourced.
/// It owns no I/O beyond a `Mutex`; every decision is the shared `lifecycle` kernel's.
struct Store {
    records: Mutex<Vec<Record>>,
    lease_millis: u64,
}

impl Store {
    fn seeded(pacts: Vec<Pact>, lease_millis: u64) -> Self {
        Self {
            records: Mutex::new(
                pacts
                    .into_iter()
                    .map(|pact| Record {
                        pact,
                        state: State::Available,
                    })
                    .collect(),
            ),
            lease_millis,
        }
    }

    fn lease_millis(&self) -> u64 {
        self.lease_millis
    }

    fn claim(&self, dockets: &[&str], now: Timestamp) -> Option<Claim> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        // Storage picks a candidate on the requested dockets; the kernel decides
        // eligibility (available / lapsed hold / reclaimable defer / settled).
        let index = records.iter().position(|record| {
            dockets.contains(&record.pact.docket.as_str())
                && lifecycle::is_claimable(&record.state, now)
        })?;
        // Mint a retainer only on a successful claim; the kernel produces the held state.
        let retainer = Retainer::new(Uuid::new_v4());
        records[index].state = lifecycle::on_claim(&retainer, now, self.lease_millis);
        let expiry = lifecycle::lease_expiry(now, self.lease_millis);
        Some(Claim::new(records[index].pact.clone(), retainer, expiry))
    }

    /// Apply a lifecycle transition to the pact held by `retainer`, within one `Mutex` scope
    /// (load, decide, and store without releasing the lock, so there is no load-then-store race).
    /// This locates the record the retainer holds — the one whose state is `Held { retainer, .. }` —
    /// exactly as a durable backend loads its row by the holder key, then runs the transition on
    /// that record and persists the result. The transition's own `Result` is propagated, so a
    /// transition that rejects the located state (a heartbeat on a lapsed-but-unreclaimed lease)
    /// still fails. A retainer that holds no record resolves to `NotHeld` without mutating anything —
    /// so an authority the caller does not hold cannot drive a transition, even one that would
    /// accept any state.
    fn apply(&self, retainer: &Retainer, transition: &Transition<'_>) -> Result<(), NotHeld> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        let record = records
            .iter_mut()
            .find(|record| matches!(&record.state, State::Held { retainer: held, .. } if held == retainer))
            .ok_or(NotHeld)?;
        record.state = transition(&record.state)?;
        Ok(())
    }
}

/// An in-memory synchronous registry seeded with a fixed set of pacts.
pub struct MemoryRegistry {
    store: Store,
}

impl MemoryRegistry {
    /// Create an empty registry that leases claims for `lease_millis`.
    #[must_use]
    pub fn new(lease_millis: u64) -> Self {
        Self::seeded(Vec::new(), lease_millis)
    }

    /// Create a registry holding `pacts`, each available to claim, leasing claims
    /// for `lease_millis`.
    #[must_use]
    pub fn seeded(pacts: Vec<Pact>, lease_millis: u64) -> Self {
        Self {
            store: Store::seeded(pacts, lease_millis),
        }
    }
}

impl Registry for MemoryRegistry {
    type Error = NotHeld;

    fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error> {
        Ok(self.store.claim(dockets, now))
    }

    fn lease_millis(&self) -> u64 {
        self.store.lease_millis()
    }

    fn apply(&self, retainer: &Retainer, transition: &Transition<'_>) -> Result<(), Self::Error> {
        self.store.apply(retainer, transition)
    }
}

/// An in-memory asynchronous registry seeded with a fixed set of pacts — the reference
/// [`AsyncRegistry`](pacta_contract::AsyncRegistry) backend, over the same private store as
/// [`MemoryRegistry`]. Its I/O is trivial (a `Mutex`), so its `async fn`s are ready futures, but it
/// exercises the exact same async surface a durable backend implements.
#[cfg(feature = "async")]
pub struct MemoryRegistryAsync {
    store: Store,
}

#[cfg(feature = "async")]
impl MemoryRegistryAsync {
    /// Create an empty registry that leases claims for `lease_millis`.
    #[must_use]
    pub fn new(lease_millis: u64) -> Self {
        Self::seeded(Vec::new(), lease_millis)
    }

    /// Create a registry holding `pacts`, each available to claim, leasing claims for `lease_millis`.
    #[must_use]
    pub fn seeded(pacts: Vec<Pact>, lease_millis: u64) -> Self {
        Self {
            store: Store::seeded(pacts, lease_millis),
        }
    }
}

#[cfg(feature = "async")]
impl pacta_contract::AsyncRegistry for MemoryRegistryAsync {
    type Error = NotHeld;

    async fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, NotHeld> {
        Ok(self.store.claim(dockets, now))
    }

    fn lease_millis(&self) -> u64 {
        self.store.lease_millis()
    }

    async fn apply(&self, retainer: &Retainer, transition: &Transition<'_>) -> Result<(), NotHeld> {
        // The store's `apply` is one atomic `Mutex` scope; awaiting nothing, this backend's futures
        // are ready, but it exercises the same async surface a durable backend implements. It
        // locates the record held by `retainer`, as a durable backend loads its row by holder.
        self.store.apply(retainer, transition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_registry_conformance() {
        pacta_conformance::run(MemoryRegistry::seeded);
    }

    /// The sync reference backend upholds at-most-once authority under real concurrent claim and
    /// settlement contention, through the shared portable check.
    #[test]
    fn passes_sync_contention() {
        pacta_conformance::run_contention(MemoryRegistry::seeded);
    }

    fn a_pact() -> Pact {
        Pact::new(Uuid::new_v4(), "d".to_string(), "k".to_string(), Vec::new())
    }

    #[test]
    fn release_rejects_a_non_holder() {
        let registry = MemoryRegistry::seeded(vec![a_pact()], 1000);
        registry
            .claim(&["d"], Timestamp::from_millis(0))
            .expect("claim should not error")
            .expect("a pact should be claimable");
        let stranger = Retainer::new(Uuid::new_v4());
        assert_eq!(
            registry.release(&stranger, Timestamp::from_millis(0)),
            Err(NotHeld),
            "release by a non-holder must be rejected, like fulfill and breach"
        );
    }

    #[test]
    fn a_settled_pact_cannot_be_released() {
        let registry = MemoryRegistry::seeded(vec![a_pact()], 1000);
        let claim = registry
            .claim(&["d"], Timestamp::from_millis(0))
            .expect("claim should not error")
            .expect("a pact should be claimable");
        registry
            .fulfill(&claim.retainer)
            .expect("fulfill should settle");
        assert_eq!(
            registry.release(&claim.retainer, Timestamp::from_millis(0)),
            Err(NotHeld),
            "a concluded obligation has no claim to relinquish"
        );
    }

    /// Adversarial authority: a stranger retainer paired with a transition that would accept *any*
    /// state must not drive a transition, because `apply` locates the record the retainer holds and
    /// the stranger holds none. The held pact is left untouched — the true holder still settles it —
    /// so authority is enforced by the located record, not merely by the transition closure.
    #[test]
    fn apply_rejects_a_stranger_even_with_an_any_state_transition() {
        let registry = MemoryRegistry::seeded(vec![a_pact()], 1000);
        let claim = registry
            .claim(&["d"], Timestamp::from_millis(0))
            .expect("claim should not error")
            .expect("a pact should be claimable");
        let stranger = Retainer::new(Uuid::new_v4());
        // This transition would accept any state — the safety must come from apply locating the
        // stranger's (nonexistent) held record, not from the transition policing the holder.
        let accept_any = |_state: &State| Ok::<State, lifecycle::NotCurrentHolder>(State::Settled);
        assert_eq!(
            registry.apply(&stranger, &accept_any),
            Err(NotHeld),
            "a retainer that holds no record cannot apply, even an any-state transition"
        );
        // The held pact was not mutated: the true holder still settles it.
        registry
            .fulfill(&claim.retainer)
            .expect("the held state was untouched, so the holder still settles");
    }

    /// The correct holder's lifecycle transitions still succeed after locating by retainer:
    /// heartbeat extends, and release then rotates authority away.
    #[test]
    fn apply_admits_the_true_holder() {
        let registry = MemoryRegistry::seeded(vec![a_pact()], 1000);
        let claim = registry
            .claim(&["d"], Timestamp::from_millis(0))
            .expect("claim should not error")
            .expect("a pact should be claimable");
        registry
            .heartbeat(&claim.retainer, Timestamp::from_millis(500))
            .expect("the holder's heartbeat extends the lease");
        registry
            .release(&claim.retainer, Timestamp::from_millis(0))
            .expect("the holder releases");
        assert_eq!(
            registry.fulfill(&claim.retainer),
            Err(NotHeld),
            "release rotated authority, so the prior retainer no longer holds a record"
        );
    }

    /// The reference async backend is held to the same scenarios as every sync backend, through the
    /// shared conformance suite — the async binding proving itself, over the same `Store`.
    #[cfg(feature = "async")]
    #[test]
    fn passes_async_conformance() {
        pacta_conformance::run_async(MemoryRegistryAsync::seeded);
    }

    /// The async binding enforces authority the same way: a stranger retainer with an any-state
    /// transition is rejected, over the same shared `Store::apply`.
    #[cfg(feature = "async")]
    #[tokio::test]
    async fn async_apply_rejects_a_stranger_even_with_an_any_state_transition() {
        use pacta_contract::AsyncRegistry;

        let registry = MemoryRegistryAsync::seeded(vec![a_pact()], 1000);
        let claim = registry
            .claim(&["d"], Timestamp::from_millis(0))
            .await
            .expect("claim should not error")
            .expect("a pact should be claimable");
        let stranger = Retainer::new(Uuid::new_v4());
        let accept_any = |_state: &State| Ok::<State, lifecycle::NotCurrentHolder>(State::Settled);
        assert_eq!(
            registry.apply(&stranger, &accept_any).await,
            Err(NotHeld),
            "the async binding also locates by retainer"
        );
        registry
            .fulfill(&claim.retainer)
            .await
            .expect("the held state was untouched, so the holder still settles");
    }

    /// The at-most-once invariant under concurrent contention, through the shared *portable* runner
    /// — the exact check any async backend runs, driven by OS threads and `block_on` (no runtime).
    #[cfg(feature = "async")]
    #[test]
    fn passes_async_contention() {
        pacta_conformance::run_async_contention(MemoryRegistryAsync::seeded);
    }
}
