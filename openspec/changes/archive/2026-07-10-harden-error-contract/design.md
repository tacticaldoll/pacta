## Context

A release-readiness gap sweep found that Pacta's public trait errors are not
library-grade: `Registry::Error` and `Executor::Error` are unbounded, and neither
`DriverError` nor the shipped backend error `NotHeld` implements `Display` or
`std::error::Error`. Consumers cannot print, chain, or `?`-convert these errors.
This is the first of a short sequence of release-readiness changes and lands first
because the facade to follow will re-export these error types.

## Goals / Non-Goals

**Goals:**
- Make every public error a standard error: bound the associated error types and
  implement `Display` + `std::error::Error` on the concrete ones.
- Preserve the source chain so a driver error reveals the underlying cause.

**Non-Goals:**
- No new error taxonomy, no error enum redesign, no `thiserror`/dependency.
- No change to lifecycle behavior or to how errors are produced.

## Decisions

### Decision: Bind the associated error types to `std::error::Error`
`Registry::Error: std::error::Error` and `Executor::Error: std::error::Error`.

Rationale: an unbounded associated error is the thinnest possible declaration, but
it makes the whole stack unusable as a library — `DriverError` cannot implement
`Display`/`Error` if its inner errors are unbounded, so the papercut propagates
outward. `std::error::Error` (which requires `Debug + Display`) is the conventional
Rust contract for a public error and costs an implementor only a trivial `Display`
impl. This is courtesy, not behavior, so it does not thicken the lifecycle.

Alternative considered: bind only to `Debug + Display`. Rejected — `std::error::Error`
adds source chaining for negligible extra cost and is the idiomatic bound.

Alternative considered: leave unbounded and impl `Display`/`Error` only on the
concrete types. Rejected — `DriverError<R, E>` cannot meaningfully implement
`Display`/`Error` without its inner errors being displayable, so the bound is what
makes the concrete impls possible.

### Decision: Implement `Error` with a real source chain on `DriverError`
`DriverError` implements `Display` and `std::error::Error`, returning the wrapped
registry or executor error from `source()`.

Rationale: the value of a standard error is the chain; a driver error should let a
caller reach the underlying cause, not just say "registry error."

### Decision: Test and example errors satisfy the bound
In-crate test errors that used `()` become a small real error type; the executor's
test `DummyError` gains the impls; the composition example already uses
`std::convert::Infallible`, which implements `std::error::Error`, so it is
unchanged.

Rationale: keep the change self-consistent — everything that implements the traits
must satisfy the new bound within this change.

## Risks / Trade-offs

- [Breaking change for downstream implementors] → Real but small and correct: an
  error type that is not a standard error is exactly what this fixes; migration is
  a trivial `Display` impl or switching a `()` to a real type. Documented as
  BREAKING in the proposal.
- [`std::error::Error` ties the contract to `std`] → Acceptable: the workspace is
  already `std` (uses `String`, `Vec`); no `no_std` goal exists. `std::error::Error`
  is the same trait as `core::error::Error` since it was stabilized there.

## Open Questions

- None. The bound and impls are mechanical once the decision to bind is taken.
