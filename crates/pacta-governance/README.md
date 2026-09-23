# pacta-governance

Executable architectural governance for the Pacta workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It runs
the [Tianheng](https://github.com/tacticaldoll/tianheng) family (guibiao static
boundaries + hunyi semantic reactions) to react to the architectural drift it can
observe:

- the normal-dependency boundaries between crates (every workspace crate must have one);
- the sans-I/O source scans on the core (`pacta-contract` makes no inline `std::time` `now`
  call, no inline `uuid` `now_v7`/`now_v1` call, and no inline call into a
  `std::io`/`fs`/`net`/`process` path);
- the colorless-kernel reactions on **both** the step-driver `kernel` and the `lifecycle`
  kernel — no public `async fn` anywhere in either subtree, no public fn or method in either
  module itself returning a written `impl Trait` — plus the step-driver kernel's no-`serde`
  rule;
- the facade's kernel-exclusion (only the `kernel` module is barred from `pacta`; `lifecycle`
  is the re-exported backend-author surface) and its re-exports-only shape;
- the executor's orchestration-vocabulary reaction (no `retry`/`timeout`/`backoff`/
  `circuit`/`quota`/`rate-limit` name declared on a line that begins with `pub ` and defines
  a `fn`, `struct`, `enum`, `trait`, `type`, `static`, `union`, `mod`, or `macro`; the line
  scan skips `pub use` re-exports, `pub const` values, trait method declarations, and `pub`
  fields);
- active-prose drift;
- a freshness-gated projection of the accepted constitution in
  `AGENTS.pacta-law.md`.

Each reaction observes only what its boundary's reason states. What a source scan cannot
see — a macro-expanded item, a clock read or I/O call through a method on a value — stays
review-governed; a green check means no visible violation, not proof of the judgment.

Run it from the workspace root:

```sh
cargo run -p pacta-governance -- check --manifest-path Cargo.toml
```

The Rust `Constitution` in `src/main.rs` is the executable authority.
`AGENTS.pacta-law.md` is generated orientation for contributors and agents, while
OpenSpec specs remain the durable requirements. Regenerate the projection only
after a deliberate, separately authorized law change:

```sh
BLESS=1 cargo test -p pacta-governance law_projection_is_fresh
```

Without `BLESS`, the same test fails if the projection is missing, unreadable, or
stale. Tianheng's `GovernanceTest` also proves the current workspace is clean and
that every workspace crate remains covered; focused negative fixtures continue to
assert the exact reaction that bites.

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
