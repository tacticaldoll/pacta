# pacta-conformance

A backend-agnostic conformance suite for Pacta `Registry` implementations.

Any `Registry` backend — in-memory, SQLite, Postgres, or your own — must pass the
same lease-lifecycle contract: claim, settlement, lapse, heartbeat, and the
at-least-once safety property that a reclaimed pact rejects the prior holder's
authority. The suite is generic over `Registry` and takes a seeding closure, so a
backend runs it from its own test module; time is driven through the trait by
passing controlled `Timestamp` values, never a wall clock.

Use it as a dev-dependency to prove a backend correct.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
