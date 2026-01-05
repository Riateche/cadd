[![Docs](https://docs.rs/cadd/badge.svg)](https://docs.rs/cadd/latest/cadd/)
[![Crates.io](https://img.shields.io/crates/v/cadd.svg)](https://crates.io/crates/cadd)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Riateche/cadd#license)

# {{crate}}

{{readme}}

## Development tips

### Updating generated code in `ops.rs`

```
cargo run -p cadd_generator -- . && cargo fmt && cargo clippy && cargo doc
```

### Updating `README.md`

```
cargo install cargo-readme
cargo readme --output README.md
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
