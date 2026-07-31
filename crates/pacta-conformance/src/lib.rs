//! A backend-agnostic conformance suite for [`Registry`] implementations.
//!
//! The suite is generic over `Registry` and takes a constructor closure that
//! returns a seeded backend, so it defines no seeding trait: a backend runs the
//! suite from its own `#[cfg(test)]` module and keeps `pacta-conformance` a pure
//! dev-dependency. Time is driven entirely through the trait by passing controlled
//! [`Timestamp`] values, never a wall clock.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fmt::Debug;

use pacta_contract::{Pact, Registry, Retainer, Timestamp};
use uuid::Uuid;

/// The lease duration, in milliseconds, the suite constructs every backend with.
pub const LEASE_MILLIS: u64 = 1000;

const DOCKET: &str = "conformance";

fn at(millis: u64) -> Timestamp {
    Timestamp::from_millis(millis)
}

fn a_pact_on(docket: &str) -> Pact {
    Pact::new(
        Uuid::new_v4(),
        docket.to_string(),
        "conformance".to_string(),
        Vec::new(),
    )
}

fn a_pact() -> Pact {
    a_pact_on(DOCKET)
}

/// Run the full conformance suite against a backend built by `make`.
///
/// `make(pacts, lease_millis)` must return a fresh registry seeded with `pacts`
/// and configured to lease claims for `lease_millis`. The suite calls it once per
/// scenario. A failing assertion panics, failing the calling test.
pub fn run<R, F>(make: F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    no_available_pact_returns_none(&make);
    unrequested_docket_is_not_claimed(&make);
    claim_returns_claim_with_lease(&make);
    held_pact_not_reclaimable_before_expiry(&make);
    expired_lease_lapses_and_reclaims_with_rotated_retainer(&make);
    stale_retainer_settle_rejected_after_reclaim(&make);
    late_fulfill_before_reclaim_succeeds(&make);
    fulfill_settles_and_pact_not_claimable(&make);
    breach_settles_terminally(&make);
    released_pact_withheld_until_reclaimable(&make);
    released_pact_reclaimable_at_its_instant(&make);
    immediate_reclaim_reclaims_like_lapse(&make);
    release_rotates_authority_from_prior_holder(&make);
    heartbeat_extends_lease_preventing_lapse(&make);
    heartbeat_at_expiry_boundary_succeeds(&make);
    heartbeat_on_lapsed_lease_rejected(&make);
    heartbeat_unknown_retainer_rejected(&make);
}

/// The number of rounds the contention checks repeat to surface a racing interleaving. This is a
/// **probabilistic stress**, not a deterministic proof: an atomic backend passes every round, and a
/// non-atomic one is overwhelmingly likely — but not guaranteed on any single round — to be caught
/// here. The harness's *teeth* are proven deterministically by the barrier-synchronized broken
/// fixture in this crate's tests, not by this repetition count.
pub const CONTENTION_ROUNDS: usize = 2000;

/// Verify a sync [`Registry`] backend upholds at-most-once authority under real concurrency: two
/// workers contending a settlement on one claimed pact settle it at most once, and two workers
/// contending a claim on one available pact issue at most one claim. Both are asserted through the
/// public trait only — never by inspecting the backend's lock, transaction, or compare-and-set
/// mechanism — so the check holds for any concurrency-control strategy. Concurrency is real OS
/// threads; a failing assertion panics, failing the calling test.
///
/// This is the sync sibling of [`run_async_contention`]; the async binding has its own because a
/// backend implements only one of the two bindings.
pub fn run_contention<R, F>(make: F)
where
    R: Registry + Send + Sync + 'static,
    R::Error: Debug + Send,
    F: Fn(Vec<Pact>, u64) -> R,
{
    settle_contention(&make);
    claim_contention(&make);
}

/// Two workers race a settlement on one claimed pact: exactly one succeeds, the other resolves to a
/// not-current-holder — the at-most-once `apply` invariant. Split out so the non-vacuity guard can
/// prove this branch has teeth independently of the claim branch.
fn settle_contention<R, F>(make: &F)
where
    R: Registry + Send + Sync + 'static,
    R::Error: Debug + Send,
    F: Fn(Vec<Pact>, u64) -> R,
{
    use std::sync::Arc;

    for _ in 0..CONTENTION_ROUNDS {
        let registry = Arc::new(make(vec![a_pact()], LEASE_MILLIS));
        let retainer = registry
            .claim(&[DOCKET], at(0))
            .expect("claim should not error")
            .expect("a pact should be claimable")
            .retainer;

        let a = {
            let registry = Arc::clone(&registry);
            let retainer = retainer.clone();
            std::thread::spawn(move || registry.fulfill(&retainer))
        };
        let b = {
            let registry = Arc::clone(&registry);
            let retainer = retainer.clone();
            std::thread::spawn(move || registry.fulfill(&retainer))
        };
        let (ra, rb) = (a.join().unwrap(), b.join().unwrap());

        let winners = [ra.is_ok(), rb.is_ok()]
            .into_iter()
            .filter(|&ok| ok)
            .count();
        assert_eq!(
            winners, 1,
            "a settlement must apply exactly once under contention: a={ra:?} b={rb:?}"
        );
        assert!(
            registry
                .claim(&[DOCKET], at(0))
                .expect("claim should not error")
                .is_none(),
            "a settled pact must not be claimable again"
        );
    }
}

/// Two workers race a claim on one available pact: exactly one gets a claim, never two — the
/// at-most-one-issue `claim` invariant. Split out so the non-vacuity guard can prove this branch has
/// teeth independently of the settlement branch.
fn claim_contention<R, F>(make: &F)
where
    R: Registry + Send + Sync + 'static,
    R::Error: Debug + Send,
    F: Fn(Vec<Pact>, u64) -> R,
{
    use std::sync::Arc;

    for _ in 0..CONTENTION_ROUNDS {
        let registry = Arc::new(make(vec![a_pact()], LEASE_MILLIS));
        let a = {
            let registry = Arc::clone(&registry);
            std::thread::spawn(move || {
                registry
                    .claim(&[DOCKET], at(0))
                    .expect("claim should not error")
                    .map(|claim| claim.retainer.id())
            })
        };
        let b = {
            let registry = Arc::clone(&registry);
            std::thread::spawn(move || {
                registry
                    .claim(&[DOCKET], at(0))
                    .expect("claim should not error")
                    .map(|claim| claim.retainer.id())
            })
        };
        let (ra, rb) = (a.join().unwrap(), b.join().unwrap());

        let claims = [ra, rb].into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(
            claims.len(),
            1,
            "exactly one worker must claim the single available pact: {claims:?}"
        );
    }
}

/// Async conformance: hold an [`AsyncRegistry`](pacta_contract::AsyncRegistry) backend to the
/// exact same scenarios as the sync suite.
///
/// The async runner reuses [`run`] rather than a duplicated scenario set: it adapts the async
/// backend into the sync [`Registry`] by driving each operation to completion with a
/// [`BlockingDriver`], so sync and async coverage cannot drift. This proves state-machine parity —
/// the same bar the sync suite meets, which itself exercises no concurrency. The at-most-once
/// invariant under concurrent contention is a separate check (`run_async_contention`).
///
/// Two entries drive the one shared scenario set, differing only in the driver:
/// [`run_async_with`] takes a caller-supplied [`BlockingDriver`], so a **real-reactor** backend runs
/// the scenarios on its own runtime; [`run_async`] uses the built-in [`SelfProgress`] driver and is
/// correct only for backends whose futures make progress without an external reactor. Neither
/// imposes a `Send` bound on the backend's futures or pulls an async runtime into the crate.
#[cfg(feature = "async")]
mod async_runner {
    use core::future::Future;

    use pacta_contract::AsyncRegistry;
    use pacta_contract::{Claim, Pact, Registry, Retainer, Timestamp, Transition};

    /// Drives an async operation to completion — the seam by which a backend supplies its own
    /// runtime without the conformance suite committing to one. The method is generic over the
    /// future and imposes **no `Send` bound**, so future coloring stays the backend's; it is a
    /// static bound (no `dyn`, no boxing), so the crate takes on no async-runtime dependency.
    ///
    /// A real-reactor backend implements this over its runtime (for example a wrapper whose `drive`
    /// calls `tokio::runtime::Runtime::block_on`) and passes it to [`run_async_with`]. A backend
    /// whose futures are ready without a reactor uses the built-in [`SelfProgress`].
    pub trait BlockingDriver {
        /// Drive `future` to completion and return its output.
        fn drive<F: Future>(&self, future: F) -> F::Output;
    }

    /// The built-in [`BlockingDriver`] for backends whose futures make progress **without** an
    /// external reactor (a ready-future backend, such as the in-memory reference). It drives with a
    /// no-op-waker poll loop and pulls no async runtime. It is **not** correct for a backend whose
    /// futures park pending an external event (real I/O, a timer): such a backend must pass its own
    /// runtime to [`run_async_with`] instead, or this driver will spin without ever completing.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct SelfProgress;

    impl BlockingDriver for SelfProgress {
        fn drive<F: Future>(&self, future: F) -> F::Output {
            block_on(future)
        }
    }

    /// Drive a future to completion on the current thread with a no-op waker. Correct for futures
    /// that make progress without an external reactor; keeps the crate dependency- and unsafe-free.
    fn block_on<F: Future>(future: F) -> F::Output {
        use core::task::{Context, Poll};

        let mut future = core::pin::pin!(future);
        let mut cx = Context::from_waker(core::task::Waker::noop());
        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => core::hint::spin_loop(),
            }
        }
    }

    /// Adapts an [`AsyncRegistry`] into the sync [`Registry`] by driving each primitive to
    /// completion through a [`BlockingDriver`], so the async binding runs the sync suite verbatim.
    /// Because both bindings share one transition port, the adapter forwards only the primitives
    /// (`claim`, `lease_millis`, `apply`); the four transition ops come from the sync trait's
    /// default methods over `apply`.
    struct BlockOn<R, D> {
        registry: R,
        driver: D,
    }

    impl<R: AsyncRegistry, D: BlockingDriver + Send + Sync> Registry for BlockOn<R, D> {
        type Error = R::Error;

        fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error> {
            self.driver.drive(self.registry.claim(dockets, now))
        }

        fn lease_millis(&self) -> u64 {
            self.registry.lease_millis()
        }

        fn apply(
            &self,
            retainer: &Retainer,
            transition: &Transition<'_>,
        ) -> Result<(), Self::Error> {
            self.driver.drive(self.registry.apply(retainer, transition))
        }
    }

    /// Run the full conformance suite against an async backend built by `make`, driving its futures
    /// with a caller-supplied `driver`. A real-reactor backend passes a driver wrapping its own
    /// runtime, so the shared scenarios run on that runtime — no scenario is re-declared, and the
    /// entry imposes no `Send` bound and adds no async-runtime dependency.
    ///
    /// `make(pacts, lease_millis)` returns a fresh async registry seeded with `pacts`, exactly as
    /// [`run`](crate::run) expects for the sync binding.
    pub fn run_async_with<R, F, D>(make: F, driver: D)
    where
        R: AsyncRegistry,
        R::Error: core::fmt::Debug,
        F: Fn(Vec<Pact>, u64) -> R,
        D: BlockingDriver + Copy + Send + Sync,
    {
        crate::run(move |pacts, lease_millis| BlockOn {
            registry: make(pacts, lease_millis),
            driver,
        });
    }

    /// Run the full conformance suite against a **ready-future** async backend built by `make`,
    /// driving its futures with the built-in [`SelfProgress`] driver.
    ///
    /// This is a convenience over [`run_async_with`] for backends whose futures make progress
    /// without an external reactor (the in-memory reference backend). A backend whose futures park
    /// pending real I/O or a timer must use [`run_async_with`] with a driver over its own runtime;
    /// `SelfProgress` would spin without completing such a future.
    pub fn run_async<R, F>(make: F)
    where
        R: AsyncRegistry,
        R::Error: core::fmt::Debug,
        F: Fn(Vec<Pact>, u64) -> R,
    {
        run_async_with(make, SelfProgress);
    }

    /// Verify at-most-once authority under concurrent contention, for a ready-future async backend.
    ///
    /// Two checks, each through the public ops only (never inspecting the backend's concurrency
    /// mechanism), so both hold for a lock, a transaction, or a compare-and-set backend alike:
    /// two workers race a settlement on one claimed pact — exactly one succeeds; and two workers
    /// race a claim on one available pact — exactly one gets a claim.
    ///
    /// Parallelism is real (OS threads); each thread drives its future to completion with
    /// `block_on`, so a future never migrates across threads and **no `Send` bound on the future is
    /// required** — the suite pulls no async runtime. Like [`run_async`], this convenience is for
    /// ready-future backends; the repetition count is a **probabilistic stress**, not a
    /// deterministic proof (the harness's teeth are proven by the barrier-synchronized broken
    /// fixture in this crate's tests).
    pub fn run_async_contention<R, F>(make: F)
    where
        R: AsyncRegistry + 'static,
        R::Error: core::fmt::Debug + Send,
        F: Fn(Vec<Pact>, u64) -> R,
    {
        use std::sync::Arc;

        // Two workers race a settlement on one claimed pact: exactly one succeeds.
        for _ in 0..crate::CONTENTION_ROUNDS {
            let reg = Arc::new(make(vec![crate::a_pact()], crate::LEASE_MILLIS));
            let retainer = block_on(reg.claim(&[crate::DOCKET], crate::at(0)))
                .expect("claim should not error")
                .expect("a pact should be claimable")
                .retainer;

            let a = {
                let reg = Arc::clone(&reg);
                let retainer = retainer.clone();
                std::thread::spawn(move || block_on(reg.fulfill(&retainer)))
            };
            let b = {
                let reg = Arc::clone(&reg);
                let retainer = retainer.clone();
                std::thread::spawn(move || block_on(reg.fulfill(&retainer)))
            };
            let (ra, rb) = (a.join().unwrap(), b.join().unwrap());

            let winners = [ra.is_ok(), rb.is_ok()]
                .into_iter()
                .filter(|&ok| ok)
                .count();
            assert_eq!(
                winners, 1,
                "settlement must apply exactly once: a={ra:?} b={rb:?}"
            );
            assert!(
                block_on(reg.claim(&[crate::DOCKET], crate::at(0)))
                    .expect("claim should not error")
                    .is_none(),
                "a settled pact must not be claimable again"
            );
        }

        // Two workers race a claim on one available pact: exactly one gets a claim, never two.
        for _ in 0..crate::CONTENTION_ROUNDS {
            let reg = Arc::new(make(vec![crate::a_pact()], crate::LEASE_MILLIS));
            let a = {
                let reg = Arc::clone(&reg);
                std::thread::spawn(move || {
                    block_on(reg.claim(&[crate::DOCKET], crate::at(0)))
                        .expect("claim should not error")
                        .map(|claim| claim.retainer.id())
                })
            };
            let b = {
                let reg = Arc::clone(&reg);
                std::thread::spawn(move || {
                    block_on(reg.claim(&[crate::DOCKET], crate::at(0)))
                        .expect("claim should not error")
                        .map(|claim| claim.retainer.id())
                })
            };
            let (ra, rb) = (a.join().unwrap(), b.join().unwrap());

            let claims = [ra, rb].into_iter().flatten().collect::<Vec<_>>();
            assert_eq!(
                claims.len(),
                1,
                "exactly one worker must claim the single available pact: {claims:?}"
            );
        }
    }

    /// A reactor-backed fixture proving the runtime-compatible entry drives futures that a naive
    /// poll loop cannot: each op first parks on a real timer (`tokio::time::sleep`) and completes
    /// only when a real runtime advances it. It runs the whole shared scenario set through the
    /// public [`run_async_with`] entry, on a current-thread Tokio runtime — establishing that a
    /// backend needing an external reactor reuses the exact same scenarios without re-declaring
    /// them, and that the entry forces no `Send` future (a current-thread runtime needs none).
    #[cfg(test)]
    mod reactor_fixture {
        use super::*;
        use core::time::Duration;
        use pacta_contract::lifecycle::{self, State};
        use std::sync::Mutex;
        use uuid::Uuid;

        #[derive(Debug, PartialEq, Eq)]
        struct NotHeld;
        impl core::fmt::Display for NotHeld {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("not held")
            }
        }
        impl std::error::Error for NotHeld {}
        impl From<lifecycle::NotCurrentHolder> for NotHeld {
            fn from(_: lifecycle::NotCurrentHolder) -> Self {
                NotHeld
            }
        }

        /// An async backend whose every op parks on a real timer before touching its store, so it
        /// makes no progress without a runtime driving the timer. Otherwise a faithful in-memory
        /// backend: atomic claim, and `apply` that locates the record held by the retainer.
        struct ReactorBacked {
            records: Mutex<Vec<(Pact, State)>>,
            lease_millis: u64,
        }

        impl ReactorBacked {
            fn seeded(pacts: Vec<Pact>, lease_millis: u64) -> Self {
                Self {
                    records: Mutex::new(pacts.into_iter().map(|p| (p, State::Available)).collect()),
                    lease_millis,
                }
            }

            async fn park() {
                // A real timer: pending until the runtime's time driver advances it. A no-op-waker
                // poll loop would spin here forever, so this backend needs a real runtime.
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }

        impl AsyncRegistry for ReactorBacked {
            type Error = NotHeld;

            async fn claim(
                &self,
                dockets: &[&str],
                now: Timestamp,
            ) -> Result<Option<Claim>, NotHeld> {
                Self::park().await;
                let mut records = self.records.lock().unwrap();
                let Some(index) = records.iter().position(|(pact, state)| {
                    dockets.contains(&pact.docket.as_str()) && lifecycle::is_claimable(state, now)
                }) else {
                    return Ok(None);
                };
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
                retainer: &Retainer,
                transition: &Transition<'_>,
            ) -> Result<(), NotHeld> {
                Self::park().await;
                let mut records = self.records.lock().unwrap();
                let (_, state) = records
                    .iter_mut()
                    .find(|(_, state)| {
                        matches!(state, State::Held { retainer: held, .. } if held == retainer)
                    })
                    .ok_or(NotHeld)?;
                *state = transition(state)?;
                Ok(())
            }
        }

        #[derive(Clone, Copy)]
        struct TokioDriver<'a>(&'a tokio::runtime::Runtime);
        impl BlockingDriver for TokioDriver<'_> {
            fn drive<F: Future>(&self, future: F) -> F::Output {
                self.0.block_on(future)
            }
        }

        #[test]
        fn reactor_backed_backend_runs_the_suite_via_run_async_with() {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
                .expect("current-thread runtime with a timer");
            // The same shared scenario set a ready-future backend runs — now driven on a real
            // runtime, over a backend whose futures genuinely park pending a timer.
            run_async_with(ReactorBacked::seeded, TokioDriver(&runtime));
        }
    }
}

#[cfg(feature = "async")]
pub use async_runner::{
    BlockingDriver, SelfProgress, run_async, run_async_contention, run_async_with,
};

fn no_available_pact_returns_none<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(Vec::new(), LEASE_MILLIS);
    assert!(
        registry
            .claim(&[DOCKET], at(0))
            .expect("claim should not error")
            .is_none(),
        "an empty registry must yield no claim"
    );
}

fn unrequested_docket_is_not_claimed<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact_on("other")], LEASE_MILLIS);
    assert!(
        registry
            .claim(&[DOCKET], at(0))
            .expect("claim should not error")
            .is_none(),
        "a pact on an unrequested docket must not be claimed"
    );
    assert!(
        registry
            .claim(&["other"], at(0))
            .expect("claim should not error")
            .is_some(),
        "the same pact must be claimable from its own docket"
    );
}

fn claim_returns_claim_with_lease<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(100))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    assert_eq!(
        claim.lease_expiry,
        at(100 + LEASE_MILLIS),
        "lease expiry must be now plus the lease duration"
    );
}

fn held_pact_not_reclaimable_before_expiry<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let _first = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    assert!(
        registry
            .claim(&[DOCKET], at(500))
            .expect("claim should not error")
            .is_none(),
        "a held pact must not be reclaimable before its lease expires"
    );
}

fn expired_lease_lapses_and_reclaims_with_rotated_retainer<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let first = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    let second = registry
        .claim(&[DOCKET], at(1500))
        .expect("claim should not error")
        .expect("an expired pact should be reclaimable through the claim path");
    assert_ne!(
        first.retainer.id(),
        second.retainer.id(),
        "reclaiming a lapsed pact must rotate the retainer"
    );
    assert_eq!(
        second.lease_expiry,
        at(1500 + LEASE_MILLIS),
        "the reclaim must set a fresh lease"
    );
}

fn stale_retainer_settle_rejected_after_reclaim<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let first = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    let _second = registry
        .claim(&[DOCKET], at(1500))
        .expect("claim should not error")
        .expect("an expired pact should be reclaimable");
    assert!(
        registry.fulfill(&first.retainer).is_err(),
        "the prior holder must not settle after a reclaim (at-least-once safety)"
    );
}

fn late_fulfill_before_reclaim_succeeds<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    // The lease has expired but nobody reclaimed; the holder's retainer still
    // matches, so a late fulfill of genuinely-done work settles. No time involved.
    assert!(
        registry.fulfill(&claim.retainer).is_ok(),
        "a late fulfill before any reclaim must settle"
    );
    assert!(
        registry
            .claim(&[DOCKET], at(9999))
            .expect("claim should not error")
            .is_none(),
        "a settled pact must not be claimable"
    );
}

fn fulfill_settles_and_pact_not_claimable<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .fulfill(&claim.retainer)
        .expect("fulfill should settle");
    assert!(
        registry
            .claim(&[DOCKET], at(0))
            .expect("claim should not error")
            .is_none(),
        "a fulfilled pact must not be claimable"
    );
}

fn breach_settles_terminally<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .breach(&claim.retainer)
        .expect("breach should settle");
    assert!(
        registry
            .claim(&[DOCKET], at(5000))
            .expect("claim should not error")
            .is_none(),
        "a breached pact must not be claimable, even after its lease would have expired"
    );
}

fn released_pact_withheld_until_reclaimable<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .release(&claim.retainer, at(5000))
        .expect("release should succeed for the current holder");
    // at(3000) is past the original lease (1000) — a lapse would make it claimable —
    // but the reclaimable instant (5000) is later, so release must withhold it.
    assert!(
        registry
            .claim(&[DOCKET], at(3000))
            .expect("claim should not error")
            .is_none(),
        "a released pact must not be claimable before its reclaimable instant"
    );
}

fn released_pact_reclaimable_at_its_instant<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let first = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .release(&first.retainer, at(5000))
        .expect("release should succeed");
    let second = registry
        .claim(&[DOCKET], at(5000))
        .expect("claim should not error")
        .expect("a released pact must be claimable at its reclaimable instant");
    assert_ne!(
        first.retainer.id(),
        second.retainer.id(),
        "reclaiming a released pact must rotate the retainer"
    );
}

fn immediate_reclaim_reclaims_like_lapse<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .release(&claim.retainer, at(0))
        .expect("release with an immediate reclaim should succeed");
    assert!(
        registry
            .claim(&[DOCKET], at(0))
            .expect("claim should not error")
            .is_some(),
        "an immediate reclaim must make the pact claimable at once, as a voluntary lapse"
    );
}

fn release_rotates_authority_from_prior_holder<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .release(&claim.retainer, at(0))
        .expect("release should succeed");
    assert!(
        registry.fulfill(&claim.retainer).is_err(),
        "the prior holder must not settle after releasing (release rotates authority)"
    );
}

fn heartbeat_extends_lease_preventing_lapse<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    registry
        .heartbeat(&claim.retainer, at(800))
        .expect("an in-window heartbeat should extend the lease");
    // The original lease (expiry 1000) would have lapsed by 1500, but the
    // heartbeat pushed expiry to 1800, so the pact is still held.
    assert!(
        registry
            .claim(&[DOCKET], at(1500))
            .expect("claim should not error")
            .is_none(),
        "a heartbeat within the window must prevent a lapse"
    );
}

fn heartbeat_at_expiry_boundary_succeeds<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    // The lease expires at now(0) + LEASE_MILLIS. A heartbeat *exactly at* that instant is still
    // in-window (the lease is valid up to and including its expiry: `expiry >= now`), so it must
    // extend, not be rejected — only a strictly later heartbeat lapses.
    registry
        .heartbeat(&claim.retainer, at(LEASE_MILLIS))
        .expect("a heartbeat at now == expiry must extend the lease, not be rejected");
    // The boundary heartbeat pushed expiry to 2 * LEASE_MILLIS, so the pact is still held at 1.5x.
    assert!(
        registry
            .claim(&[DOCKET], at(LEASE_MILLIS + LEASE_MILLIS / 2))
            .expect("claim should not error")
            .is_none(),
        "a heartbeat at the expiry boundary must extend the lease and prevent a lapse"
    );
}

fn heartbeat_on_lapsed_lease_rejected<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    assert!(
        registry.heartbeat(&claim.retainer, at(1200)).is_err(),
        "a heartbeat after the lease expired must be rejected, forcing a re-claim"
    );
}

fn heartbeat_unknown_retainer_rejected<R, F>(make: &F)
where
    R: Registry,
    R::Error: Debug,
    F: Fn(Vec<Pact>, u64) -> R,
{
    let registry = make(vec![a_pact()], LEASE_MILLIS);
    let _claim = registry
        .claim(&[DOCKET], at(0))
        .expect("claim should not error")
        .expect("a pact should be claimable");
    let unknown = Retainer::new(Uuid::new_v4());
    assert!(
        registry.heartbeat(&unknown, at(100)).is_err(),
        "a heartbeat with an unissued retainer must be rejected"
    );
}

#[cfg(test)]
mod contention_guard {
    //! Proves the contention harness has teeth: a deterministically non-atomic backend must fail
    //! [`run_contention`], and a matching atomic one must pass — so the gate is not vacuous.

    use super::{Registry, claim_contention, run_contention, settle_contention};
    use pacta_contract::lifecycle::{self, State};
    use pacta_contract::{Claim, Pact, Retainer, Timestamp, Transition};
    use std::sync::{Barrier, Mutex};
    use uuid::Uuid;

    #[derive(Debug, PartialEq, Eq)]
    struct NotHeld;
    impl std::fmt::Display for NotHeld {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("not held")
        }
    }
    impl std::error::Error for NotHeld {}
    impl From<lifecycle::NotCurrentHolder> for NotHeld {
        fn from(_: lifecycle::NotCurrentHolder) -> Self {
            NotHeld
        }
    }

    /// Which operation this fixture makes deliberately non-atomic. Each broken mode forces a
    /// deterministic double-issue on its op via a two-party barrier, so the matching contention check
    /// must catch it.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Atomic,
        BrokenApply,
        BrokenClaim,
    }

    /// A single-pact in-memory backend used three ways. `Atomic` is a correct one-lock-scope backend.
    /// `BrokenApply` makes `apply` non-atomic: it loads, waits until both contending workers have
    /// loaded the same pre-state, then stores by index unconditionally — a forced double-apply.
    /// `BrokenClaim` makes `claim` non-atomic the same way: both workers see the pact available before
    /// either marks it held, so both mint a claim — a forced double-issue.
    struct TestBackend {
        records: Mutex<Vec<(Pact, State)>>,
        lease_millis: u64,
        mode: Mode,
        barrier: Option<Barrier>,
    }

    impl TestBackend {
        fn build(pacts: Vec<Pact>, lease_millis: u64, mode: Mode) -> Self {
            Self {
                records: Mutex::new(pacts.into_iter().map(|p| (p, State::Available)).collect()),
                lease_millis,
                mode,
                barrier: (mode != Mode::Atomic).then(|| Barrier::new(2)),
            }
        }

        fn atomic(pacts: Vec<Pact>, lease_millis: u64) -> Self {
            Self::build(pacts, lease_millis, Mode::Atomic)
        }

        fn broken_apply(pacts: Vec<Pact>, lease_millis: u64) -> Self {
            Self::build(pacts, lease_millis, Mode::BrokenApply)
        }

        fn broken_claim(pacts: Vec<Pact>, lease_millis: u64) -> Self {
            Self::build(pacts, lease_millis, Mode::BrokenClaim)
        }

        fn held_index(records: &[(Pact, State)], retainer: &Retainer) -> Option<usize> {
            records
                .iter()
                .position(|(_, state)| matches!(state, State::Held { retainer: held, .. } if held == retainer))
        }

        fn available_index(
            records: &[(Pact, State)],
            dockets: &[&str],
            now: Timestamp,
        ) -> Option<usize> {
            records.iter().position(|(pact, state)| {
                dockets.contains(&pact.docket.as_str()) && lifecycle::is_claimable(state, now)
            })
        }
    }

    impl Registry for TestBackend {
        type Error = NotHeld;

        fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, NotHeld> {
            let mint = |records: &mut Vec<(Pact, State)>, index: usize| {
                let retainer = Retainer::new(Uuid::new_v4());
                records[index].1 = lifecycle::on_claim(&retainer, now, self.lease_millis);
                let expiry = lifecycle::lease_expiry(now, self.lease_millis);
                Claim::new(records[index].0.clone(), retainer, expiry)
            };
            match &self.barrier {
                // Non-atomic claim: both workers observe the pact available before either marks it
                // held, so both mint — a forced double-issue.
                Some(barrier) if self.mode == Mode::BrokenClaim => {
                    let index = {
                        let records = self.records.lock().unwrap();
                        Self::available_index(&records, dockets, now)
                    };
                    let Some(index) = index else { return Ok(None) };
                    barrier.wait();
                    let mut records = self.records.lock().unwrap();
                    Ok(Some(mint(&mut records, index)))
                }
                // Atomic claim (the default, and for the BrokenApply fixture whose claim is fine).
                _ => {
                    let mut records = self.records.lock().unwrap();
                    let Some(index) = Self::available_index(&records, dockets, now) else {
                        return Ok(None);
                    };
                    Ok(Some(mint(&mut records, index)))
                }
            }
        }

        fn lease_millis(&self) -> u64 {
            self.lease_millis
        }

        fn apply(&self, retainer: &Retainer, transition: &Transition<'_>) -> Result<(), NotHeld> {
            match &self.barrier {
                // Non-atomic apply: load (release lock), let both workers reach the same pre-state,
                // then store by the loaded index unconditionally — a forced double-apply.
                Some(barrier) if self.mode == Mode::BrokenApply => {
                    let (index, state) = {
                        let records = self.records.lock().unwrap();
                        let index = Self::held_index(&records, retainer).ok_or(NotHeld)?;
                        (index, records[index].1.clone())
                    };
                    barrier.wait();
                    let next = transition(&state)?;
                    let mut records = self.records.lock().unwrap();
                    records[index].1 = next;
                    Ok(())
                }
                // Atomic apply: load, decide, and store within one lock scope.
                _ => {
                    let mut records = self.records.lock().unwrap();
                    let index = Self::held_index(&records, retainer).ok_or(NotHeld)?;
                    records[index].1 = transition(&records[index].1)?;
                    Ok(())
                }
            }
        }
    }

    /// Run `body` with the panic backtrace suppressed (a deliberate failure would print one),
    /// returning whether it panicked.
    fn panicked(body: impl FnOnce() + std::panic::UnwindSafe) -> bool {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(body);
        std::panic::set_hook(previous);
        result.is_err()
    }

    #[test]
    fn harness_catches_a_non_atomic_apply() {
        assert!(
            panicked(|| settle_contention(&TestBackend::broken_apply)),
            "the settlement-contention check must fail against a non-atomic apply"
        );
    }

    #[test]
    fn harness_catches_a_non_atomic_claim() {
        assert!(
            panicked(|| claim_contention(&TestBackend::broken_claim)),
            "the claim-contention check must fail against a non-atomic claim (double-issue)"
        );
    }

    #[test]
    fn harness_passes_a_fully_atomic_backend() {
        // The matching atomic backend passes both branches, so each guard above distinguishes teeth
        // from always-firing.
        run_contention(TestBackend::atomic);
    }
}
