## ADDED Requirements

### Requirement: Advanced-Tier Enums Declare Their Evolution
Pacta SHALL manifest the advanced tier's stated stability intent in the compiler by
marking its evolving kernel protocol enumerations `#[non_exhaustive]`. The kernel
driving-protocol enums (`Directive`, `Notice`, `StepResult`) SHALL be
`#[non_exhaustive]`, so a downstream match must carry a wildcard arm and a later
added variant is not a breaking change — turning the tier's "the API may evolve"
promise from prose into an enforced contract.

#### Scenario: The kernel protocol enums are non-exhaustive
- **WHEN** the kernel driving-protocol enums are compiled
- **THEN** `Directive`, `Notice`, and `StepResult` are `#[non_exhaustive]`, so a downstream exhaustive match without a wildcard arm does not compile

#### Scenario: A new protocol variant is not breaking
- **WHEN** a later release adds a variant to `Directive`, `Notice`, or `StepResult`
- **THEN** downstream crates that matched with a wildcard arm continue to compile, because the enums were declared non-exhaustive at 0.1.0
