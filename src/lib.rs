#![no_std]
#![warn(missing_docs)]

//! # `cadd`: painless checked arithmetics and conversions
//!
//! Features:
//! * [`ext`](https://docs.rs/cadd/latest/cadd/ext/index.html):
//!   checked arithmetics with `Result`, informative errors, and backtraces
//! * [`ops`](https://docs.rs/cadd/latest/cadd/ops/index.html):
//!   generic functions for checked arithmetics
//! * [`Cinto`](https://docs.rs/cadd/latest/cadd/convert/trait.Cinto.html):
//!   `TryInto` with better error messages and backtraces for number conversions
//! * [`SaturatingInto`](https://docs.rs/cadd/latest/cadd/convert/trait.SaturatingInto.html):
//!   infallible number conversion that returns the closest valid value
//! * [`non_zero`](https://docs.rs/cadd/latest/cadd/convert/fn.non_zero.html)
//!   and [`to_non_zero()`](https://docs.rs/cadd/latest/cadd/convert/trait.ToNonZero.html):
//!   conversion to [`NonZero`](https://doc.rust-lang.org/nightly/core/num/struct.NonZero.html)
//!   with `Result`, informative errors, and backtraces
//! * <code>.[into_type](https://docs.rs/cadd/latest/cadd/convert/trait.IntoType.html)::&lt;T&gt;()</code>
//!   as an alternative to `into()` and `try_into()` without type inference errors
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
//! use cadd::{ops::csub, ext::U32Ext};
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
//! You can also freely choose between method form
//! (<code>a.[cadd](https://docs.rs/cadd/latest/cadd/ops/trait.Cadd.html#tymethod.cadd)(b)</code>)
//! and free function form (<code>[cadd](https://docs.rs/cadd/latest/cadd/ops/fn.cadd.html)(a, b)</code>)
//! as you see fit.
//! And it's not just operators (`+`, `-`, etc). For every `checked_*` function in `std`, there is a corresponding
//! function in `cadd`: [`cdiv_euclid`](https://docs.rs/cadd/latest/cadd/ops/fn.cdiv_euclid.html),
//! [`cilog2`](https://docs.rs/cadd/latest/cadd/ops/fn.cilog2.html), and so on.
//! See [`ops`](https://docs.rs/cadd/latest/cadd/ops/index.html) module documentation for more information.

extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate std;

mod convert_impls;
mod error;

pub mod convert;

/// Checked operations on numbers.
///
/// Many operators on integer primitives (`a + b`, `a / b`, etc) and associated functions (`a.pow(b)`, `a.ilog(b)`, etc)
/// can overflow or fail under certain conditions. With debug assertions enabled (default when building in debug mode),
/// any such failures will be caught and converted into a panic. With debug assertions disabled
/// (default when building in release mode), some failures (like division by zero) will still result in a panic,
/// and overflows will silently return an overflown value, which is often an unexpected and incorrect result.
/// Therefore, to write the code that returns correct values on every valid input and correctly handles every invalid input,
/// it's highly recommended to use checked alternatives.
///
/// Rust offers great capabilities for checked arithmetics. For every operation that can overflow or otherwise fail,
/// the standard library contains a function with the `checked_` prefix that returns `Option`. For example:
/// ```
/// assert_eq!(300_u32.checked_add(200_u32), Some(500));
/// assert_eq!(3_000_000_000_u32.checked_add(2_000_000_000_u32), None);
/// ```
/// However, writing code that uses checked functions can be quite cumbersome, especially if you use `Result`
/// throughout the code:
/// ```
/// # use std::error::Error;
/// fn calculate_trajectory(mass: u32, velocity: u32) -> Result<(), Box<dyn Error>> {
///     let kinetic_energy = velocity
///         .checked_pow(2)
///         .and_then(|v| mass.checked_mul(v))
///         .and_then(|v| v.checked_div(2))
///         .ok_or_else(|| "mass or velocity too large")?;
///     //...
///     Ok(())
/// }
/// ```
/// It can be improved by moving all arithmetics into functions that return `Option` so that you can use `?`
/// for early returns, but it requires even more restructuring of the code.
///
/// This crate offers a set of traits and functions for easy handling of checked arithmetics.
/// These traits and functions are modelled after the `checked_*` family of functions provided by the standard
/// library for primitive numeric types, such as [`checked_add`](u32::checked_add),
/// [`checked_pow`](u32::checked_pow), etc. These traits and functions offer a number of benefits
/// over the standard library functions:
///
/// * They return `Result` instead of `Option`, enabling the use of `?` in functions returning `Result`.
///   ```
///   use cadd::{ops::{Cpow, cmul}, ext::U32Ext};
///
///   fn kinetic_energy(mass: u32, velocity: u32) -> cadd::Result<u32> {
///       cmul(mass, velocity.cpow(2)?)?.cdiv(2)
///   }
///   ```
/// * The error values they return provide a meaningful error message and a backtrace:
///   ```
///   # use cadd::{ops::cmul, ext::U32Ext};
///   # fn kinetic_energy(mass: u32, velocity: u32) -> cadd::Result<u32> {
///   #     cmul(mass, velocity.cpow(2)?)?.cdiv(2)
///   # }
///   # fn backtrace_enabled() -> bool {
///   #     match std::env::var("RUST_LIB_BACKTRACE") {
///   #         Ok(s) => s != "0",
///   #         Err(_) => match std::env::var("RUST_BACKTRACE") {
///   #             Ok(s) => s != "0",
///   #             Err(_) => false,
///   #         },
///   #     }
///   # }
///   let err_msg = kinetic_energy(10, 100_000).unwrap_err().to_string();
///   if backtrace_enabled() {
///       assert!(err_msg.starts_with("failed to compute pow(100000, 2): u32 overflow\nstack backtrace:\n"));
///   } else {
///       assert_eq!(err_msg, "failed to compute pow(100000, 2): u32 overflow");
///   }
///   ```
/// * Both method style (`a.cadd(b)`) and function style (`cadd(a, b)`) APIs are available.
///   Free functions can make expressions more readable when there are multiple levels of nesting:
///   ```
///   # use cadd::ops::{cadd, cmul};
///   fn f1(a1: u32, b1: u32, a2: u32, b2: u32) -> cadd::Result<u32> {
///       cadd(
///           cmul(a1, b1)?,
///           cmul(a2, b2)?,
///       )
///   }
///   ```
///   Method style may be preferred for better chaining:
///   ```
///   # use cadd::ext::U32Ext;
///   fn f2(a1: u32, b1: u32, c1: u32, d1: u32) -> cadd::Result<u32> {
///       a1.cadd(b1)?
///          .cmul(c1)?
///          .cdiv(d1)
///   }
///   ```
/// * Function names are relatively short, so it's easier to keep the code readable.
///   The names may look a bit cryptic at first, but there is really only one rule to remember:
///   every function name is just the name of the unchecked alternative ([`add`](std::ops::Add::add),
///   [`pow`](u32::pow), [`ilog`](u32::ilog), etc) with the "c" suffix that stands for "checked".
///
/// See also: [crate level documentation](crate).
pub mod ops;

/// Extension traits for enhanced checked arithmetics
pub mod ext;

pub mod prelude;

mod private {
    pub trait Sealed: Sized {}
}

pub use crate::error::Error;

use core::{fmt::Debug, num::NonZero};

/// `Result` with error type defaulting to `cadd::Error`.
pub type Result<T, E = Error> = core::result::Result<T, E>;

// TODO: String <-> OsString conversions
// TODO: &[u8] -> String utf8 conversions
// TODO: Path conversions
// TODO: ops for non-nan and finite floats (real_float crate?)
// TODO: readme

struct MaybeParens<T>(T);

impl<T: Debug + IsNegative> Debug for MaybeParens<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_negative() {
            write!(f, "({:?})", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

trait IsNegative {
    fn is_negative(&self) -> bool;
}
macro_rules! impl_is_negative_false {
    ($t:ty) => {
        impl IsNegative for $t {
            fn is_negative(&self) -> bool {
                false
            }
        }
    };
}

macro_rules! impl_is_negative {
    ($t:ty) => {
        impl IsNegative for $t {
            fn is_negative(&self) -> bool {
                *self < 0
            }
        }
    };
}

macro_rules! impl_is_negative_non_zero {
    ($t:ty) => {
        impl IsNegative for $t {
            fn is_negative(&self) -> bool {
                const ONE: $t = <$t>::new(1).unwrap();
                *self < ONE
            }
        }
    };
}

impl_is_negative!(i8);
impl_is_negative!(i16);
impl_is_negative!(i32);
impl_is_negative!(i64);
impl_is_negative!(i128);
impl_is_negative!(isize);
impl_is_negative_non_zero!(NonZero<i8>);
impl_is_negative_non_zero!(NonZero<i16>);
impl_is_negative_non_zero!(NonZero<i32>);
impl_is_negative_non_zero!(NonZero<i64>);
impl_is_negative_non_zero!(NonZero<i128>);
impl_is_negative_non_zero!(NonZero<isize>);

impl_is_negative_false!(u8);
impl_is_negative_false!(u16);
impl_is_negative_false!(u32);
impl_is_negative_false!(u64);
impl_is_negative_false!(u128);
impl_is_negative_false!(usize);
impl_is_negative_false!(NonZero<u8>);
impl_is_negative_false!(NonZero<u16>);
impl_is_negative_false!(NonZero<u32>);
impl_is_negative_false!(NonZero<u64>);
impl_is_negative_false!(NonZero<u128>);
impl_is_negative_false!(NonZero<usize>);
impl_is_negative_false!(std::time::Duration);
