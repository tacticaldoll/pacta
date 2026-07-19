## Why

`pacta-governance` currently enforces the core's "no ambient clock" rule with a
bespoke line scan (`check_no_ambient_time`) that matches string markers like
`SystemTime::now`. It is brittle: it silently misses an aliased read such as
`use std::time::SystemTime as Clock; Clock::now()`. tianheng 0.1.8 adds a native
static rule — 圭表 `must_not_call_inline` — that forbids inline calls resolving to a
symbol path while resolving `use` renames, aliases, and re-exports, and a 渾儀
`including_submodules` opt-in that extends the kernel's async-exposure rule across
its whole subtree. Adopting both replaces bespoke code with governed,
alias-resolving rules — the project's "governed teeth over bespoke" ethos — and
readies the kernel's sans-I/O property to hold across any future submodule.

## What Changes

- Bump the `tianheng` workspace dependency from `0.1.6` to `0.1.8`. pacta's lockfile
  currently resolves `tianheng 0.1.7` (the floating `^0.1.6` pin); the bump fetches
  and locks `0.1.8`, whose DSL this change requires. Only `pacta-governance` depends
  on tianheng, so no published crate is affected.
- Replace the bespoke ambient-time scan with two 圭表 `must_not_call_inline`
  boundaries on `pacta-contract` (whole-crate subtree): `std::time` ending with
  `now` (covers `SystemTime::now` and `Instant::now`), and `uuid` ending with
  `now_v7`/`now_v1`. These resolve aliases and re-exports the string scan missed
  (a net improvement on idiomatic forms; see design for two documented
  non-observations the string scan happened to catch).
- Delete `check_no_ambient_time`, `check_source_content`, `AMBIENT_TIME_MARKERS`,
  `CORE_SOURCE_DIR`, and their tests; the native rules subsume them. Keep
  `SourceViolation` and `collect_rs_files` (the facade re-exports scan still uses
  them).
- Add `.including_submodules()` to the kernel async-exposure boundary so the
  sans-I/O property is governed throughout the kernel subtree, not only at its seam.
  The kernel has no non-test submodules today (only a `#[cfg(test)] mod tests`,
  which exposes no async), so this is forward-looking hardening, not a current
  behavior change.

## Capabilities

### Modified Capabilities
- `quality-governance`: the "Core Reads No Ambient Time" reaction is now the native
  tianheng inline-call confinement (alias-resolving) rather than a bespoke string
  scan; the "Kernel Async-Exposure Reaction" now governs the kernel subtree, not
  only its own seam.

## Impact

- `Cargo.toml`: `tianheng = "0.1.6"` → `"0.1.8"`.
- `Cargo.lock`: updated `tianheng 0.1.7` → `0.1.8` (plus its 0.1.8 sub-crates),
  committed with the change.
- `crates/pacta-governance/src/main.rs`: constitution gains two `must_not_call_inline`
  boundaries and the async boundary gains `.including_submodules()`; the bespoke
  ambient-time scan and its tests are removed. Net: fewer bespoke lines, stronger
  governed rules.
- No source or public-API change to any published crate; governance-only.
