## Context

`pacta-executor` currently exposes `Execution`, `Outcome`, `Settlement`, and
`Executor`. The previous change made core dependency and adapter boundaries
executable, so this change can safely add the first Pacta-native composition
surface without importing Tower's `Service`, `Layer`, request, or response
shape.

The first composition API must be small enough to stay honest. Pacta does not
yet implement retries, timeouts, rate limits, tracing spans, or async execution.
This change therefore establishes the wrapping shape and policy vocabulary only.

## Goals / Non-Goals

**Goals:**

- Add a Pacta-native `Middleware` abstraction that can wrap one `Executor` into
  another `Executor`.
- Add a minimal `Policy` value that names orchestration intent without executing
  orchestration behavior.
- Prove composition with focused tests in `pacta-executor`.
- Keep the core dependency closure clean: no new normal dependencies.
- Keep Tower and HTTP vocabulary out of the core API.

**Non-Goals:**

- Do not add Tower compatibility or adapter crates.
- Do not add `Service`, `Layer`, `Request`, or `Response` types.
- Do not implement retry, timeout, rate-limit, delay, or scheduling behavior.
- Do not introduce async execution or futures.
- Do not change registry lifecycle semantics.

## Decisions

1. `Middleware` wraps an `Executor` into another `Executor`.

   The trait shape is intentionally narrow:
   `fn wrap(&self, executor: E) -> Self::Executor`. It describes composition
   without inheriting Tower's readiness, request, response, or layer contract.
   A middleware is therefore a Pacta-native decorator over execution, not a
   transport abstraction.

   Alternative considered: define a `Service`-like `call` trait. That would make
   Pacta's core API look like a framework adapter and reopen the leak the last
   change closed.

2. `Policy` is vocabulary, not behavior.

   The first policy type should be small and inspectable, such as a named policy
   with a stable `name`. Actual retry, timeout, rate-limit, or scheduling
   behavior remains deferred. This gives docs and APIs a domain term without
   pretending orchestration exists.

   Alternative considered: add enum variants like `Retry` and `Timeout` now.
   That would create product promises before those semantics are specified.

3. Composition tests use hand-written test executors and middleware.

   Tests should prove that middleware can wrap an executor, observe an
   execution, and preserve or alter an `Outcome` through Pacta-native types only.
   No external dependency should be needed.

   Alternative considered: add helper crates or mocking dependencies. That would
   immediately pressure the closed dependency allowlists for little value.

## Risks / Trade-offs

- The first `Middleware` shape may need refinement when async execution arrives.
  Mitigation: keep the trait minimal and avoid commitments to readiness,
  buffering, or transport semantics.
- A policy value without behavior may feel thin. Mitigation: this is deliberate;
  it names the extension point while specs defer orchestration algorithms.
- `Middleware` can be implemented in many styles. Mitigation: tests document the
  intended decorator shape without adding a framework.
