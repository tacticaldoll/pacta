# pacta-governance

Executable architectural governance for the Pacta workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It runs
the [Tianheng](https://github.com/tacticaldoll/tianheng) family (guibiao static
boundaries + hunyi semantic reactions) to keep the workspace's architecture from
drifting: the dependency boundaries between crates, the sans-I/O purity of the core
(no synchronous I/O, no ambient clock), the kernel's no-`async` and no-`serde`
reactions, the facade's kernel-exclusion and re-exports-only shape, and
active-prose governance.

Run it from the workspace root:

```sh
cargo run -p pacta-governance -- check --manifest-path Cargo.toml
```

Part of [Pacta](https://github.com/tacticaldoll/pacta).

## License

Licensed under either of
[Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
