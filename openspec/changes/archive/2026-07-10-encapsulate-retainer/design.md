## Context

`Retainer(pub Uuid)` contradicts its "opaque proof token" documentation. The bare
public field lets any code construct, mutate, and read the identifier. Authority is
actually registry-validated (a forged identifier does not match an issued claim), so
this is an API-hygiene fix, not a security fix.

## Goals / Non-Goals

**Goals:**
- Encapsulate `Retainer`'s identifier (private field + `new` + `id`).
- Correct the doc so it stops overclaiming type-enforced authority.

**Non-Goals:**
- Encapsulating `Pact`/`Claim` (a broader change).
- Adding a hunyi no-bare-pub reaction — its visibility rule is module-scoped and
  would flag the still-public `Pact`/`Claim` fields; it belongs with the broader
  encapsulation.

## Decisions

1. Keep `Retainer` a tuple struct with a private field; expose `new`/`id`.

   `new` is public because `Registry` is an open trait: any backend implementation
   must be able to mint tokens. `id` is public because a registry must read the
   identifier to validate a settlement. Serde derives are unaffected by field
   privacy, so the wire form is unchanged.

   Alternative considered: a fully opaque identifier type (hiding that it is a
   `Uuid`). Rejected as over-scope — registries need a concrete, comparable id, and
   full representation hiding is a larger decision (audit finding #5).

2. Compiler is the enforcement; the hunyi ratchet is deferred.

   A private field is enforced by the compiler. The hunyi no-bare-pub ratchet is
   module-scoped, so adopting it now would require encapsulating the whole contract;
   that is deferred to a broader change.

## Risks / Trade-offs

- Breaking API (`.0` access removed). → Mitigation: pre-release, `publish = false`,
  no external users; only three in-repo construction sites change.
