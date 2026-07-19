## Context

`pacta-governance` enforces two core-integrity properties with a mix of native and
bespoke mechanisms: the kernel's async-freedom via a hunyi `AsyncExposureBoundary`
(seam-only), and the core's no-ambient-clock rule via a bespoke line scan
(`check_no_ambient_time`) that string-matches markers over `pacta-contract/src`.
tianheng 0.1.8 makes both natively governable and stronger.

## Goals / Non-Goals

**Goals:**
- Replace the bespoke ambient-time scan with native, alias-resolving tianheng rules.
- Extend the kernel async-freedom rule across the kernel subtree.
- Preserve the current coverage on idiomatic forms (whole core; `std::time` +
  `uuid` clock reads; runtime clocks outside the core still allowed), accepting two
  documented non-observations in exchange for alias/re-export resolution.

**Non-Goals:**
- No change to any published crate's source or API (governance-only).
- No adoption of the bundled `SansIoPure` profile (see Decisions).
- No new 0.1.8 rules beyond the two that upgrade existing coverage (UnsafeBoundary,
  max_visibility, confine_external_crate are out of scope here).

## Decisions

### Decision: Adopt the two primitives, not the `SansIoPure` profile
`SansIoPure` folds the clock rule and the async rule onto ONE module with ONE time
prefix. But pacta's ambient-time discipline is broader than the kernel: it covers
the WHOLE core contract (`Timestamp`, `Registry`, etc. live at the crate root, not
under `crate::kernel`) and forbids BOTH `std::time` and `uuid` clock reads. The
async rule, by contrast, is a kernel property. Their scopes differ, so bundling them
onto `crate::kernel` would regress ambient-time coverage. Adopt the underlying
primitives explicitly:
- `must_not_call_inline` on `pacta-contract` module `crate` (whole-crate subtree),
  once for `std::time` ending with `now`, once for `uuid` ending with
  `now_v7`/`now_v1`.
- `must_not_expose_async_fn().including_submodules()` on `crate::kernel`.

### Decision: Two clock boundaries, mirroring the deleted marker set
The bespoke scan matched `SystemTime::now`, `Instant::now`, `Uuid::now_v7`,
`Uuid::now_v1`. `SystemTime` and `Instant` both resolve under `std::time`, so a
single `must_not_call_inline("std::time").ending_with(["now"])` covers both; `uuid`
needs its own boundary because its verbs differ. This preserves coverage exactly
while gaining alias/re-export resolution the string scan lacked.

### Decision: Delete the bespoke scan, keep shared helpers
Remove `check_no_ambient_time`, `check_source_content`, `AMBIENT_TIME_MARKERS`,
`CORE_SOURCE_DIR`, and their tests. Keep `SourceViolation` and `collect_rs_files` —
the facade re-exports-only scan still depends on both.

### Decision: Bump the pin to 0.1.8
pacta's lockfile currently resolves `tianheng 0.1.7` (the `^0.1.6` pin floated to the
newest crate cached at lock time), but the new DSL (`must_not_call_inline`,
`including_submodules`) landed in 0.1.8. Bump `tianheng = "0.1.6"` → `"0.1.8"`; cargo
fetches 0.1.8 from the registry and rewrites `Cargo.lock` (0.1.7 → 0.1.8 plus its
0.1.8 sub-crates), which is committed with the change.

## Risks / Trade-offs

- [`module("crate")` scope wrong] → Verified at apply time: the DoD `check` must
  stay green on the clean core, and a planted `now()` in a non-kernel core file must
  fail — proving whole-crate coverage, not kernel-only.
- [`std::time` verb set misses a future read] → The engine bakes in no default
  verbs; `["now"]` matches today's reads. A future `Instant::elapsed`-style read is
  the adopter's to add — same responsibility the string scan carried.
- [Native rule semantics differ from the scan] → The rule reacts on inline calls
  resolving under the prefix with the terminal verb, resolving `use`
  renames/aliases/re-exports. It is a net superset on IDIOMATIC forms, but has two
  documented non-observations the string scan happened to catch: (a) a fully-
  qualified external-crate call written with NO `use` import (`uuid::Uuid::now_v7()`
  with no `use uuid::...` in scope resolves as a local path and is not matched — note
  `std::time::...::now()` fully-qualified IS caught, as `std` is a recognized head);
  (b) a value-position mention / fn-pointer capture (`let f = Uuid::now_v7;`, no
  call) under the default calls-only mode. Both are acceptable for pacta:
  `pacta-contract` already does `use uuid::Uuid;`, so the idiomatic `Uuid::now_v7()`
  is caught, and the alias-resolution gained is worth more than these edge forms. The
  trade is made explicit by a planted-FQ-uuid negative check at apply time, not left
  silent.

## Migration Plan

Governance-only; no code migration. Rollback is reverting the constitution edits and
restoring the bespoke scan. The actual 0.1.0 release is unaffected in content (no
published crate depends on tianheng).

## Open Questions

None blocking.
