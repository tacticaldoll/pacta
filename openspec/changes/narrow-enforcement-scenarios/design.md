# Design

## Context

The accepted boundary reasons (narrowed to what they observe in an earlier change) and Tianheng
0.6.1's observation bounds state each reaction's perimeter. The reasons already name the partial
coverage: macro-generated items, calls through a method on a value, `now` paths taken as values,
unresolvable self types, descendant modules for the impl-trait tooth, and the executor scan's
skipped `pub use` and `pub const`. The executor line scan (`check_executor_content`) also skips
trait method declarations, which carry no `pub` token, and `pub` fields, which define no item
keyword; the spec names these too, and correcting the accepted reason is left to a separate law
change. Tianheng's bounds add a module nested in a function body for async exposure, and a
re-export chain through another module or a glob for re-export exposure. The conformance crate's
`CONTENTION_ROUNDS` documentation calls the repeated rounds a probabilistic stress and says the
teeth are proven by the deterministic fixture. Its further statement that a non-atomic backend is
"overwhelmingly likely" to be caught is not proven for an arbitrary backend, so the spec does not
repeat it; that rustdoc is left unchanged here.

## Goals / Non-Goals

**Goals:**

- Every statement of what a governance reaction or test catches describes only an observed shape.
- The unobserved remainder is named as review-governed rather than dropped.

**Non-Goals:**

- Changing any boundary, reason, severity, reaction test, or product code.
- Narrowing product intent. Statements such as "the `pacta-contract` core SHALL NOT read an
  ambient wall clock" and "the step-driver kernel SHALL remain reachable only through
  `pacta-contract`" are judgment and may be broader than their tooth.
- Adding "not observed" scenarios, which would assert non-reactions no repository test pins.

## Decisions

- **Name the remainder in the requirement body.** One sentence per requirement says what is
  observed and what review holds, matching the style of the no-I/O and executor requirements,
  which already stated their partiality.
- **Use the reasons' wording.** "Normal dependency", "inline call", "public `async fn`", "written
  `impl Trait`", and "self type it resolves" match the accepted reasons, so prose and law read the
  same.
- **Keep scenario names.** Headings such as "An aliased ambient clock read is still caught" stay,
  since their bodies now name the observed shape.

## Risks / Trade-offs

- [A future Tianheng may observe more] -> The scenarios still hold; the review-governed sentence
  can narrow in a later change.
