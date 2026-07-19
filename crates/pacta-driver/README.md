# pacta-driver

The mechanical runtime loop for Pacta: it performs the directives the sans-I/O
kernel issues, composing a `Registry` and an `Executor` without deciding lifecycle
outcomes itself.

The driver claims through the registry, runs the executor (optionally wrapped in
`Middleware`), and settles the claim as the kernel decides — injecting the current
time so the core reads no ambient clock. It is a reference runtime; to change the
runtime shape you can build your own loop over the kernel instead.

Part of [Pacta](https://github.com/tacticaldoll/pacta); most consumers depend on the
[`pacta`](https://crates.io/crates/pacta) facade rather than this crate directly.

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/pacta/blob/main/LICENSE-MIT), at your option.
