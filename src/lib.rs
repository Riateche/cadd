#![no_std]
#![warn(missing_docs)]
//! # `cadd`: painless checked arithmetics and conversions
//!
//! Features:
//! * [ops]: Checked arithmetics with `Result` and backtraces
//! * [`Cinto`](crate::convert::Cinto): `TryInto` with better error messages and backtraces for number conversions
//! * [`SaturatingInto`](crate::convert::SaturatingInto): infallible number conversion that returns the closest valid value
//! * [`non_zero`](crate::convert::non_zero) and [`to_non_zero()`](crate::convert::ToNonZero): conversion to [`NonZero`](std::num::NonZero) with `Result` and backtraces
//! * <code>.[into_type](crate::convert::IntoType)::&lt;T&gt;()</code> as an alternative to `into()` and `try_into()` without type inference errors
//!
//! ## Intro to checked and unchecked math
//!
//! In Rust, most of the basic arithmetic operations (like `a + b`) are *unchecked* in Release mode.
//! This means that they can silently overflow. This is great for performance, but may not be so great
//! when your application bills the customer the wrong amount of money.
//!
//! In addition, some of the operations (like `a / b` and `a.ilog(b)`) will panic if their preconditions
//! are unmet. This can bring down the whole process if you're not careful. If the inputs are untrusted,
//! a checked alternative should be used.
//!
//! Thankfully, Rust offers great capabilities for *checked* arithmetics.
//! For every operation that can overflow or otherwise fail,
//! the standard library contains a function with the `checked_` prefix that returns `Option`.
//!
//! Let's suppose we have some (almost) production-ready code:
//! ```
//! # struct S;
//! # impl S {
//! #   fn price(&self) -> anyhow::Result<u32> { todo!() }
//! #   fn discount_rate(&self) -> u32 { todo!() }
//! #   async fn bill_user(&self, amount: u32) -> anyhow::Result<()> { todo!() }
//! async fn handle_request(&self) -> anyhow::Result<()> {
//!     let price = self.price()?;
//!     let discount_rate = self.discount_rate();
//!     let amount = price - discount_rate * price / 100;
//!     self.bill_user(amount).await?;
//!     Ok(())
//! }
//! # }
//! ```
//! After it billed some user $18446744073709551596 by accident, we decided to use checked arithmetics in our business logic:
//! ```
//! # struct S;
//! # impl S {
//! #   async fn handle_request(&self) -> anyhow::Result<()> {
//! #       let price: u32 = 0;
//! #       let discount_rate: u32 = 0;
//!     use anyhow::Context as _;
//!
//!     let amount = discount_rate
//!         .checked_mul(price)
//!         .and_then(|v| v.checked_div(100))
//!         .and_then(|v| price.checked_sub(v))
//!         .context("amount overflow")?;
//! #       Ok(())
//! #   }
//! # }
//! ```
//! Now this is production ready! And also quite painful to look at.
//!
//! ## Checked operations with `cadd`
//!
//! `cadd` provides traits and functions that make checked arithmetics just as easy to do as
//! unchecked ones. Just add "c" to the name of the corresponding unchecked function
//! and import it:
//! ```
//! # struct S;
//! # impl S {
//! #   async fn handle_request(&self) -> anyhow::Result<()> {
//! #       let price: u32 = 0;
//! #       let discount_rate: u32 = 0;
//! use cadd::ops::{csub, Cmul, Cdiv};
//!
//! let amount = csub(
//!     price,
//!     discount_rate.cmul(price)?.cdiv(100)?,
//! )?;
//! #       Ok(())
//! #   }
//! # }
//! ```
//! Not only it's much more consise, but it also returns a `Result` with an error type that contains
//! the failed operation, its arguments, and a backtrace:
//! ```text
//! overflow: 100 - 200
//! stack backtrace:
//!    0: std::backtrace_rs::backtrace::libunwind::trace
//! ...
//! ```
//! You can also freely choose between method form (<code>a.[cadd](ops::Cadd::cadd)(b)</code>)
//! and free function form (<code>[cadd](ops::cadd)(a, b)</code>) as you see fit.
//! And it's not just operators (`+`, `-`, etc). For every `checked_*` function in `std`, there is a corresponding
//! function in `cadd`: [`cdiv_euclid`](ops::cdiv_euclid), [`cilog2`](ops::cilog2), and so on.
//! See [ops] module documentation for more information.
extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate std;

mod convert_impls;
mod error;
mod ops_impls;
#[cfg(test)]
mod tests;

pub mod convert;
pub mod ops;
pub mod prelude;

pub use crate::error::Error;

/// `Result` with error type defaulting to `cadd::Error`.
pub type Result<T, E = Error> = core::result::Result<T, E>;

// TODO: String <-> OsString conversions
// TODO: &[u8] -> String utf8 conversions
// TODO: Path conversions
// TODO: ops for non-nan and finite floats (real_float crate?)
// TODO: readme
