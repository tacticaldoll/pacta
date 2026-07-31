# pacta-governance

Executable architectural governance for the Pacta workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It runs
the [Tianheng](https://github.com/tacticaldoll/tianheng) family (guibiao static
boundaries + hunyi semantic reactions) to keep the workspace's architecture from
drifting:

- the dependency boundaries between crates (every workspace crate must have one);
- the sans-I/O purity of the core (`pacta-contract` reads no ambient clock and performs no
  synchronous I/O);
- the colorless-kernel reactions on **both** the step-driver `kernel` and the `lifecycle`
  kernel — no exposed `async fn`, no return-position `impl Trait` — plus the step-driver
  kernel's no-`serde` rule;
- the facade's kernel-exclusion (only the `kernel` module is barred from `pacta`; `lifecycle`
  is the re-exported backend-author surface) and its re-exports-only shape;
- the executor's orchestration-vocabulary reaction (no public `retry`/`timeout`/`backoff`/
  `circuit`/`quota`/`rate-limit` symbol);
- active-prose drift.

Run it from the workspace root:

```sh
cargo run -p pacta-governance -- check --manifest-path Cargo.toml
```

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
