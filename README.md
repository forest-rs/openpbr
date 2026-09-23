# openpbr

A small Rust workspace centered on [`crates/openpbr`](./crates/openpbr), a
`no_std` crate describing the
[OpenPBR Surface](https://academysoftwarefoundation.github.io/OpenPBR/)
parameter set: typed values with the specification's defaults, ranges,
metadata for every parameter, and color-space-typed colors built on
[`color`](https://crates.io/crates/color).

See the [crate README](./crates/openpbr/README.md) for usage.

## Validation

```sh
typos
cargo fmt --all
taplo fmt
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --no-deps
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.
