## Why

Pacta's public trait errors are unusable as library errors. `Registry::Error` and
`Executor::Error` are unbounded associated types, and no shipped error type
(`DriverError`, `NotHeld`) implements `Display` or `std::error::Error`. A consumer
cannot `?` a `DriverError` into `Box<dyn Error>` or `anyhow`, cannot print a human
message, and cannot walk the error source chain. For a library people are meant to
compose against, standard-error citizenship is table stakes and does not cost
thinness — the thin part is behavior, not error courtesy.

## What Changes

- Bind `Registry::Error` and `Executor::Error` to `std::error::Error`, so every
  implementation's error is displayable, debuggable, and source-chainable.
- Implement `Display` and `std::error::Error` on `DriverError`, exposing the
  underlying registry or executor error as its `source`.
- Implement `Display` and `std::error::Error` on the shipped backend error
  `NotHeld`.
- Update in-crate and example error types to satisfy the bound (test errors that
  used `()` become a real error type; `Infallible` already qualifies).

## Capabilities

### Modified Capabilities
- `runtime-skeleton`: require that the public runtime trait errors are standard
  errors — the `Registry` and `Executor` associated error types are bound by
  `std::error::Error`, and the driver and shipped backend errors implement
  `Display` and `std::error::Error`.

## Impact

- Code: `pacta-contract` (Registry error bound), `pacta-executor` (Executor error
  bound), `pacta-driver` (`DriverError` implements `Display`/`Error`), and
  `pacta-memory` (`NotHeld` implements `Display`/`Error`).
- **BREAKING**: `Registry::Error` and `Executor::Error` now require
  `std::error::Error`. A downstream implementation whose error type does not
  implement `std::error::Error` (for example a bare `()`) must switch to a real
  error type. `std::convert::Infallible` already qualifies.
- No dependency changes and no new crates.
