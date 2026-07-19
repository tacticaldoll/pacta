## Context

Pacta began as a correction to Worklane's heavier task-queue shape: keep the
durable lifecycle thin, move orchestration into composition, and let users own
their obligation semantics. The current codebase already reflects that turn:
core crates are Pacta-native, Tower is adapter scope, and Tianheng governs the
crate dependency graph.

The remaining risk is prose drift. `AGENTS.md`, `README.md`, `PROJECT.md`, and
`BACKLOG.md` can still make future sessions read Pacta as Tower-first,
queue-first, or phase-roadmap driven. That is especially risky because Pacta's
product value is not a broad feature checklist; it is a thin, elegant, governed
framework for durable user-defined obligations.

## Goals / Non-Goals

**Goals:**

- Lock Pacta's product positioning as a thin durable contract fabric and
  governed pattern framework for Rust.
- Define the architecture blueprint as extension surfaces, not promised phases.
- Make Pacta's vocabulary part of governance instead of decorative branding.
- Add executable prose governance for high-risk stale architecture language.
- Keep Tianheng as the governance reaction that bites architecture drift.

**Non-Goals:**

- Add runtime behavior, a backend, a Tower adapter, or orchestration algorithms.
- Turn benchmark comparisons into compatibility commitments.
- Expand the core crate graph or add runtime dependencies.
- Treat every future extension example as required roadmap work.

## Decisions

### Product Positioning Is Governance

Pacta will describe itself as a thin, elegant durable contract fabric and a
governed pattern framework for user-defined obligations. This is narrower than
"task queue" and less integration-specific than "Tower middleware runtime".

Alternative considered: leave positioning in conversational context only. That
fails because future agents and contributors read repository prose, not chat
history.

### Blueprint Names Surfaces, Not Phases

The blueprint will name four extension surfaces: user-defined obligation,
execution composition, lifecycle persistence, and integration boundary. These
surfaces explain where future patterns may attach without creating a phase list.

Alternative considered: keep a phase roadmap for adapters, schedulers, and
backends. That repeats Worklane's failure mode by making thinness look like a
temporary prelude to a larger queue framework.

### Tower And Benchmarks Stay External

Tower, Apalis, Worklane, and lightweight background-job crates may be used as
benchmarks or origin context. They must not define Pacta's public core shape.
Tower compatibility remains user/adapter scope until a future governed change
proves otherwise.

Alternative considered: define a core `Service`-style abstraction now. That
would couple Pacta's durable obligation model to request/response expectations
before the native design has earned that shape.

### Tianheng Governs Prose Drift

The existing `pacta-governance` binary will grow a small prose gate. It will scan
active project prose for high-risk phrases that imply the old architecture. This
keeps the reaction in the same governance path CI already runs.

Alternative considered: document the rule only. That leaves the most dangerous
drift channel unenforced.

## Risks / Trade-offs

- **Risk: vocabulary checks become too blunt.** Mitigation: scan active guidance
  and current specs, not archived ADR/change history; keep the phrase list short
  and focused on architecture-defining drift.
- **Risk: product prose becomes marketing.** Mitigation: anchor every claim in a
  spec, a blueprint boundary, or an executable governance reaction.
- **Risk: benchmark references imply compatibility.** Mitigation: document that
  benchmarks are used for contrast and calibration, not API commitments.
- **Risk: the blueprint hides needed implementation choices.** Mitigation: keep
  concrete implementation work in future changes with their own specs.
