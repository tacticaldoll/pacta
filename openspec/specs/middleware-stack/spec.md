# Middleware Stack Specification

## Purpose

Define Pacta's reified execution-composition mechanism: an empty-stack value, a stack value
that is itself a `Middleware`, and a blind assembler that make composition a holdable, ordered,
legible thing — the closure property turned into a value — without shipping any concrete policy
middleware.

## Requirements

### Requirement: Empty Composition Value
Pacta SHALL provide `Identity`, a no-op `Middleware` whose `wrap` returns the given executor
unchanged, so "zero middleware" is a first-class, holdable value and the neutral element of
composition.

#### Scenario: Identity leaves the executor unchanged
- **WHEN** `Identity` wraps an executor and the composed executor executes a claimed pact
- **THEN** the observed outcome is exactly the inner executor's outcome, with no behavior added

#### Scenario: Identity is the neutral element of a stack
- **WHEN** `Identity` is the base an assembler starts from and no middleware is added
- **THEN** applying the assembler to an executor yields an executor whose behavior equals the
  bare executor's

### Requirement: Reified Composition Value
Pacta SHALL provide `Stack<Inner, Outer>` that itself implements `Middleware`, reifying the
closure property (middleware compose into a `Middleware`) as a value that can be named, stored,
and passed *before* an executor exists. `Stack` SHALL be expressed in Pacta-native execution
vocabulary and SHALL NOT expose Tower, Service, Layer, or HTTP types as its governing public
shape.

#### Scenario: A stack is itself a middleware
- **WHEN** `Stack<Inner, Outer>` wraps an executor
- **THEN** the result is an `Executor`, so a stack composes with further middleware exactly as a
  single middleware does

#### Scenario: A composed stack is a value held before an executor exists
- **WHEN** two or more middleware are combined into a `Stack`
- **THEN** the `Stack` is a value that can be named, stored, and passed as one middleware prior to
  being applied to any executor

### Requirement: Blind Stack Assembler
Pacta SHALL provide an assembler that builds a composed middleware by accumulating `Stack` over
`Identity` through a single generic add operation. The assembler SHALL be **blind**: it accepts
any `Middleware` through the generic operation and SHALL NOT expose any named orchestration or
policy method (no `retry`, `timeout`, `backoff`, `circuit`, `quota`, or `rate-limit`), so the
seam stays a mechanism and cannot accrete sibling-owned or non-goal policy under a convenience
method.

#### Scenario: The assembler composes any middleware through one generic operation
- **WHEN** a middleware is added to the assembler
- **THEN** it is accepted through a single generic add operation that does not inspect what the
  middleware is or does

#### Scenario: The assembler exposes no named policy method
- **WHEN** the assembler's public API is compiled
- **THEN** it offers only the generic add operation and an application operation, and no method
  named for retry, timeout, backoff, circuit, quota, or rate-limit

#### Scenario: Applying the assembler yields the composed executor
- **WHEN** the assembler holding a sequence of middleware is applied to an executor
- **THEN** it produces the executor wrapped by every added middleware in the documented order

### Requirement: Documented Composition Order
Pacta SHALL document the order in which stacked middleware observe an execution, so composition
is legible rather than inferred from nesting direction, and the runtime order SHALL match the
documented convention.

#### Scenario: Composition order is stated and honored
- **WHEN** middleware are stacked through the assembler
- **THEN** the documentation states which middleware observes the execution first, and the order
  in which middleware wrap the execution at runtime matches that statement

#### Scenario: The reified composition preserves the closure property under test
- **WHEN** `cargo test --workspace` runs
- **THEN** a test drives a composition of two pass-through middleware assembled through
  `Identity`, `Stack`, and the assembler to a settlement, so the reified mechanism is proven to
  compose and fails if that property regresses

### Requirement: Composition Order Is Proven By A Full Enter/Exit Trace
Pacta SHALL prove the documented composition order with an observable enter/exit trace, not only by
comparing the final `Outcome`. A trace-recording middleware SHALL record when each layer is entered
and exited around the inner execution, and a test SHALL assert the complete ordering for a
composition of two such middleware over an executor: the **first** middleware added with `then` is
outermost — it is entered first and exited last; the **second** is nested within it — entered
second and exited second-to-last; and the executor is innermost. Asserting the full trace, rather
than the settled `Outcome` alone, proves the nesting direction and not merely that composition
produces some working executor.

#### Scenario: The full enter/exit trace matches the documented nesting
- **WHEN** a composition of two trace-recording middleware over an executor executes a pact
- **THEN** the recorded trace is: first-enter, second-enter, executor, second-exit, first-exit — so the first-added middleware is outermost (entered first, exited last), the second is nested within it, and the executor is innermost

#### Scenario: The proof compares the trace, not only the outcome
- **WHEN** the composition-order proof runs
- **THEN** it asserts the full ordered enter/exit trace rather than only the final `Outcome`, so a regression that inverted the nesting while preserving the outcome would still fail
