## Why

The governance gate's two file-scanning checks — `check_active_prose` and
`check_facade_reexports_only` — silently `continue` when a governed file cannot be
read (`let Ok(content) = fs::read_to_string(&path) else { continue; }`). For a tool
whose whole job is to fail loudly, a canonical governed file that vanishes or becomes
unreadable makes that check pass vacuously. The coverage check already guards its
analogous case (`coverage.total > 0`); these two scans should have the same
no-vacuous-pass parity. `pacta-governance` is unpublished, so this carries no
public-API/SemVer consequence — it is pure internal hardening.

## What Changes

- `check_active_prose`: an unreadable or absent file in the canonical
  `ACTIVE_PROSE_FILES` list SHALL fail the gate with a clear message identifying the
  file, instead of being silently skipped.
- `check_facade_reexports_only`: an unreadable facade source file SHALL fail the gate,
  and the scan SHALL fail if it finds no facade source files at all (an empty or
  missing facade source tree), mirroring the `coverage.total > 0` guard.
- Reaction tests prove each new failure path fires (a missing governed input fails)
  and that the real workspace stays clean.

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `quality-governance`: the executable gate fails loudly on an unreadable or absent
  governed input rather than skipping it, so no file-scanning check passes vacuously.

## Impact

- **Code:** `crates/pacta-governance/src/main.rs` (unpublished crate) — the two scan
  functions and their tests. No change to any published crate or the public API.
- **Behavior:** governance now fails (exit 1) if a canonical governed file is missing
  or unreadable, or if the facade source tree yields no files. Normal operation
  (all files present) is unaffected.
- **Not in scope:** the frozen 0.1.0 public surface (decoupled from publish); and the
  narrow residual that `collect_rs_files` uses `entries.flatten()`, which silently
  drops an individual `DirEntry` that errors mid-iteration — a rare per-entry I/O
  fault that leaves `scanned > 0` intact. The primary vacuous-pass holes (missing
  file, unreadable file, empty tree) are closed; hardening `collect_rs_files` to
  propagate per-entry errors would ripple a `Result` through its callers and is left
  for a future change if that tail is ever judged worth it.
