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
    heartbeat_on_lapsed_lease_rejected(&make);
    heartbeat_unknown_retainer_rejected(&make);
}

/// Async conformance: hold an [`AsyncRegistry`](pacta_contract_async::AsyncRegistry) backend to the
/// exact same scenarios as the sync suite.
///
/// The async runner reuses [`run`] rather than a duplicated scenario set: it adapts the async
/// backend into the sync [`Registry`] by driving each operation to completion, so sync and async
/// coverage cannot drift. This proves state-machine parity — the same bar the sync suite meets, which
/// itself exercises no concurrency; concurrent contention on the async binding's `load`/`cas`
/// decomposition is a separate, backend-side proof.
///
/// The adapter drives futures with a poll loop, so it fits backends whose futures make progress
/// without an external reactor (the in-memory reference backend); a real-reactor durable backend
/// proves itself against the same scenarios through its own async harness.
#[cfg(feature = "async")]
mod async_runner {
    use core::future::Future;

    use pacta_contract::{Claim, Pact, Registry, Retainer, Timestamp, Transition};
    use pacta_contract_async::AsyncRegistry;

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

    /// Adapts an [`AsyncRegistry`] into the sync [`Registry`] by blocking on each primitive, so the
    /// async binding runs the sync suite verbatim. Because both bindings share one transition port,
    /// the adapter forwards only the primitives (`claim`, `lease_millis`, `apply`); the four
    /// transition ops come from the sync trait's default methods over `apply`.
    struct BlockOn<R>(R);

    impl<R: AsyncRegistry> Registry for BlockOn<R> {
        type Error = R::Error;

        fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error> {
            block_on(self.0.claim(dockets, now))
        }

        fn lease_millis(&self) -> u64 {
            self.0.lease_millis()
        }

        fn apply(
            &self,
            retainer: &Retainer,
            transition: &Transition<'_>,
        ) -> Result<(), Self::Error> {
            block_on(self.0.apply(retainer, transition))
        }
    }

    /// Run the full conformance suite against an async backend built by `make`.
    ///
    /// `make(pacts, lease_millis)` returns a fresh async registry seeded with `pacts`, exactly as
    /// [`run`](crate::run) expects for the sync binding.
    pub fn run_async<R, F>(make: F)
    where
        R: AsyncRegistry,
        R::Error: core::fmt::Debug,
        F: Fn(Vec<Pact>, u64) -> R,
    {
        crate::run(move |pacts, lease_millis| BlockOn(make(pacts, lease_millis)));
    }
}

#[cfg(feature = "async")]
pub use async_runner::run_async;

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
