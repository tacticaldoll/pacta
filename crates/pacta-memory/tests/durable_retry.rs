//! Wires `examples/durable_retry.rs` into `cargo test`: plain `cargo test --workspace`
//! builds an example but never executes its `main()`, so the demonstration's own
//! `assert!`/`assert_eq!` calls never ran under the Definition of Done. This file adds no
//! logic of its own -- it includes the example verbatim and calls its `main()` from a
//! `#[test]`, so a regression there now fails the gate instead of passing silently.

#[path = "../examples/durable_retry.rs"]
mod durable_retry_example;

#[test]
fn durable_retry_demonstration_runs_without_panicking() {
    durable_retry_example::main();
}
