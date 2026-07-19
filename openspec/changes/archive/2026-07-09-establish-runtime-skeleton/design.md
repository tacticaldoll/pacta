## Context

Pacta currently has an isolated contract crate, a canonical contract-domain
glossary, and CI/governance gates. The next step is a runtime skeleton, but the
first shape must not accidentally make Tower the public identity of the system.
Pacta should remain middleware-oriented while exposing Pacta-native concepts.

Worklane is useful as a warning: it already contains runtime, backends,
scheduling, conformance, metrics, and adapters. Pacta needs the thinner first
move: branded vocabulary, executor abstraction, and a mechanical driver loop.

## Goals / Non-Goals

**Goals:**

- Govern Pacta's runtime vocabulary before adding public runtime crates.
- Reposition the project from Tower-native to Pacta-native and
  middleware-oriented.
- Add `pacta-executor` as the Pacta-native execution abstraction.
- Add `pacta-driver` as the mechanical loop that composes `Registry` and
  executor.
- Keep new crates covered by Tianheng governance and existing CI gates.

**Non-Goals:**

- Add registry backends or persistence.
- Add retry, timeout, scheduling, rate-limiting, or Tribunal behavior.
- Add a conformance suite.
- Add a Tower adapter or expose Tower terms from core runtime APIs.

## Decisions

1. Make Pacta-native vocabulary the public runtime identity.

   Public runtime APIs should use `Executor`, `Execution`, `Outcome`, and
   `Settlement` where those concepts appear. Mechanical terms such as `Driver`
   remain allowed for the polling loop because they describe implementation
   machinery clearly.

2. Treat Tower as an adapter target, not the core abstraction.

   Tower remains valuable, but exposing `tower::Service` as Pacta's primary
   executor would make a framework's vocabulary define Pacta's domain. The core
   skeleton should therefore define a Pacta executor trait first. A later
   `pacta-tower` adapter can bridge Tower services into that trait.

3. Keep `pacta-contract` isolated from runtime crates.

   The contract crate remains the zero-dependency data and lifecycle contract.
   Runtime crates may depend on it; it must never depend back on runtime crates.

4. Split executor and driver crates.

   `pacta-executor` owns execution abstractions and result vocabulary.
   `pacta-driver` owns the loop that claims, executes, and settles. This keeps
   middleware-facing API independent from loop mechanics.

5. Keep the first driver behavior narrow.

   The skeleton only needs to prove the settlement path: successful execution
   calls `fulfill`; failed execution calls `breach`. Polling policy, shutdown,
   retry, and backoff are deferred.

## Risks / Trade-offs

- **Risk:** Pacta-native names can become theatrical and obscure.
  **Mitigation:** Use branded terms only for domain concepts; keep clear
  engineering terms such as middleware and driver where clarity wins.
- **Risk:** Avoiding Tower in core could delay ecosystem integration.
  **Mitigation:** Preserve Tower as an explicit adapter target for a later,
  separately scoped change.
- **Risk:** Splitting executor and driver crates may feel early.
  **Mitigation:** The split is small and protects the contract between
  middleware-facing execution and loop mechanics.

## Migration Plan

1. Update domain-language and project positioning.
2. Add executor and driver skeleton crates.
3. Update workspace and Tianheng governance boundaries.
4. Update docs and roadmap.
5. Run all quality gates.
