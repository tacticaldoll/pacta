# Backlog & Deferred Decisions

This file records deferred decisions and candidate patterns. It is not a phase
roadmap and does not create implementation commitments. Shipped truth lives in
`openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- Core contract vocabulary and `Registry` lifecycle authority are shipped.
- Pacta-native executor, middleware, policy, and driver skeletons are shipped.
- CI, cargo-deny, rustdoc, clippy, fmt, Tianheng dependency boundaries, and
  governance checks are shipped.
- Product positioning and blueprint boundaries are governed by the active
  `establish-product-vision-blueprint` change until synced.

## Candidate Pattern Areas

These areas may become future OpenSpec changes only after their boundary and
extension surface are justified.

### Execution Composition

- Retry, timeout, rate limit, tracing, and similar orchestration behavior.
- Policy evaluation semantics.
- Composition ergonomics around `Executor`.

Surface: execution composition.

### Registry Conformance

- Shared conformance tests for lifecycle behavior.
- Backend-agnostic correctness checks.
- In-memory or durable registry implementations.

Surface: lifecycle persistence.

### Integration Boundaries

- Framework or runtime adapters.
- Transport ingress patterns.
- External observability exports.

Surface: integration boundary.

Adapter examples are not core commitments. Compatibility work must stay outside
the core unless a future spec proves a Pacta-native boundary.

### Operator Review

- Tribunal inspection patterns.
- Manual review flows for exhausted pacts.
- Minimal operational visibility.

Surface: user-defined obligation or integration boundary, depending on the
proposal.

## Explicitly Deferred

- Workflow DAGs and inter-obligation dependency graphs.
- Built-in schedulers as core behavior.
- Broad broker behavior.
- Exactly-once delivery as a core guarantee.
- Backend-specific business policy in the lifecycle kernel.

## Prioritization

Prefer changes that preserve thinness and strengthen governance:

1. Protect the lifecycle kernel and domain vocabulary.
2. Keep user obligations user-owned.
3. Add behavior only as a governed pattern on a named extension surface.
4. Reject backflow from adapters, benchmarks, or backend convenience into core.
