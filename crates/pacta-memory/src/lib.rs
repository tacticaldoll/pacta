//! An in-memory [`Registry`] backend with real lease and lapse semantics.
//!
//! This is a **reference** backend, not a durable or production one: it holds pacts
//! in memory, so nothing survives the process. It exists to demonstrate correct
//! lifecycle semantics and to calibrate against — durable backends live outside this
//! workspace and prove themselves against `pacta-conformance` just as this one does.
//!
//! It owns only storage and retainer minting; every eligibility decision and state
//! transition is delegated to the shared, pure [`pacta_contract::lifecycle`] kernel, so
//! the lifecycle semantics are single-sourced and cannot drift from any other backend.
//! It reads no clock — time is injected into `claim` and `heartbeat`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Mutex;

use pacta_contract::lifecycle::{self, State};
use pacta_contract::{Claim, Pact, Registry, Retainer, Timestamp};
use uuid::Uuid;

/// The error a memory registry returns when a retainer is not the current holder,
/// or when a heartbeat arrives after its lease has already lapsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotHeld;

impl std::fmt::Display for NotHeld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "retainer is not the current holder of any claim")
    }
}

impl std::error::Error for NotHeld {}

struct Record {
    pact: Pact,
    state: State,
}

/// An in-memory registry seeded with a fixed set of pacts.
pub struct MemoryRegistry {
    records: Mutex<Vec<Record>>,
    lease_millis: u64,
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

    /// Apply a lifecycle transition to the one record its current holder owns: the
    /// first record for which the transition succeeds is updated. If none succeed, no
    /// record is held by that retainer (stale, settled, or lapsed) — `NotHeld`. The
    /// authority check lives in the `lifecycle` kernel, not here; this only scans
    /// storage and writes the new state.
    fn apply_transition(
        &self,
        transition: impl Fn(&State) -> Result<State, lifecycle::NotCurrentHolder>,
    ) -> Result<(), NotHeld> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        for record in records.iter_mut() {
            if let Ok(next) = transition(&record.state) {
                record.state = next;
                return Ok(());
            }
        }
        Err(NotHeld)
    }
}

impl Registry for MemoryRegistry {
    type Error = NotHeld;

    fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        // Storage picks a candidate on the requested dockets; the kernel decides
        // eligibility (available / lapsed hold / reclaimable defer / settled).
        let claimable = records.iter().position(|record| {
            dockets.contains(&record.pact.docket.as_str())
                && lifecycle::is_claimable(&record.state, now)
        });

        // Mint a retainer only on a successful claim; the kernel produces the held state.
        let Some(index) = claimable else {
            return Ok(None);
        };
        let retainer = Retainer::new(Uuid::new_v4());
        records[index].state = lifecycle::on_claim(&retainer, now, self.lease_millis);
        let expiry = lifecycle::lease_expiry(now, self.lease_millis);
        Ok(Some(Claim::new(
            records[index].pact.clone(),
            retainer,
            expiry,
        )))
    }

    fn heartbeat(&self, retainer: &Retainer, now: Timestamp) -> Result<(), Self::Error> {
        self.apply_transition(|state| {
            lifecycle::on_heartbeat(state, retainer, now, self.lease_millis)
        })
    }

    fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        // fulfill and breach share the same lifecycle transition: the pact concludes.
        self.apply_transition(|state| lifecycle::on_settle(state, retainer))
    }

    fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.apply_transition(|state| lifecycle::on_settle(state, retainer))
    }

    fn release(&self, retainer: &Retainer, reclaimable_at: Timestamp) -> Result<(), Self::Error> {
        self.apply_transition(|state| lifecycle::on_release(state, retainer, reclaimable_at))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_registry_conformance() {
        pacta_conformance::run(MemoryRegistry::seeded);
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
}
