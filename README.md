# cadd

## `cadd`: painless checked arithmetics and conversions

Features:
* [`ops`](https://docs.rs/cadd/latest/cadd/ops/index.html):
  Checked arithmetics with `Result` and backtraces
* [`Cinto`](https://docs.rs/cadd/latest/cadd/convert/trait.Cinto.html):
  `TryInto` with better error messages and backtraces for number conversions
* [`SaturatingInto`](https://docs.rs/cadd/latest/cadd/convert/trait.SaturatingInto.html):
  infallible number conversion that returns the closest valid value
* [`non_zero`](https://docs.rs/cadd/latest/cadd/convert/fn.non_zero.html)
  and [`to_non_zero()`](https://docs.rs/cadd/latest/cadd/convert/trait.ToNonZero.html):
  conversion to [`NonZero`](https://doc.rust-lang.org/nightly/core/num/struct.NonZero.html)
  with `Result` and backtraces
* <code>.[into_type](https://docs.rs/cadd/latest/cadd/convert/trait.IntoType.html)::&lt;T&gt;()</code>
  as an alternative to `into()` and `try_into()` without type inference errors

### Intro to checked and unchecked math

In Rust, most of the basic arithmetic operations (like `a + b`) are *unchecked* in Release mode.
This means that they can silently overflow. This is great for performance, but may not be so great
when your application bills the customer the wrong amount of money.

In addition, some of the operations (like `a / b` and `a.ilog(b)`) will panic if their preconditions
are unmet. This can bring down the whole process if you're not careful. If the inputs are untrusted,
a checked alternative should be used.

Thankfully, Rust offers great capabilities for *checked* arithmetics.
For every operation that can overflow or otherwise fail,
the standard library contains a function with the `checked_` prefix that returns `Option`.

Let's suppose we have some (almost) production-ready code:
```rust
async fn handle_request(&self) -> anyhow::Result<()> {
    let price = self.price()?;
    let discount_rate = self.discount_rate();
    let amount = price - discount_rate * price / 100;
    self.bill_user(amount).await?;
    Ok(())
}
```
After it billed some user $18446744073709551596 by accident, we decided to use checked arithmetics in our business logic:
```rust
    use anyhow::Context as _;

    let amount = discount_rate
        .checked_mul(price)
        .and_then(|v| v.checked_div(100))
        .and_then(|v| price.checked_sub(v))
        .context("amount overflow")?;
```
Now this is production ready! And also quite painful to look at.

### Checked operations with `cadd`

`cadd` provides traits and functions that make checked arithmetics just as easy to do as
unchecked ones. Just add "c" to the name of the corresponding unchecked function
and import it:
```rust
use cadd::ops::{csub, Cmul, Cdiv};

let amount = csub(
    price,
    discount_rate.cmul(price)?.cdiv(100)?,
)?;
```
Not only it's much more consise, but it also returns a `Result` with an error type that contains
the failed operation, its arguments, and a backtrace:
```
overflow: 100 - 200
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
...
```
You can also freely choose between method form
(<code>a.[cadd](https://docs.rs/cadd/latest/cadd/ops/trait.Cadd.html#tymethod.cadd)(b)</code>)
and free function form (<code>[cadd](https://docs.rs/cadd/latest/cadd/ops/fn.cadd.html)(a, b)</code>)
as you see fit.
And it's not just operators (`+`, `-`, etc). For every `checked_*` function in `std`, there is a corresponding
function in `cadd`: [`cdiv_euclid`](https://docs.rs/cadd/latest/cadd/ops/fn.cdiv_euclid.html),
[`cilog2`](https://docs.rs/cadd/latest/cadd/ops/fn.cilog2.html), and so on.
See [`ops`](https://docs.rs/cadd/latest/cadd/ops/index.html) module documentation for more information.

License: MIT OR Apache-2.0
