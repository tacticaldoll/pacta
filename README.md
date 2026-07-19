# Pacta

A Pacta-native, middleware-oriented contract runtime for Rust.

Pacta was born from a fundamental critique of traditional, broker-centric job queues. Traditional queues suffer from "Semantic Bloat" — they pull execution policies (retries, backoff, routing, dead-letter storage, priority) into the storage layer and the core job envelope. This forces every storage backend (Postgres, Redis, etc.) to implement complex business logic, making the system heavy, hard to extend, and prone to breaking.

Pacta takes a different approach: **Execution is Middleware, the Registry is
purely a Lifecycle State Machine.**

## Philosophy

By completely delegating execution orchestration (retries, timeouts, tracing,
rate limiting) to the middleware ecosystem, the storage role (`Registry`)
degrades into a pure, minimal state machine. The Registry only tracks lifecycle
transitions: `claim`, `heartbeat`, `fulfill`, and `breach`.

- **Registry**: Pure state machine. Never computes backoff, never manages delays,
  and never inspects clauses.
- **Executor**: Public role for executing claimed pacts through middleware.
- **Middleware / Policy**: Pacta-native decorators and orchestration intents. Currently an architectural skeleton; actual retry, timeout, and rate-limit behavior are deferred to future implementation.
- **Contract**: Zero-dependency `pacta-contract` enforced by `tianheng`.

## Domain Language

Pacta's public API uses contract and arbitration terms:

- `Pact` - durable unit of obligation.
- `Docket` - grouping from which pacts are selected.
- `Clause` - business data carried by a pact.
- `Claim` and `Retainer` - short-term processing authority.
- `Fulfill` and `Breach` - lifecycle settlement outcomes.
- `Middleware` and `Policy` - native execution composition and orchestration intent.
- `Tribunal` - terminal review for exhausted pacts.

See `docs/domain-language.md` for the full glossary and legacy mapping.

## Getting Started

*(Wait for Phase 2 implementation...)*

## Contributing

This project uses **OpenSpec** (spec-driven development) and strictly enforces AI-agent-friendly governance. 

1. Please read `AGENTS.md` before contributing. It contains the project's absolute axioms and adversarial review stance.
2. Read `PROJECT.md` for terminology.
3. Check `BACKLOG.md` for roadmap and deferred work.
4. Use the `openspec` CLI to propose and apply changes.
5. Run the quality gates before checking off implementation tasks.

```bash
# Scaffold a new change
openspec new change "your-feature-name"

# Definition of Done
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo run -p pacta-governance -- check --manifest-path Cargo.toml
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
