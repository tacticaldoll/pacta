## Context

The release branch already contains Pacta's foundation: `pacta-contract`
defines `Pact`, `Store`, `ReservationReceipt`, `Reservation`, and lifecycle
methods `reserve`, `heartbeat`, `ack`, and `nack`; `PROJECT.md` and `README.md`
explain the storage-purity and middleware-execution axioms; `BACKLOG.md`
outlines future driver, conformance, and backend phases.

The new vocabulary must therefore be a deliberate public API migration, not a
fresh glossary layered on top of starter files.

## Goals / Non-Goals

**Goals:**

- Rename public model, trait, method, field, and documentation terms so Pacta
  reads as a contract/arbitration system.
- Keep storage purity intact: the registry remains a pure lifecycle state
  machine and does not gain retry, backoff, routing, priority, or payload
  inspection.
- Update roadmap language so future driver, conformance, backend, and terminal
  review work follows the same vocabulary.
- Keep the zero-dependency contract boundary enforced by governance.

**Non-Goals:**

- Add runtime behavior, new storage backends, retry middleware, or tribunal
  persistence.
- Change serialization formats beyond the public type and field names required
  by the vocabulary migration.
- Add compatibility aliases for the old names in this pre-release API.
- Rename private helper names when plain engineering terms are clearer.

## Decisions

1. Rename `Store` to `Registry`.

   `Registry` is the public storage role that preserves dockets and pact
   lifecycle state. The storage-purity axiom remains unchanged: it must not
   compute backoff, inspect clauses, own routing policy, or execute pacts.

2. Rename lane selection to dockets.

   `Pact::lane` becomes `Pact::docket`, and selection parameters use dockets.
   `Docket` replaces the old lane metaphor without implying a stronger ordering
   guarantee than future specs declare.

3. Rename payload to clause and operational envelope data to brief.

   `Pact::payload` becomes `Pact::clause`. If public operational metadata is
   introduced, it uses `Brief`; the current foundation does not need to add a
   `Brief` field unless implementation work identifies a concrete attribute.

4. Rename reservation authority to claim and retainer.

   `ReservationReceipt` becomes `Retainer`, and `Reservation` becomes `Claim`.
   `Registry::reserve` becomes `Registry::claim`. `heartbeat` remains acceptable
   as a plain mechanical verb unless the apply phase introduces a clearer
   retainer-specific method name without obscuring behavior.

5. Rename completion methods to fulfill and breach.

   `ack` becomes `fulfill`; `nack` becomes `breach`. Terminal-review naming
   uses `Tribunal`, but this change does not implement tribunal persistence.

6. Keep `Executor` as the future public role for pact handling.

   Existing docs use `Handler` for the middleware stack. This change establishes
   `Executor` as the public role while allowing internal implementation to use
   handler/service terminology when it refers to Tower mechanics.

## Risks / Trade-offs

- **Risk:** Renaming public API types could obscure queue-like semantics for new
  contributors -> Mitigation: document each Pacta term with its old engineering
  counterpart in `PROJECT.md`, `README.md`, and an ADR.
- **Risk:** `Registry` might be confused with a type registry or service
  registry -> Mitigation: define it narrowly as the storage lifecycle role.
- **Risk:** Retaining `heartbeat` may leave one old lease-era verb in the public
  API -> Mitigation: review during apply; rename only if a clearer term emerges
  that preserves mechanical clarity.
- **Risk:** Scope could expand into runtime behavior -> Mitigation: keep this
  change to naming, docs, and compile-safe contract API migration.

## Migration Plan

1. Update `pacta-contract` public names and crate documentation.
2. Update `PROJECT.md`, `README.md`, and `BACKLOG.md`.
3. Add a domain-language ADR.
4. Run the documented Definition of Done commands.
5. Sync the `domain-language` spec after implementation review.
