//! An asynchronous binding of the Pacta `Registry` contract.
//!
//! Real durable backends do async I/O and cannot implement the synchronous
//! [`pacta_contract::Registry`]. [`AsyncRegistry`] is the same frozen five-op contract, made
//! asynchronous — a *second binding*, not new semantics. The lifecycle semantics are
//! single-sourced in [`pacta_contract::lifecycle`], which this binding composes over, so the
//! sync and async bindings cannot drift.
//!
//! A backend implements only two primitives — [`claim`](AsyncRegistry::claim) (a native selection
//! carrying the eligibility invariant) and [`apply`](AsyncRegistry::apply) (the transition port) —
//! plus a [`lease_millis`](AsyncRegistry::lease_millis) accessor. The four transition operations
//! (heartbeat, fulfill, breach, release) are default methods that call `apply` with the
//! corresponding [`lifecycle`] decision, so their semantics come from
//! the shared kernel.
//!
//! # The transition port
//!
//! [`apply`](AsyncRegistry::apply) runs a pure kernel decision — a
//! `Fn(&State) -> Result<State, NotCurrentHolder>`, i.e. a `lifecycle::on_X` — within the backend's
//! *own atomic scope*. The backend loads the held state, computes the next state through the
//! decision, and applies it atomically; it never decides the transition itself. Crucially, the
//! backend owns *how* the scope is atomic: a transaction, a lock, a native conditional write, or
//! compare-and-set. The contract fixes the decision, not the concurrency-control mechanism.
//!
//! A backend whose only atomic primitive is compare-and-set can satisfy `apply` by delegating to
//! [`apply_via_cas`], which runs the `load → decide → set-if-unchanged` retry loop once.
//!
//! # Both halves of the contract
//!
//! This binding does not re-specify lifecycle behavior — it references the governed truth in
//! [`pacta_contract::lifecycle`] and `pacta-conformance`. But an implementer and a consumer each
//! owe the contract obligations, made explicit here so reading only this crate shows both halves.
//!
//! ## What a backend must satisfy (implementer half)
//!
//! - **`apply` must be atomic.** It must load, decide, and store within one indivisible step
//!   (a transaction, a held lock, or a conditional/compare-and-set write). If the load-to-store
//!   window is not atomic, two workers can both observe the same state and both write — and
//!   exactly-once and retainer fencing break silently. This is the load-bearing obligation of the
//!   transition port.
//! - **`claim` must honor the eligibility invariant and rotate the retainer.** It must select
//!   only a pact [`lifecycle::is_claimable`] would admit (available, a lapsed hold, or a
//!   deferred pact past its instant — never a settled one) and mint a fresh retainer, all
//!   atomically. It must be a **native, full-scan-free** selection (e.g. SQL `SKIP LOCKED`); it
//!   must not load the whole docket to filter in memory. Eligibility is re-expressed natively
//!   per backend, so `pacta-conformance` is the executable proof it matches the invariant.
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
//! - **Runtime and its coloring are yours.** This binding is deliberately `Send`-agnostic: it forces
//!   no `Send` bound on its futures and pulls no runtime. Async, `Send`, and executor choice are the
//!   consumer's to compose — a multi-threaded executor requires `Send`, which the consumer satisfies
//!   at its own call site over a concrete backend. The contract carries no runtime property.
//!
//! Note the fence rule this binding inherits from the frozen contract: a holder whose lease has
//! lapsed but whose pact **no one has reclaimed** can still settle (its retainer is still the
//! current holder) — reclaim, not expiry, rotates authority.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
// The async binding is deliberately `Send`-agnostic: the runtime is the consumer's, so its futures
// carry no `Send` bound and a consumer composes `Send` at its concrete call site, where its executor
// needs it. Native `async fn` in traits (AFIT) is what expresses that — `#[async_trait]` would force
// `Box<dyn Future + Send>`, i.e. pacta dictating a runtime property. This `allow` is that design
// declaration, not a workaround.
#![allow(async_fn_in_trait)]

use core::future::Future;

use pacta_contract::lifecycle::{self, State};
use pacta_contract::{Claim, Retainer, Timestamp};

/// The transition port's decision type, shared with the sync binding so the port is one shape.
/// A pure kernel decision (a `lifecycle::on_X`), `Send + Sync` so [`apply`](AsyncRegistry::apply)'s
/// future stays `Send` across `.await`.
pub use pacta_contract::Transition;

/// An asynchronous [`pacta_contract::Registry`]: the same frozen five-op contract, made async.
///
/// Backends implement the two primitives ([`claim`](Self::claim), [`apply`](Self::apply)) and the
/// [`lease_millis`](Self::lease_millis) accessor; the four transition operations are provided as
/// default methods composing over [`pacta_contract::lifecycle`] through [`apply`](Self::apply).
pub trait AsyncRegistry: Send + Sync {
    /// Error returned by the backend. It must be able to represent a lost/absent authority, so
    /// the shared kernel's [`lifecycle::NotCurrentHolder`] converts into it.
    type Error: std::error::Error + Send + Sync + 'static + From<lifecycle::NotCurrentHolder>;

    // --- required primitives ---

    /// Claim an eligible pact from one of `dockets` at `now`, rotating the retainer. This is the
    /// selection primitive: a backend performs it natively (a full-scan-free selection, e.g. SQL
    /// `SKIP LOCKED`), because selection cannot be built from the transition port.
    ///
    /// **Obligation:** select only a pact [`lifecycle::is_claimable`] would admit, and mint a
    /// fresh retainer, atomically. Eligibility is re-expressed natively per backend, so
    /// `pacta-conformance` is the executable proof it matches the invariant.
    async fn claim(&self, dockets: &[&str], now: Timestamp) -> Result<Option<Claim>, Self::Error>;

    /// The backend's lease duration in milliseconds — used by the default
    /// [`heartbeat`](Self::heartbeat) to compute the extended lease while keeping the faithful
    /// `heartbeat(retainer, now)` signature (the sync contract takes no lease parameter).
    fn lease_millis(&self) -> u64;

    /// Apply a lifecycle transition to the pact held by `retainer`, within the backend's own
    /// atomic scope. `transition` is a pure kernel decision (a [`lifecycle`] `on_X`): the backend
    /// loads the held state, computes the next state through `transition`, and applies it
    /// atomically — never deciding the transition itself, so the lifecycle semantics stay
    /// single-sourced in the kernel and the sync and async bindings cannot drift. A transition
    /// applied against a pact the retainer no longer holds resolves to a not-current-holder error.
    ///
    /// **Obligation:** the load-to-store must be atomic (a transaction, a held lock, or a
    /// conditional/compare-and-set write); otherwise two workers can both write and exactly-once
    /// and retainer fencing break silently. The backend owns *which* concurrency-control mechanism
    /// it uses; a compare-and-set-only backend may delegate to [`apply_via_cas`].
    async fn apply(
        &self,
        retainer: &Retainer,
        transition: &Transition<'_>,
    ) -> Result<(), Self::Error>;

    // --- default transition methods: apply the corresponding kernel decision ---

    /// Extend the lease of the pact held by `retainer` to `now + lease`, provided the retainer
    /// currently holds it and the lease has not lapsed. Composes over [`lifecycle::on_heartbeat`].
    async fn heartbeat(&self, retainer: &Retainer, now: Timestamp) -> Result<(), Self::Error> {
        let lease = self.lease_millis();
        self.apply(retainer, &|state| {
            lifecycle::on_heartbeat(state, retainer, now, lease)
        })
        .await
    }

    /// Conclude the pact held by `retainer` (fulfilled). Composes over [`lifecycle::on_settle`].
    async fn fulfill(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.apply(retainer, &|state| lifecycle::on_settle(state, retainer))
            .await
    }

    /// Conclude the pact held by `retainer` (breached). Shares the settlement transition with
    /// [`fulfill`](Self::fulfill) — the lifecycle state records that the obligation concluded,
    /// not which outcome concluded it.
    async fn breach(&self, retainer: &Retainer) -> Result<(), Self::Error> {
        self.apply(retainer, &|state| lifecycle::on_settle(state, retainer))
            .await
    }

    /// Relinquish the claim held by `retainer` non-terminally, making the pact reclaimable at or
    /// after `reclaimable_at`. Composes over [`lifecycle::on_release`].
    async fn release(
        &self,
        retainer: &Retainer,
        reclaimable_at: Timestamp,
    ) -> Result<(), Self::Error> {
        self.apply(retainer, &|state| {
            lifecycle::on_release(state, retainer, reclaimable_at)
        })
        .await
    }
}

/// Run the [`apply`](AsyncRegistry::apply) transition port as a `load → decide → set-if-unchanged`
/// retry loop, for a backend whose only atomic primitive is compare-and-set.
///
/// A backend with a transaction or a lock implements `apply` with that native atomic scope and does
/// not need this. A compare-and-set-only backend (a conditional-put KV, etc.) satisfies `apply` by
/// delegating here, so the retry loop is written once rather than per backend:
///
/// - `load` returns the current [`State`] of the pact held by the retainer, or `None`.
/// - `cas` atomically sets the state from `expected` to `next` iff it still equals `expected`,
///   returning whether it applied. A `false` means the state changed under the caller (contention,
///   or a lapse and reclaim), so the loop reloads and re-decides.
///
/// A lost authority — `load` returns `None`, or the reload no longer satisfies the transition —
/// resolves to the backend error via `From<`[`lifecycle::NotCurrentHolder`]`>`.
pub async fn apply_via_cas<E, L, C, LFut, CFut>(
    load: L,
    cas: C,
    transition: &Transition<'_>,
) -> Result<(), E>
where
    E: From<lifecycle::NotCurrentHolder>,
    L: Fn() -> LFut,
    C: Fn(State, State) -> CFut,
    LFut: Future<Output = Result<Option<State>, E>>,
    CFut: Future<Output = Result<bool, E>>,
{
    loop {
        let current = load().await?.ok_or(lifecycle::NotCurrentHolder)?;
        let next = transition(&current)?;
        if cas(current, next).await? {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pacta_contract::Pact;
    use std::sync::Mutex;
    use uuid::Uuid;

    /// A trivial in-memory async backend implementing only the two primitives, to prove the five
    /// ops emerge through the defaults. Not a reference backend — that is `pacta-memory-async`.
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

        fn lease_millis(&self) -> u64 {
            self.lease_millis
        }

        async fn apply(
            &self,
            _retainer: &Retainer,
            transition: &Transition<'_>,
        ) -> Result<(), NotHeld> {
            // One `Mutex` scope is the atomic boundary: load, decide, and store without releasing
            // the lock. The transition carries the authority check, so the first record it accepts
            // is the held pact.
            let mut records = self.records.lock().unwrap();
            for (_, state) in records.iter_mut() {
                if let Ok(next) = transition(state) {
                    *state = next;
                    return Ok(());
                }
            }
            Err(NotHeld)
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
        // fulfill is a default method built on apply.
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
        // The prior holder can no longer settle — authority rotated (apply finds it no longer held).
        assert_eq!(reg.fulfill(&first.retainer).await, Err(NotHeld));
        reg.fulfill(&second.retainer)
            .await
            .expect("new holder settles");
    }

    /// The optional compare-and-set helper drives the same five ops for a backend whose only
    /// atomic primitive is set-if-unchanged.
    #[tokio::test]
    async fn apply_via_cas_applies_once_and_rejects_a_stranger() {
        let cell = Mutex::new(Some(State::Held {
            retainer: Retainer::new(Uuid::new_v4()),
            expiry: Timestamp::from_millis(1000),
        }));
        let holder = match cell.lock().unwrap().clone().unwrap() {
            State::Held { retainer, .. } => retainer,
            _ => unreachable!(),
        };

        let cell_ref = &cell;
        let load = || async move { Ok::<_, NotHeld>(cell_ref.lock().unwrap().clone()) };
        let cas = |expected: State, next: State| async move {
            let mut slot = cell_ref.lock().unwrap();
            if slot.as_ref() == Some(&expected) {
                *slot = Some(next);
                Ok::<_, NotHeld>(true)
            } else {
                Ok(false)
            }
        };

        // The holder settles through the helper.
        apply_via_cas(load, cas, &|s| lifecycle::on_settle(s, &holder))
            .await
            .expect("holder settles");
        assert_eq!(*cell.lock().unwrap(), Some(State::Settled));

        // A stranger is rejected: the transition never succeeds against the settled state.
        let stranger = Retainer::new(Uuid::new_v4());
        assert_eq!(
            apply_via_cas(load, cas, &|s| lifecycle::on_settle(s, &stranger)).await,
            Err(NotHeld)
        );
    }
}
