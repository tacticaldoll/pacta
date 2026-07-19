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
    Pact {
        id: Uuid::new_v4(),
        docket: docket.to_string(),
        kind: "conformance".to_string(),
        clause: Vec::new(),
    }
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
    heartbeat_extends_lease_preventing_lapse(&make);
    heartbeat_on_lapsed_lease_rejected(&make);
    heartbeat_unknown_retainer_rejected(&make);
}

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
