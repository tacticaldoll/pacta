# pacta-conformance

A backend-agnostic conformance suite for Pacta `Registry` implementations.

Any `Registry` backend — in-memory, SQLite, Postgres, or your own — must pass the
same lease-lifecycle contract: claim, settlement, lapse, heartbeat (including the
`now == expiry` boundary), deferred reclaim on release, and the at-least-once safety
property that a reclaimed pact rejects the prior holder's authority. The suite is
generic over `Registry` and takes a seeding closure, so a backend runs it from its own
test module; time is driven through the trait by passing controlled `Timestamp` values,
never a wall clock.

Beyond the sequential lifecycle scenarios it verifies **atomic authority under real
concurrency** through the public trait only (never by inspecting a backend's locking):

- `run_contention` — for a sync `Registry`: two workers race a settlement on one claimed
  pact (exactly one succeeds) and two workers race a claim on one available pact (exactly
  one gets it).
- `run_async_contention` — the same at-most-once claim and settlement checks for a
  ready-future `AsyncRegistry`, driven by OS threads with no async runtime.

For the **async binding** the suite runs the one shared scenario set two ways, so coverage
cannot drift and a real backend never re-declares scenarios:

- `run_async_with(make, driver)` — the runtime-compatible entry: a real-reactor backend
  drives the scenarios on its **own** runtime via a small `BlockingDriver` (for example one
  wrapping `tokio::runtime::Runtime::block_on`). It imposes no `Send` bound on the backend's
  futures and pulls no async runtime into this crate.
- `run_async` — a convenience for **ready-future** backends, using the built-in
  `SelfProgress` driver. It is not correct for a backend whose futures park pending real
  I/O or a timer; such a backend uses `run_async_with`.

The suite proves *behavioral* equivalence to the shared lifecycle (eligibility, transitions,
lapse/reclaim, at-most-once contention). It does **not** prove a backend's *query shape* —
that claim selection is full-scan-free is a separate obligation established by review, since
a sequential functional suite cannot observe query cost.

Use it as a dev-dependency to prove a backend correct.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
