## ADDED Requirements

### Requirement: Runtime Injects Current Time
The runtime SHALL supply the current time to time-dependent registry operations
while the sans-I/O kernel stays time-free, so reading the clock is a runtime
concern and the kernel commits to no time source.

#### Scenario: The driver injects time into time-dependent registry operations
- **WHEN** the driver performs a claim directive
- **THEN** it obtains the current time and passes it to the time-dependent
  registry operation rather than the registry reading a clock itself

#### Scenario: The kernel issues time-free directives
- **WHEN** the kernel issues a claim directive
- **THEN** the directive carries no time, and the runtime attaches the current
  time when it performs the directive
