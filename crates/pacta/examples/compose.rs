//! Facade composition smoke: a downstream consumer wiring the whole lifecycle
//! through the single `pacta` entrypoint.
//!
//! It imports only from `pacta` — never from `pacta-contract`, `pacta-executor`, or
//! `pacta-driver` directly — proving the curated surface is composition-complete on
//! its own. It builds a ledger holding one claimable pact, wraps a performer with a
//! pass-through witness (the composition seam), and drives one mechanical step:
//! claim -> execute -> settle. The witness forwards execution unchanged and occupies
//! the exact slot a future retry or timeout middleware would fill; this example
//! carries no orchestration behaviour.

#![forbid(unsafe_code)]

use std::convert::Infallible;
use std::sync::Mutex;

use pacta::{
    Claim, Driver, Execution, Executor, Middleware, Outcome, Pact, Registry, Retainer, Step,
    Timestamp,
};
use uuid::Uuid;

/// A minimal in-memory registry: a ledger that hands out one held claim and then
/// has nothing left to claim. It is a pure lifecycle state machine — it inspects no
/// clause, computes no delay or backoff, and evaluates no policy.
struct Ledger {
    pending: Mutex<Option<Claim>>,
}

impl Registry for Ledger {
    type Error = Infallible;

    fn claim(&self, _dockets: &[&str], _now: Timestamp) -> Result<Option<Claim>, Self::Error> {
        Ok(self
            .pending
            .lock()
            .expect("ledger mutex should not be poisoned")
            .take())
    }

    fn heartbeat(&self, _retainer: &Retainer, _now: Timestamp) -> Result<(), Self::Error> {
        Ok(())
    }

    fn fulfill(&self, _retainer: &Retainer) -> Result<(), Self::Error> {
        Ok(())
    }

    fn breach(&self, _retainer: &Retainer) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// The party that carries out a claimed pact. Here it always fulfils.
struct Performer;

impl Executor for Performer {
    type Error = Infallible;

    fn execute(&mut self, _execution: Execution) -> Result<Outcome, Self::Error> {
        Ok(Outcome::Fulfilled)
    }
}

/// A pass-through middleware: it witnesses execution flowing through and forwards it
/// unchanged. This is the forward-compatibility seam — a real retry or timeout
/// middleware would later drop into exactly this position.
struct Witness;

impl<E: Executor> Middleware<E> for Witness {
    type Executor = Witnessed<E>;

    fn wrap(&self, executor: E) -> Self::Executor {
        Witnessed { inner: executor }
    }
}

/// The executor produced by wrapping another executor in a [`Witness`].
struct Witnessed<E> {
    inner: E,
}

impl<E: Executor> Executor for Witnessed<E> {
    type Error = E::Error;

    fn execute(&mut self, execution: Execution) -> Result<Outcome, Self::Error> {
        self.inner.execute(execution)
    }
}

fn pending_claim() -> Claim {
    Claim {
        pact: Pact {
            id: Uuid::new_v4(),
            docket: "default".to_string(),
            kind: "example".to_string(),
            clause: Vec::new(),
        },
        retainer: Retainer::new(Uuid::new_v4()),
        lease_expiry: Timestamp::from_millis(0),
    }
}

fn main() {
    let ledger = Ledger {
        pending: Mutex::new(Some(pending_claim())),
    };
    let performer = Witness.wrap(Performer);

    let mut driver = Driver::new(ledger, performer, ["default".to_string()]);

    let step = driver.step().expect("driver step should not fail");
    assert_eq!(step, Step::Fulfilled);

    println!("composed lifecycle step through the pacta facade: {step:?}");
}
