# Product Positioning Specification

## Purpose

Define Pacta's product category, elegance, lightness, benchmark stance,
composition-pattern documentation, and explicit non-positioning.
## Requirements
### Requirement: Product Category
Pacta SHALL position itself as a thin, elegant durable contract fabric and governed pattern framework for Rust user-defined obligations.

#### Scenario: Product prose states category
- **WHEN** active product prose describes what Pacta is
- **THEN** it describes Pacta as a durable contract fabric or governed pattern framework rather than a job queue, broker, workflow engine, or Tower-first runtime

#### Scenario: User obligations define value
- **WHEN** active product prose explains Pacta's value
- **THEN** it centers durable user-defined obligations and composable execution rather than built-in queue features

### Requirement: Elegance And Lightness
Pacta SHALL define elegance as engineering restraint: minimal lifecycle state, composable execution, user-owned obligations, and enforceable boundaries.

#### Scenario: Elegance is technical
- **WHEN** active project prose uses elegance as a Pacta value
- **THEN** it ties elegance to thin lifecycle state, clear vocabulary, composition, and governance rather than decorative branding

#### Scenario: Lightness is preserved
- **WHEN** active project prose describes future growth
- **THEN** it frames growth as thin, governed extension patterns rather than broad queue-runtime feature accumulation

### Requirement: Benchmark Stance
Pacta SHALL use Worklane, Apalis, Tower, and lightweight background-job systems as calibration references without inheriting their architecture.

#### Scenario: Worklane is origin context
- **WHEN** active project prose mentions Worklane
- **THEN** it treats Worklane as origin context and a bloat warning rather than a blueprint to recreate

#### Scenario: External projects are benchmarks
- **WHEN** active project prose mentions external queue or middleware projects
- **THEN** it treats them as comparison points rather than compatibility promises

### Requirement: Non-Positioning
Pacta SHALL explicitly reject product positions that would force heavy broker, queue, workflow, or framework-first semantics into the core.

#### Scenario: Core is not a broker
- **WHEN** active product prose describes Pacta's core
- **THEN** it does not present the core as a broker, scheduler, workflow engine, or queue feature platform

#### Scenario: Core is not Tower-first
- **WHEN** active product prose describes integration with Tower or middleware ecosystems
- **THEN** it keeps those integrations outside the product's core positioning

### Requirement: The Composition Pattern Is Documented
Pacta SHALL document, where consumers look, its mechanism-versus-policy composition
pattern: the core — including the reference `Driver`, a mechanical loop that decides
no outcome — owns the lifecycle mechanism and no policy, and the consumer composes
the policy at the `Registry`, `Executor`, and `Middleware` seams. The documentation
SHALL depict only shipped behavior — presenting `Middleware` as the composition seam
rather than as shipped retry, timeout, or backoff — and SHALL name no sibling product.

#### Scenario: The README distinguishes what the core owns from what the consumer composes
- **WHEN** a consumer reads the README
- **THEN** it distinguishes what Pacta owns (the lifecycle contract, the sans-I/O kernel, lease/lapse recovery, and the reference `Driver` that decides no outcome) from what the consumer composes (a `Registry` backend, an `Executor`, and `Middleware`), and states that the core decides what happens to a pact but not how much to retry or when to give up

#### Scenario: The documented pattern claims only shipped behavior
- **WHEN** the composition pattern is documented
- **THEN** it presents `Middleware` as the seam where policy such as retry, timeout, or fail-fast composes, without presenting those as behavior Pacta ships

