## Context

Pacta is a durable primitive thin enough that only the lifecycle contract lives in
the core; everything else is composed by the user, in the Tower middleware lineage.
0.1.0 is about to publish to crates.io, which freezes the public API. This is the
last moment at which enum exhaustiveness, struct field visibility, and type presence
can change without a major version. An adversarial pre-publish audit found several
irreversible traps and one prose-only vision invariant that the Tianheng family can
now enforce. This change is the single foundation-hardening pass before publish.

The guiding filter is **irreversibility**: do now only what is breaking-to-fix-later
or is a permanent invariant worth a governance tooth. Additive changes (new derives,
new `#[must_use]`, `serde` presence) are deliberately left for post-1.0, because
doing them now buys nothing and churns a working, governed surface.

## Goals / Non-Goals

**Goals:**
- Give every public enum a deliberate exhaustiveness stance and every public struct
  a deliberate extensibility stance, chosen by the type's role, not by accident.
- Shave the one accreted inert type (`Policy`) before it freezes.
- Ratchet two permanent kernel-purity invariants (no serde, no synchronous I/O) that
  currently live only in prose, using the Tianheng family, proven to fire.
- Make the user-obligation delivery pattern explicit and executably proven.

**Non-Goals:**
- Shipping any part of the deferred orchestration cluster: concrete middleware
  (retry/timeout), the `Policy` trait, or a `ServiceBuilder`-style stack assembler.
  Each must co-arrive with the others so it is validated by a real client; shipping
  one now would freeze a contract with no consumer — the very mistake `Policy` is.
- Adding `Send`/`Sync` bounds to `Executor`/`Middleware` for a multi-threaded runtime
  that is itself deferred.
- Any behavioral change to the state machine, the lease lifecycle, or the driver loop.

## Decisions

### Exhaustiveness by role, not uniformly
`#[non_exhaustive]` is applied where a variant set is *expected to grow*, and
withheld where a set is *conceptually complete*. Kernel protocol (`Directive`,
`Notice`, `StepResult`) is advanced-tier and already declared to evolve; `Step`
(driver-loop status, already carrying `Idle` beyond the settlement binary, with
heartbeat/lapse variants foreseen in the backlog) and `DriverError` (the textbook
open error enum) grow. `Outcome` is `Fulfilled | Breached` — a complete settlement
binary that recommended-tier consumers should match exhaustively — so it stays
closed. Alternative rejected: blanket `#[non_exhaustive]` on all enums. That would
force a wildcard arm on `Outcome`, degrading the recommended surface to freeze a set
that will not grow — symmetry for its own sake, against the thin vision.

### Extensible records via `#[non_exhaustive]` + constructors
`Pact` and `Claim` are durable records with all-public fields; frozen as-is they can
never gain a field (priority, metadata, attempt count — all live backlog topics)
without a major version. They gain `#[non_exhaustive]` plus `Pact::new`/`Claim::new`.
Fields stay public for reading; construction moves to the constructor (the only cost,
borne by backend authors — a sophisticated audience already implementing a trait —
and by the facade doctest). `Execution` is the executor's designated growth seam and
gains `#[non_exhaustive]` (its `new` already exists; downstream reads `.pact`, does
not construct it, so the cost is near zero). Alternative rejected: keep records
closed with public-field literal construction ("a `Pact` is exactly these four
fields; growth is 0.2.0"). Coherent and maximally ergonomic, but it bets that the
most-likely-to-grow types never grow, and this is the only free moment to hedge that
bet. `Retainer`/`Timestamp` are already opaque and need nothing.

### Remove `Policy`; record its correct future form
`Policy` is a public `struct` with a `&'static str` name, no consumer, and no
validating implementation. The workspace discipline — verified against `Registry`,
`Executor`, and `Middleware` — is that every user-obligation type ships with an
in-workspace consumer and a reference impl. `Policy` meets neither. The orchestration
seam users actually compose against is `Middleware` (the Tower `Layer`), which
already exists; removing `Policy` removes no capability. Its correct future form is a
user-implemented trait in the sense of `tower::retry::Policy` — a parameter to a
concrete retry middleware — co-designed with that middleware so its method set is
validated by a real client. That is recorded in `BACKLOG.md`, not shipped now.
Alternative rejected: invert `Policy` into a trait now. A user-implementable trait is
*harder* to freeze than the struct (adding a required method breaks all implementors)
and, with no consumer, would freeze an unvalidated contract.

### Keep `Settlement`
`Settlement` is a type alias of `Outcome` used in zero signatures, which superficially
resembles `Policy`. It differs decisively: `architecture-blueprint` names the core
lifecycle `Signal -> Pact -> Claim -> Execution -> Settlement`, and every other
realized stage (`Pact`, `Claim`, `Execution`) is a type. Removing `Settlement` would
make the terminal stage the only one without a type — an asymmetry that retracts
architectural identity. It is kept as the named terminal stage.

### Ratchet kernel purity with Tianheng — the shape dictates the mechanism
The Tianheng family is prohibition-shaped: it can forbid, not require. So invariants
of the form "X must be present" (an enum carries `#[non_exhaustive]`; a type derives
`Eq`) are not Tianheng-expressible and are proven by other means (compiler truth /
compile-assert tests / `cargo-semver-checks` against the published baseline from
0.1.1 on). Invariants of the form "the kernel must not acquire / must not do X" fit
exactly. Two are added: a hunyi `forbidden_marker` boundary forbidding the
`crate::kernel` subtree from acquiring `Serialize`/`Deserialize` (durable state
serializes; transient driving protocol must not), and a guibiao `must_not_call_inline`
boundary forbidding synchronous `std::io`/`fs`/`net`/`process` anywhere in the
`pacta-contract` core (completing the sans-I/O guarantee, whose async half is already
ratcheted). The no-I/O boundary targets the whole crate (`module("crate")`), like the
sibling ambient-time tooth: the guibiao module rule governs a file-based module (the
`kernel` module is inline and owns no file) and the entire core is sans-I/O, not the
kernel alone — a broader and more correct scope, not a workaround. It runs in default
mode with no `strict_external()`: `std` is a sysroot head caught by default (the flag
exists only to also catch external-crate heads such as `uuid`), exactly as the shipped
`std::time` ambient-time tooth is declared. The
no-serde reaction is proven to fire by a fixture-based reaction test, per the existing
"reactions are executably proven" discipline; the no-I/O reaction relies on the same
`must_not_call_inline` mechanism the `std::time` tooth already exercises. The no-I/O
tooth is acknowledged as inherently partial (I/O heads cannot be enumerated the way
`std::time` can, and macro-expanded I/O is not seen by a source scan) and says so in
its reason string; it is still worth having because
it uses a proven mechanism, catches the common heads, and is permanent.

### Name and prove the delivery pattern
Pacta already embodies the Tower pattern: `Executor` is `Service` narrowed to the
lifecycle (request = `Execution`, response fixed to `Outcome`, sans-I/O sync),
`Middleware<E>` is `Layer`, and the closure property (`Executor -> Executor`, so
middleware stacks arbitrarily) holds — confirmed by the generic-over-`E` impls. The
one Tower piece pacta lacks, a `ServiceBuilder`-style assembler, is deferred with the
rest of the orchestration cluster. Two cheap, non-speculative additions make the
existing pattern legible and durable: a test that stacks two pass-through middleware
(proving the closure property that "compose the rest" rests on, and guarding the
generic shape from regression), and a composition-governance scenario naming the two
obligation-delivery patterns (Service/Layer closure for execution; trait-plus-
conformance for persistence).

## Risks / Trade-offs

- **Large surface for one change (7 spec deltas + contract/executor/driver/facade/
  governance edits).** → The change is internally coherent (one theme: freeze the
  foundation) and each concern is independently verifiable by the adversarial gate;
  splitting would double ritual overhead and fracture a story that should land once.
- **`#[non_exhaustive]` on `Pact`/`Claim` breaks downstream literal construction,
  including the facade doctest.** → Intended and done now while free; constructors are
  added and the doctest is migrated in the same change, so the composition proof still
  compiles and runs.
- **The no-I/O tooth is partial and could give false confidence.** → Its reason
  string states the partiality; it complements, not replaces, review, and matches the
  already-accepted partiality of the ambient-time tooth.
- **Judgment that the orchestration cluster is genuinely deferrable.** → If a consumer
  needs retry today they compose it as a `Middleware` now; nothing about the frozen
  seam blocks the cluster from attaching later. The seam, not the behavior, is what
  0.1.0 must get right.

## Open Questions

None. The remaining orchestration-cluster design (concrete middleware, the `Policy`
trait, the stack assembler) is deliberately deferred and recorded in `BACKLOG.md`.
