## Why

Pacta already has a minimal contract crate, backlog, and project contract, but
its public language still mixes the Pacta brand with generic queue terms such as
lane, payload, store, reservation, reserve, ack, and nack. Establishing the
contract/arbitration vocabulary now prevents the first public API from carrying
the old factory and traffic metaphors forward.

## What Changes

- **BREAKING**: Rename public `pacta-contract` model and trait terms from the
  current Store/Reservation/ack/nack vocabulary to Pacta's contract-domain
  vocabulary.
- Define the canonical mapping for existing concepts:
  - `Store` -> `Registry`
  - `lane` / lane selection -> `Docket`
  - `payload` -> `Clause`
  - operational envelope fields -> `Brief`
  - `reserve` -> `claim`
  - `Reservation` -> `Claim`
  - `ReservationReceipt` / lease token -> `Retainer`
  - `ack` -> `fulfill`
  - `nack` -> `breach`
  - public `Handler` role -> `Executor`
  - `Driver` remains an internal runtime-loop term unless a public API exposes it
  - expired lease recovery -> `lapse`
  - dead-letter / terminal review -> `Tribunal`
- Keep the foundation axiom intact: storage remains a pure lifecycle state
  machine and execution orchestration remains middleware.
- Update `README.md`, `PROJECT.md`, `BACKLOG.md`, ADRs, and crate documentation
  so the release branch consistently reflects the new public language.
- Do not add new runtime behavior beyond the naming migration.

## Capabilities

### New Capabilities

- `domain-language`: Defines Pacta's canonical public vocabulary and the boundary
  between domain terms and private implementation mechanics.

### Modified Capabilities

- None. There are no existing living requirement specs to modify.

## Impact

- Affects public Rust API names in `crates/pacta-contract`.
- Affects project documentation: `README.md`, `PROJECT.md`, `BACKLOG.md`, and a
  new ADR for the naming decision.
- Affects future work planning for `pacta-driver`, conformance, backends, and
  terminal review.
- Does not change dependency policy, middleware orchestration ownership, or
  storage purity.
