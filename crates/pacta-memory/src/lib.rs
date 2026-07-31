//! An in-memory [`Registry`] backend with real lease and lapse semantics.
//!
//! This is the first concrete backend: a pure lifecycle state machine that holds
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

enum State {
    Available,
    Held { retainer: Uuid, expiry: Timestamp },
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
        Ok(Some(Claim {
            pact: records[index].pact.clone(),
            retainer,
            lease_expiry: expiry,
        }))
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
}
