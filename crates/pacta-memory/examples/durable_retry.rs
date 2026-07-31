//! Durable retry via `release`: the backoff pattern the primitive exists for.
//!
//! On a failed attempt, the consumer computes a backoff instant (the *policy*) and calls
//! `release(retainer, reclaimable_at = now + backoff)` — pacta withholds the pact until
//! that instant, then it is reclaimed and retried. The core computes no delay; the backoff
//! lives here, in the consumer. Time is injected, never read from a clock.
//!
//! This is durable retry, not in-executor retry: the claim is handed back between attempts,
//! so a backoff survives process death and never holds a lease while it waits.

use pacta_contract::{Pact, Registry, Timestamp};
use pacta_memory::MemoryRegistry;
use uuid::Uuid;

const DOCKET: &str = "retries";
const LEASE_MILLIS: u64 = 1000;

fn at(millis: u64) -> Timestamp {
    Timestamp::from_millis(millis)
}

/// The backoff *policy* — consumer-owned. A linear 1s-per-attempt here; a real consumer
/// would choose its own curve. The core never sees or computes this.
fn backoff_millis(attempt: u32) -> u64 {
    1000 * u64::from(attempt)
}

fn a_pact() -> Pact {
    Pact::new(
        Uuid::new_v4(),
        DOCKET.to_string(),
        "retry-demo".to_string(),
        Vec::new(),
    )
}

fn main() {
    // Seed one pact. (Creation is backend-inherent — here via the reference constructor.)
    let registry = MemoryRegistry::seeded(vec![a_pact()], LEASE_MILLIS);
    let mut attempts = 0u32;
    let mut now = 0u64;

    // Attempt 1 at t=0: claim, the work fails, release with a consumer-computed backoff.
    let first = registry
        .claim(&[DOCKET], at(now))
        .expect("claim should not error")
        .expect("the pact should be claimable");
    attempts += 1;
    let delay = backoff_millis(attempts); // consumer policy
    registry
        .release(&first.retainer, at(now + delay))
        .expect("release should succeed for the current holder");
    println!(
        "attempt {attempts} failed; released until t={}",
        now + delay
    );

    // During backoff: a claim before the reclaimable instant yields nothing.
    assert!(
        registry
            .claim(&[DOCKET], at(now + delay / 2))
            .expect("claim should not error")
            .is_none(),
        "a released pact must be withheld before its reclaimable instant"
    );

    // Attempt 2 at the reclaimable instant: reclaim (rotated retainer), succeed, fulfill.
    now += delay;
    let second = registry
        .claim(&[DOCKET], at(now))
        .expect("claim should not error")
        .expect("the pact should be reclaimable at its reclaimable instant");
    assert_ne!(
        first.retainer.id(),
        second.retainer.id(),
        "reclaiming must rotate the retainer"
    );
    attempts += 1;
    registry
        .fulfill(&second.retainer)
        .expect("fulfill should settle the reclaimed pact");
    println!("attempt {attempts} succeeded; fulfilled");

    // Settled: no longer claimable, even past when its lease would have expired.
    assert!(
        registry
            .claim(&[DOCKET], at(now + LEASE_MILLIS + 1))
            .expect("claim should not error")
            .is_none(),
        "a fulfilled pact must not be claimable again"
    );
    assert_eq!(
        attempts, 2,
        "the pact should have taken exactly two attempts"
    );

    println!("durable retry OK: withheld through backoff, reclaimed, fulfilled on retry");
}
