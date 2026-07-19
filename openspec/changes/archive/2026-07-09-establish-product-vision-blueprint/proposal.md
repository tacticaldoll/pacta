## Why

Pacta's implementation has moved to `Registry`, Pacta-native composition, and
adapter-owned Tower integration, but several governance documents still carry
older Worklane/Tower/job-queue language. That drift is dangerous because it can
make future agents treat optional integrations or feature examples as roadmap
commitments and pull Pacta back toward a heavier queue runtime.

## What Changes

- Establish Pacta's product positioning: a thin, elegant durable contract fabric
  and governed pattern framework for Rust.
- Define the plugin-kernel hypothesis: the thinnest durable execution core is
  itself a plugin kernel over user-defined obligations.
- Define an architecture blueprint that names extension surfaces without turning
  them into phases or promised work.
- Define prose governance and document authority rules so AGENTS, README,
  PROJECT, BACKLOG, docs, and specs cannot define competing architectures.
- Rewrite governance prose once across `AGENTS.md`, `PROJECT.md`, `README.md`,
  `BACKLOG.md`, and blueprint/domain documentation.
- Add a governance reaction for high-risk stale vocabulary in active project
  prose.

## Capabilities

### New Capabilities

- `product-positioning`: Defines Pacta's vision, product category, lightness,
  elegance, benchmark stance, and non-positioning.
- `architecture-blueprint`: Defines Pacta's pattern framework, plugin-kernel
  extension surfaces, and non-commitment rules.
- `governance-prose`: Defines document authority, prose drift boundaries, and
  active-prose vocabulary governance.

### Modified Capabilities

- `composition-governance`: Align composition governance with the plugin-kernel
  and pattern-growth principles.
- `quality-governance`: Add prose governance as an executable quality reaction.
- `domain-language`: Clarify that Pacta vocabulary is part of the product
  governance system, not decorative branding.

## Impact

- Affects project prose: `AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`,
  and docs.
- Affects OpenSpec living specs and archived change artifacts for this change.
- Affects `pacta-governance` if prose drift is made executable.
- Does not add runtime behavior, new core crates, Tower adapters, backend
  implementations, or orchestration algorithms.
