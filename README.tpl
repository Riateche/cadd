[![Docs](https://docs.rs/cadd/badge.svg)](https://docs.rs/cadd/latest/cadd/)
[![Crates.io](https://img.shields.io/crates/v/cadd.svg)](https://crates.io/crates/cadd)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Riateche/cadd#license)

# {{crate}}

{{readme}}

## Updating generated code in `ops.rs`

```
cargo run -p cadd_generator -- . && cargo fmt && cargo clippy && cargo doc
```

## Updating `README.md`

```
cargo install cargo-readme
cargo readme --output README.md
```

License: {{license}}
