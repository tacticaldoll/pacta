//! An in-memory [`Registry`] backend with real lease and lapse semantics.
//!
//! This is a **reference** backend, not a durable or production one: it holds pacts
//! in memory, so nothing survives the process. It exists to demonstrate correct
//! lifecycle semantics and to calibrate against — durable backends live outside this
//! workspace and prove themselves against `pacta-conformance` just as this one does.
//!
//! It is a pure lifecycle state machine that holds
//! pacts, leases claims for a user-supplied duration, reclaims lapsed pacts through
//! the normal claim path, and rotates the retainer on every claim so a stale holder
//! cannot settle. It reads no clock — time is injected into `claim` and `heartbeat`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Mutex;

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

enum State {
    Available,
    Held {
        retainer: Uuid,
        expiry: Timestamp,
    },
    /// Released, non-terminal: claimable again only at or after `rearm_at`.
    Deferred {
        rearm_at: Timestamp,
    },
    Settled,
}

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

    fn find_holder(records: &mut [Record], retainer: &Retainer) -> Option<usize> {
        records.iter().position(|record| {
            matches!(record.state, State::Held { retainer: held, .. } if held == retainer.id())
        })
    }
}

impl Registry for MemoryRegistry {
    type Error = NotHeld;

    fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        let claimable = records.iter().position(|record| {
            if !dockets.contains(&record.pact.docket.as_str()) {
                return false;
            }
            match record.state {
                State::Available => true,
                // A lapse: an expired hold is reclaimable through this claim path.
                State::Held { expiry, .. } => expiry < now,
                // A re-arm: a released pact is claimable once its instant has passed.
                State::Deferred { rearm_at } => rearm_at <= now,
                State::Settled => false,
            }
        });

        // Mint a retainer only on a successful claim.
        let Some(index) = claimable else {
            return Ok(None);
        };
        let retainer = Retainer::new(Uuid::new_v4());
        let expiry = now.plus_millis(self.lease_millis);
        records[index].state = State::Held {
            retainer: retainer.id(),
            expiry,
        };
        Ok(Some(Claim::new(
            records[index].pact.clone(),
            retainer,
            expiry,
        )))
    }

    fn heartbeat(&self, retainer: &Retainer, now: Timestamp) -> Result<(), Self::Error> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        let index = Self::find_holder(&mut records, retainer).ok_or(NotHeld)?;
        let State::Held { expiry, .. } = records[index].state else {
            return Err(NotHeld);
        };
        // Refuse to revive a lapsed lease: the holder must re-claim.
        if expiry < now {
            return Err(NotHeld);
        }
        records[index].state = State::Held {
            retainer: retainer.id(),
            expiry: now.plus_millis(self.lease_millis),
        };
        Ok(())
    }

    fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.settle(retainer)
    }

    fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.settle(retainer)
    }

    fn release(&self, retainer: &Retainer, rearm_at: Timestamp) -> Result<(), Self::Error> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        // Only the current holder may release; a settled or non-held pact has no
        // current holder, so this rejects settled-release and stale retainers alike —
        // the same authority check as fulfill and breach.
        let index = Self::find_holder(&mut records, retainer).ok_or(NotHeld)?;
        // Non-terminal: drop the hold (rotating authority away from this retainer) and
        // re-arm. The core honors the injected instant; it computes no delay.
        records[index].state = State::Deferred { rearm_at };
        Ok(())
    }
}

impl MemoryRegistry {
    // fulfill and breach share the same authority check: a stale retainer no longer
    // matches the rotated current holder, so no time is needed to reject it.
    fn settle(&self, retainer: &Retainer) -> Result<(), NotHeld> {
        let mut records = self
            .records
            .lock()
            .expect("registry mutex should not be poisoned");
        let index = Self::find_holder(&mut records, retainer).ok_or(NotHeld)?;
        records[index].state = State::Settled;
        Ok(())
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
