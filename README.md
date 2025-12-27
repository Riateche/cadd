[![Docs](https://docs.rs/cadd/badge.svg)](https://docs.rs/cadd/latest/cadd/)
[![Crates.io](https://img.shields.io/crates/v/cadd.svg)](https://crates.io/crates/cadd)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Riateche/cadd#license)

# cadd

## `cadd`: painless checked arithmetics and conversions

Features:
* [`ops`]: checked arithmetics with `Result`, informative errors, and backtrace.
* [`Cinto`](convert::Cinto): `TryInto` alternative for type conversions with better error messages and backtrace.
  Works for integer types, other primitives, arrays, and string-like types.
* [`SaturatingInto`](convert::SaturatingInto):
  infallible number conversion that returns the closest valid value.
* [`non_zero`](convert::non_zero) and [`to_non_zero()`](convert::ToNonZero):
  conversion to [`NonZero`] with `Result`, informative errors, and backtrace.
* <code>.[into_type](convert::IntoType)::&lt;T&gt;()</code>
  as an alternative to `into()` and `try_into()` without type inference errors.
* `no_std` support (`alloc` is still required).

## Introduction

Rust generally offers amazing capability to create safe, correct programs. However, since it is still
a systems programming language, there are still a few aspects where the "default" or the easiest way
to do something is the cheapest, but not always the correct one. One of the most ubiquitous example
is the behavior of arithmetic operations on integer types and casts between them:

* Many arithmetic operators (`+`, `-`, `*`, ...) and some functions (like `.pow()`) are
  *unchecked* in Release mode. This means that they can silently overflow and wrap, producing a value
  that is not the true output of the operation.
* Some of the operations (like `a / b` and `a.ilog(b)`) will panic if their preconditions
  are unmet.
* `as` conversion between integer types will silently truncate values as necessary to fit the output type.
  It's also capable of reinterpreting signed values as unsigned and vice versa, producing a value that
  is not equal to the input.

Sometimes this behavior is fine. However, there are many cases where performance is not as important
as correctness. For most high level applications, calculating the values precisely is much more important,
as these values may represent things like amount of money or amount of physical goods.
Arguably, **checked math and conversions should be the default**. A faster, but potentially wrapping or
truncating alternative should only be chosen in specific cases, after ensuring that it will not cause
a logic bug.

Rust's core library provides very good APIs for checked operations. The only problem with them is that
they require much more verbose code. Compare a simple `a + b` with a properly done checked addition:
`a.checked_add(b).ok_or(Error::Overflow)?`. Any non-trivial code quickly becomes a wall of checked
calls and numerous error conversions.

This is where `cadd` comes in. This crate aims to provide a set of APIs that are (almost)
as easy to use as the basic operations, while being a safe, correct default. These functions
never wrap, truncate, reinterpret, or panic.

Any errors returned by `cadd` will contain as much information as possible (to a reasonable limit)
and will also capture a backtrace if it's enabled. This makes it easy to integrate it into
any error handling approach, be it custom error types or "catch-all" errors like `anyhow`.

## Example

The easiest, but error-prone approach:
```rust
let amount = price - discount_rate * price / 100;
let output = amount as u32;
```
A safe approach that uses `std` checked functions and returns a non-informative error on overflow:
```rust
let amount = discount_rate
    .checked_mul(price)
    .and_then(|v| v.checked_div(100))
    .and_then(|v| price.checked_sub(v))
    .ok_or_else(|| Error::Overflow)?;
let output: u32 = amount.try_into().map_err(|_| Error::Overflow)?;
```
A safe and concise approach with traits and functions from [`cadd::ops`](ops),
returning an informative error:
```rust
use cadd::{ops::{csub, Cmul, Cdiv}, convert::IntoType};

let amount = csub(price, discount_rate.cmul(price)?.cdiv(100)?)?;
let output = amount.cinto_type::<u32>()?;
```
Errors may look like this:
```
failed to compute 20 - 40: u64 overflow
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
...
```

## Recommended clippy lints

In order to eliminate unintended side effects, wraps, truncations, and signed-vs-unsigned reinterpretation,
it's recommended to set the following clippy lints to `warn` level:

- `arithmetic_side_effects`
- `cast_possible_wrap`
- `cast_precision_loss`
- `cast_sign_loss`

## Updating generated code in `ops.rs`

```
cargo run -p cadd_generator -- . && cargo fmt && cargo clippy && cargo doc
```

## Updating `README.md`

```
cargo install cargo-readme
cargo readme --output README.md
```

License: MIT OR Apache-2.0
