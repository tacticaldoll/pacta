## Context

The current workspace has four crates: `pacta-contract`, `pacta-executor`,
`pacta-driver`, and `pacta-governance`. Tianheng already governs workspace
dependency direction, but the present rules do not prevent a core crate from
adding a normal dependency on a framework such as Tower. The specs say Tower is
an adapter target, yet that boundary is not fully executable.

The contract crate also uses `serde` and `uuid` while several docs call it
`zero-dependency`. The intended invariant is stricter and more useful:
`pacta-contract` is isolated from other workspace crates and has only a small
declared external dependency set.

## Goals / Non-Goals

**Goals:**

- Define Pacta-native composition as the core design center.
- Keep Tower, HTTP, backend, and adapter vocabulary out of core runtime crates.
- Make dependency leakage executable through Tianheng closed allowlists.
- Clarify contract isolation wording so docs and code point future work in the
  same direction.
- Clarify executor infrastructure error handling before larger middleware work.

**Non-Goals:**

- Do not add Tower or a Tower adapter.
- Do not add HTTP request/response types.
- Do not add registry backends, retry policies, or a complete middleware stack.
- Do not split `pacta-governance` into a separate constitution crate in this
  change.

## Decisions

1. Core crate dependencies are closed by name.

   Use Tianheng `restrict_dependencies_to` for normal dependencies instead of
   only `restrict_workspace_dependencies_to`. This lets the constitution allow
   the dependencies each core crate currently needs and reject accidental
   framework, backend, or adapter dependencies. It also keeps the rule
   understandable: adding a normal dependency to a core crate is an architectural
   amendment, not a casual implementation detail.

   Alternative considered: keep workspace-only rules and add prose. That leaves
   the main leakage path unenforced.

2. Pacta-native composition remains vocabulary-first until real middleware
   behavior exists.

   This change defines the boundary and terms for middleware and policies, but
   does not add a broad composition API yet. The existing `Executor` remains the
   public execution role. Later middleware work can add concrete abstractions
   under the new boundary.

   Alternative considered: immediately introduce a Tower-like `Service`
   abstraction. That would answer the wrong pressure by importing a foreign
   shape before Pacta's own execution contract is mature.

3. Adapter support is user/integration scope.

   Tower compatibility belongs in a future adapter-owned crate, not in
   `pacta-contract`, `pacta-executor`, or `pacta-driver`. When an adapter crate
   exists, Hunyi semantic boundaries can be added to make sure adapter-owned
   types are not exposed by core APIs.

   Alternative considered: create `pacta-tower` now only to reserve the
   boundary. That adds a crate with no current runtime value.

4. Executor infrastructure errors must be surfaced.

   A deliberate `Outcome::Breached` is a lifecycle decision. An executor
   infrastructure error is not the same thing; the driver should attempt to
   breach the claim but must return an executor error to callers. This preserves
   the current lifecycle safety while avoiding silent observability loss.

   Alternative considered: keep mapping every executor error to
   `Step::Breached`. That is simple but makes infrastructure failure
   indistinguishable from an intentional breach.

## Risks / Trade-offs

- Closed dependency allowlists can require explicit amendments for ordinary
  library additions. Mitigation: that friction is intentional for core crates;
  integration crates can have looser boundaries later.
- Surfacing executor errors changes the driver API. Mitigation: the runtime is
  still a skeleton with no downstream stability promise in this repository.
- Hunyi semantic boundaries are deferred until there is an adapter surface to
  govern. Mitigation: dependency allowlists prevent the current concrete leak,
  and the spec records when semantic boundaries become mandatory.
