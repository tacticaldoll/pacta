# Pacta

Pacta is a thin, elegant durable contract fabric and governed pattern framework
for Rust user-defined obligations.

It is designed for the narrow space between "I need durable execution" and "I
do not want a broker-shaped framework to own my application semantics." Pacta
keeps the durable lifecycle small, then lets users attach obligation semantics,
execution composition, registry persistence, and integrations as governed
patterns.

```text
(Signal) -> Pact -> Claim -> Execution -> Settlement
```

`Signal -> Pact` is user-provided ingress: you turn your own events into pacts.
Pacta ships the lifecycle from `Pact` onward, not an ingress API.

## Status (0.1.0)

0.1.0 is the thin lifecycle foundation, not a complete durable runtime. It ships:

- a curated **facade** (`pacta`) — the recommended single entrypoint you depend on;
- the lifecycle **contract** (`Registry`) and a sans-I/O lifecycle **kernel**;
- **lease/lapse** semantics with injected time (the core reads no ambient clock);
- execution **composition** vocabulary (`Executor`, `Middleware`, `Policy`);
- a mechanical **driver** that runs the kernel against a registry and executor;
- an in-memory **reference backend** (`pacta-memory`);
- a backend-agnostic **conformance** suite (`pacta-conformance`);
- **executable governance** enforcing the architecture.

Durable/persistent backends deliberately live **outside** this workspace and prove
themselves against the conformance suite. No ingress API, framework adapters, or
retry/backoff/timeout orchestration ship in 0.1.0 — see `CHANGELOG.md`.

## Why Pacta

Many queue runtimes become heavy when retries, backoff, routing, scheduling,
dead-letter policy, and backend behavior all collapse into one storage-shaped
center. Pacta takes the opposite path:

- `Registry` preserves lifecycle authority.
- `Executor` handles claimed pacts.
- `Middleware` and `Policy` describe Pacta-native execution composition.
- Adapters and backends remain outside the core contract.
- Tianheng governance rejects architecture drift.

The result is deliberately thin: enough structure to bite, not enough bulk to
own the user's domain.

## Domain Language

Pacta uses contract and arbitration terms as architecture, not branding:

- `Pact` - durable unit of obligation.
- `Docket` - grouping from which pacts are selected.
- `Clause` - business data carried by a pact.
- `Brief` - non-business operational context attached to a pact.
- `Claim` and `Retainer` - short-term processing authority.
- `Fulfill` and `Breach` - settlement outcomes.
- `Tribunal` - terminal review for exhausted pacts.

See `docs/domain-language.md` for the full glossary and legacy mapping.

## Architecture

Start with:

- `PROJECT.md` for vision, positioning, and non-goals.
- `docs/blueprint.md` for extension surfaces and non-commitment boundaries.
- `openspec/specs/` for shipped requirements.
- `BACKLOG.md` for deferred decisions and candidate patterns.

## Contributing

This project uses OpenSpec and Tianheng-native governance.

```bash
openspec new change "your-change-name"

cargo build --workspace
cargo test --workspace
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
cargo run -p pacta-governance -- check --manifest-path Cargo.toml
```

Read `AGENTS.md` before making repository changes.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
