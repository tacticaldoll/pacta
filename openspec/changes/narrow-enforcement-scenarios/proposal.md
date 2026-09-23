# Proposal

## Why

Several spec requirements and scenarios say a governance reaction or a test catches shapes it does
not observe. The accepted boundary reasons in `AGENTS.pacta-law.md`, Tianheng's observation bounds,
and the executor scan's own code say what each boundary sees: the dependency boundaries read the
normal dependency table; the ambient-time boundaries see an inline `std::time` `now` call and an
inline `uuid` `now_v7` or `now_v1` call, not a clock read through a method on a value or another
crate's re-export; the no-I/O boundaries see an inline call into a `std::io`, `std::fs`, `std::net`,
or `std::process` path, not a call through a method on a value; the async-exposure boundaries see a
declared public `async fn`, not a macro-generated item or a body-nested module; the impl-trait
boundaries cover each kernel module itself, not its descendants; the no-serde boundary sees a
written derive or a hand-written impl whose self type it resolves; the facade kernel-exclusion sees
a written `pacta_contract::kernel` path in a re-export or a public signature; and the executor
vocabulary scan judges only a line that begins with `pub ` and defines a named item, not `pub use`,
`pub const`, a trait method declaration, or a `pub` field. The contention checks' teeth are proven
deterministically only by the barrier-synchronized fixtures; against an arbitrary backend the
repeated rounds are a probabilistic stress that guarantees no catch. The specs still speak of "a
forbidden dependency", "ambient current-time reads", "renamed or re-exported import", "synchronous
I/O anywhere", "the public API of the kernels", "any item of the kernel module", and a harness that
"actually catches a non-atomic backend", so they claim more than the gate or the suite proves.

## What Changes

- `quality-governance`:
  - "Tianheng Governance Reaction": the body names the normal dependency table as observed; the
    contract-isolation scenario names a normal dependency outside the allowlist.
  - "Kernel Async-Exposure Reaction": the body names what the reaction observes and what is
    review-governed; the scenarios say "declares".
  - "Core Reads No Ambient Time": the body and scenarios name the inline `std::time` `now` and
    `uuid` `now_v7`/`now_v1` calls and a renamed `use` import, and list the unobserved remainder
    as review-governed.
  - "Kernel Is Not Serializable": the body and scenario name a written derive or a hand-written
    impl whose self type resolves; a macro-generated or unresolvable impl is review-governed.
  - "The Core Contract Performs No Synchronous I/O": the body and scenario name an inline call into
    one of the four paths, and the partiality list adds a call through a method on a value.
  - "Executor Orchestration-Vocabulary Reaction": the failing scenario names a public item
    definition, and the body's partiality list names every shape the line scan skips: `pub use`
    re-exports, `pub const` values, trait method declarations, and `pub` fields.
  - "Colorless Kernel Exposes No Return-Position Existentials": the body and scenario name a
    written RPIT on a public fn or method in each kernel module itself, and name a descendant
    module, a boxed `dyn Future`, and a macro-generated item as review-governed.
- `public-facade` "Facade Excludes The Kernel": the body names a written `pacta_contract::kernel`
  path in a re-export or a public signature as observed, and the scenario names a re-export
  written through that path; other routes are review-governed.
- `lifecycle-persistence` "Injected Time" requirement, scenario "Core reads no ambient time": the
  governance check is described as rejecting the inline clock calls it can observe, leaving other
  clock reads to review.
- `registry-conformance` "The Contention Harness Is Proven Non-Vacuous": the harness "can catch" a
  non-atomic backend; the body says what the contention run exercises (the contended settlement
  and claim, repeated for a fixed number of rounds on real concurrent workers), that this
  repetition guarantees no catch against an arbitrary backend, and that the teeth are proven only
  by the deterministic fixtures.

The product intent in each requirement stays as written.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-governance`: seven requirements narrow what their bodies and scenarios say the
  governance check catches.
- `public-facade`: one requirement narrows its kernel-exclusion enforcement claim.
- `lifecycle-persistence`: one scenario narrows its governance-check claim.
- `registry-conformance`: one requirement narrows what the contention harness is said to catch.

## Impact

- The four specs above in `openspec/specs/` after sync.
- No code or law change: the constitution, its reasons, `AGENTS.pacta-law.md`, the reaction
  tests, and `list --format json` stay as they are.
