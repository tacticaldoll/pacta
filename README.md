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

## Scope

Pacta is a thin lifecycle foundation, not a complete durable runtime. It ships the durable
lifecycle **contract** (`Registry`, over one `apply` transition port — sync, and behind the
`async` feature an `AsyncRegistry`), a sans-I/O **kernel**, lease/lapse with injected time,
deferred reclaim (`release`), execution **composition** (`Executor`/`Middleware`), a mechanical
**driver**, in-memory **reference backends**, a backend-agnostic **conformance** suite, and
executable **governance**.

It does **not** ship an ingress API, framework adapters, or retry/backoff/timeout orchestration —
those compose at the `Registry`/`Middleware` seams, and durable/persistent backends live outside
this workspace and prove themselves against the conformance suite. See `CHANGELOG.md` for what
each release adds.

## Why Pacta

Many queue runtimes become heavy when retries, backoff, routing, scheduling,
dead-letter policy, and backend behavior all collapse into one storage-shaped
center. Pacta takes the opposite path:

- `Registry` preserves lifecycle authority.
- `Executor` handles claimed pacts.
- `Middleware` describes Pacta-native execution composition.
- Adapters and backends remain outside the core contract.
- Tianheng governance rejects architecture drift.

The result is deliberately thin: enough structure to bite, not enough bulk to
own the user's domain.

## What Pacta owns, and what you compose

Pacta owns a *mechanism* and no *policy*. It decides **what** happens to a pact —
claim, execute, settle, or leave unsettled to lapse — and never **how much** to
retry or **when** to give up. That half is yours, composed at the seams.

```text
You compose  (policy + I/O)
  Registry    your durable backend
  Executor    your work, made idempotent
  Middleware  the seam where retry / timeout / fail-fast compose
        │
        ▼  run by
Pacta owns  (mechanism, no policy)
  the lifecycle contract
  the sans-I/O kernel — decides the lifecycle, fabricates no outcome
  lease / lapse recovery
  a reference Driver — runs your pieces, deciding no outcome itself
```

The core owns the mechanism; you own the policy — and you own the runtime. The async
binding forces no `Send` on its futures, so async and executor choice are yours to
compose (a backend type is `Send + Sync`, thread-shareable, in both bindings).
Orchestration such as retry, timeout, or backoff is not shipped — it
composes onto the `Middleware` seam. You run your `Registry` and `Executor` with the
reference `Driver`, or compose your own loop over the kernel.

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

This project uses OpenSpec and Tianheng-native governance. Start a change with:

```bash
openspec new change "your-change-name"
```

Run the full Definition of Done (see `AGENTS.md`) before committing, and read
`AGENTS.md` before making repository changes.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
