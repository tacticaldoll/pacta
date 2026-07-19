## 1. Dependency bump

- [x] 1.1 In root `Cargo.toml`, change `tianheng = "0.1.6"` to `tianheng = "0.1.8"`; run a build so cargo fetches 0.1.8 and rewrites `Cargo.lock` (0.1.7 → 0.1.8 + sub-crates), and commit the updated `Cargo.lock` with the change

## 2. Adopt native rules

- [x] 2.1 In `constitution()`, add a `ModuleBoundary::in_crate("pacta-contract").module("crate").must_not_call_inline("std::time").ending_with(["now"]).because(...)` boundary (covers `SystemTime::now` and `Instant::now` across the whole core)
- [x] 2.2 Add a second `must_not_call_inline("uuid").ending_with(["now_v7", "now_v1"])` boundary on `pacta-contract` module `crate`
- [x] 2.3 Add `.including_submodules()` to the existing kernel `AsyncExposureBoundary` (before `.because(...)`)

## 3. Remove the bespoke scan

- [x] 3.1 Delete `check_no_ambient_time`, `check_source_content`, `AMBIENT_TIME_MARKERS`, `CORE_SOURCE_DIR`, and the call to `check_no_ambient_time` in `main()`
- [x] 3.2 Delete the ambient-time tests (`current_core_reads_no_ambient_time`, `ambient_time_reads_are_rejected`); keep `SourceViolation` and `collect_rs_files` (the facade scan uses them)
- [x] 3.3 Repurpose or update `AMBIENT_TIME_REASON` as the `.because(...)` justification for the new clock boundaries; add a `uuid`-specific reason if clearer

## 4. Verify coverage is preserved and stronger

- [x] 4.1 Confirm the DoD `check` passes on the clean core
- [x] 4.2 Manually confirm teeth + whole-core scope + the documented trade, then revert each probe: (a) plant an ambient `now()` read in a NON-kernel core file → reaction fails (whole-core scope); (b) plant an aliased `use std::time::SystemTime as Clock; Clock::now()` → caught (alias resolution, the improvement); (c) plant a fully-qualified `uuid::Uuid::now_v7()` with NO `use uuid::...` import → confirm it is NOT caught (the documented non-observation), so the trade is known rather than assumed

## 5. Spec sync

- [x] 5.1 The `quality-governance` delta modifies "Core Reads No Ambient Time" (native, alias-resolving) and "Kernel Async-Exposure Reaction" (subtree-wide)

## 6. Definition of Done

- [x] 6.1 `cargo build --workspace` and `cargo test --workspace`
- [x] 6.2 `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check`
- [x] 6.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- [x] 6.4 `cargo run -p pacta-governance -- check --manifest-path Cargo.toml` (clock + async + facade + prose + coverage all green)
- [x] 6.5 `cargo deny check` (tianheng 0.1.8 passes supply-chain policy)
- [x] 6.6 `openspec validate adopt-native-sans-io-governance --strict`
