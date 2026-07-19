## ADDED Requirements

### Requirement: Retainer Encapsulation
The `Retainer` proof token SHALL encapsulate its identifier rather than expose it as
a bare public field, matching its documented role as an authority token that a
registry validates.

#### Scenario: Retainer identifier is not a bare public field
- **WHEN** the contract crate exposes `Retainer`
- **THEN** its identifier is constructed through a constructor and read through an accessor rather than a public field

#### Scenario: Retainer authority is registry-validated
- **WHEN** a settlement presents a `Retainer`
- **THEN** the registry validates it against the claim it issued, rather than the type system proving authority by construction
