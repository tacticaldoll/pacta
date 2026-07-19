# Project Contract

## Vision

Pacta is a thin, elegant durable contract fabric and governed pattern framework
for Rust user-defined obligations.

It exists to fill a narrow gap in the Rust ecosystem: the thinnest useful core
for durable execution without forcing users into a heavy broker, broad workflow
engine, or framework-first runtime. Pacta keeps the durable lifecycle small and
lets users compose their own obligation semantics around it.

## Product Positioning

Pacta is for systems that need durable user-defined obligations with clear
lifecycle authority:

```text
Signal -> Pact -> Claim -> Execution -> Settlement
```

The product promise is not "more queue features". It is a clean place to attach
patterns:

- user-defined obligation semantics
- Pacta-native execution composition
- lifecycle persistence through registries
- adapter-owned integration boundaries

## Core Contract

The behavior that must be protected at all costs:

- **Thin lifecycle kernel**: `pacta-contract` owns durable pact data and
  `Registry` lifecycle authority. It does not own retries, timeouts, routing,
  scheduling, transport adapters, or backend-specific business behavior.
- **Composition over accumulation**: execution behavior grows through
  Pacta-native middleware, policies, and named governed design patterns — the
  first of which is the transition port (`apply`): the backend owns *how* a
  transition is applied atomically, the kernel owns *what* it decides.
- **The runtime is the consumer's**: pacta owns the colorless mechanism; the async
  binding forces no `Send` bound on its futures, so async and executor coloring are
  the consumer's composition, not baked into the contract. (Both bindings do require a
  backend *type* to be `Send + Sync` — thread-shareable — a shareability requirement
  distinct from that future coloring.)
- **Correctness is self-proven**: `pacta-conformance` is the definition of a
  correct backend, so a design is proven when the reference backend passes the
  suite — never validated by a downstream consumer. Consumers ignite pacta's
  work; they never gate it, and performance on a real backend is consumer-owned
  edge, not a correctness gate.
- **Adapter boundary**: framework, transport, and storage integrations remain
  outside the core and cannot define first-layer Pacta APIs.
- **Governance with teeth**: Tianheng and project specs enforce the boundaries
  that prose claims.

## Elegance

Elegance in Pacta is technical restraint:

- fewer lifecycle states
- precise contract/arbitration vocabulary
- user-owned obligations
- small composable interfaces
- executable governance against architectural drift

The naming universe is part of that restraint. `Pact`, `Docket`, `Clause`,
`Brief`, `Registry`, `Claim`, `Retainer`, `Fulfill`, `Breach`, and `Tribunal`
are not decorative terms; they keep the system from sliding back into generic
queue-runtime thinking.

## Non-Goals

Pacta core is not:

- a message broker
- a general job queue feature platform
- a workflow engine
- a scheduler
- a transport framework
- a backend abstraction that owns business policy

## References

- Canonical shipped requirements: `openspec/specs/`
- Active proposed requirements: `openspec/changes/`
- Architecture blueprint: `docs/blueprint.md`
- Domain language: `docs/domain-language.md`
- Deferred decisions: `BACKLOG.md`
