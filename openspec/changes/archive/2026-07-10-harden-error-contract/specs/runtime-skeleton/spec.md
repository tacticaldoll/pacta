## ADDED Requirements

### Requirement: Public Errors Are Standard Errors
Pacta's public runtime trait errors SHALL be standard errors, so a consumer can
display them, chain them, and convert them into common error types.

#### Scenario: Registry error is a standard error
- **WHEN** the `Registry` trait declares its associated error type
- **THEN** that type is bound by `std::error::Error`

#### Scenario: Executor error is a standard error
- **WHEN** the `Executor` trait declares its associated error type
- **THEN** that type is bound by `std::error::Error`

#### Scenario: The driver error is displayable and chainable
- **WHEN** the driver returns its error
- **THEN** the error implements `Display` and `std::error::Error`, and exposes the
  underlying registry or executor error as its source

#### Scenario: A shipped backend error is a standard error
- **WHEN** a shipped registry backend returns its error
- **THEN** the error implements `Display` and `std::error::Error`
