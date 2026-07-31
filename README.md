# Pacta

A Tower-native, Middleware-driven task runtime for Rust.

Pacta was born from a fundamental critique of traditional, broker-centric job queues. Traditional queues suffer from "Semantic Bloat" — they pull execution policies (retries, backoff, routing, dead-letter storage, priority) into the storage layer and the core job envelope. This forces every storage backend (Postgres, Redis, etc.) to implement complex business logic, making the system heavy, hard to extend, and prone to breaking.

Pacta takes a different approach: **Execution is Middleware, Storage is purely a Lifecycle State Machine.**

## Philosophy

By completely delegating execution orchestration (retries, timeouts, tracing, rate limiting) to the `Middleware` ecosystem, the storage layer (`Store`) degrades into a pure, minimal state machine. The Store only tracks lifecycle transitions: `reserve` (lease), `ack` (success), and `nack` (failure/dead).

- **Store**: Pure state machine. Never computes backoff, never manages delays.
- **Execution**: Handled by standard `Tower` Middleware.
- **Contract**: Zero-dependency `pacta-contract` enforced by `tianheng`.

## Getting Started

*(Wait for Phase 2 implementation...)*

## Contributing

This project uses **OpenSpec** (spec-driven development) and strictly enforces AI-agent-friendly governance. 

1. Please read `AGENTS.md` before contributing. It contains the project's absolute axioms and adversarial review stance.
2. Read `PROJECT.md` for terminology.
3. Check `BACKLOG.md` for roadmap and deferred work.
4. Use the `openspec` CLI to propose and apply changes.

```bash
# Scaffold a new change
openspec new change "your-feature-name"
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
