#![no_std]
#![warn(missing_docs)]

//! # `cadd`: painless checked arithmetics and conversions
//!
//! Features:
//! * [`ops`]: checked arithmetics with `Result`, informative errors, and backtrace.
//! * [`Cinto`](convert::Cinto): `TryInto` alternative for type conversions with better error messages and backtrace.
//!   Works for integer types, other primitives, arrays, and string-like types.
//! * [`SaturatingInto`](convert::SaturatingInto):
//!   infallible number conversion that returns the closest valid value.
//! * [`non_zero`](convert::non_zero) and [`to_non_zero()`](convert::ToNonZero):
//!   conversion to [`NonZero`] with `Result`, informative errors, and backtrace.
//! * <code>.[into_type](https://docs.rs/cadd/latest/cadd/convert/trait.IntoType.html)::&lt;T&gt;()</code>
//!   as an alternative to `into()` and `try_into()` without type inference errors.
//! * `no_std` support (`alloc` is still required).
//!
//! # Introduction
//!
//! Rust generally offers amazing capability to create safe, correct programs. However, since it is still
//! a systems programming language, there are still a few aspects where the "default" or the easiest way
//! to do something is the cheapest, but not always the correct one. One of the most ubiquitous example
//! is the behavior of arithmetic operations on integer types and casts between them:
//!
//! * Many arithmetic operators (`+`, `-`, `*`, ...) and some functions (like `.pow()`) are
//!   *unchecked* in Release mode. This means that they can silently overflow and wrap, producing a value
//!   that is not the true output of the operation.
//! * Some of the operations (like `a / b` and `a.ilog(b)`) will panic if their preconditions
//!   are unmet.
//! * `as` conversion between integer types will silently truncate values as necessary to fit the output type.
//!   It's also capable of reinterpreting signed values as unsigned and vice versa, producing a value that
//!   is not equal to the input.
//!
//! Sometimes this behavior is fine. However, there are many cases where performance is not as important
//! as correctness. For most high level applications, calculating the values precisely is much more important,
//! as these values may represent things like amount of money or amount of physical goods.
//! Arguably, **checked math and conversions should be the default**. A faster, but potentially wrapping or
//! truncating alternative should only be chosen in specific cases, after ensuring that it will not cause
//! a logic bug.
//!
//! Rust's core library provides very good APIs for checked operations. The only problem with them is that
//! they require much more verbose code. Compare a simple `a + b` with a properly done checked addition:
//! `a.checked_add(b).ok_or(Error::Overflow)?`. Any non-trivial code quickly becomes a wall of checked
//! calls and numerous error conversions.
//!
//! This is where `cadd` comes in. This crate aims to provide a set of APIs that are (almost)
//! as easy to use as the basic operations, while being a safe, correct default. These functions
//! never wrap, truncate, reinterpret, or panic.
//!
//! Any errors returned by `cadd` will contain as much information as possible (to a reasonable limit)
//! and will also capture a backtrace if it's enabled. This makes it easy to integrate it into
//! any error handling approach, be it custom error types or "catch-all" errors like `anyhow`.
//!
//! # Example
//!
//! The easiest, but error-prone approach:
//! ```
//! # let price = 1_u64;
//! # let discount_rate = 1_u64;
//! # let price = 1_u64;
//! let amount = price - discount_rate * price / 100;
//! let output = amount as u32;
//! ```
//! A safe approach that uses `std` checked functions and returns a non-informative error on overflow:
//! ```
//! # #[derive(Debug, thiserror::Error)]
//! # enum Error { #[error("overflow")] Overflow }
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let price = 1_u64;
//! # let discount_rate = 1_u64;
//! # let price = 1_u64;
//! let amount = discount_rate
//!     .checked_mul(price)
//!     .and_then(|v| v.checked_div(100))
//!     .and_then(|v| price.checked_sub(v))
//!     .ok_or_else(|| Error::Overflow)?;
//! let output: u32 = amount.try_into().map_err(|_| Error::Overflow)?;
//! # Ok(())
//! # }
//! ```
//! A safe and concise approach with extension traits from [`cadd::ext`](ext)
//! and functions from [`cadd::ops`](ops),
//! returning an informative error:
//! ```
//! # #[derive(Debug, thiserror::Error)]
//! # enum Error { #[error("overflow")] Overflow }
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let price = 1_u64;
//! # let discount_rate = 1_u64;
//! # let price = 1_u64;
//! use cadd::{ext::U64Ext, ops::csub, convert::IntoType};
//!
//! let amount = csub(price, discount_rate.cmul(price)?.cdiv(100)?)?;
//! let output = amount.cinto_type::<u32>()?;
//! # Ok(())
//! # }
//! ```
//! Errors may look like this:
//! ```text
//! failed to compute 20 - 40: u64 overflow
//! stack backtrace:
//!    0: std::backtrace_rs::backtrace::libunwind::trace
//! ...
//! ```
//!
//! # Recommended clippy lints
//!
//! In order to eliminate unintended side effects, wraps, truncations, and signed-vs-unsigned reinterpretation,
//! it's recommended to set the following clippy lints to `warn` level:
//!
//! - `arithmetic_side_effects`
//! - `cast_possible_wrap`
//! - `cast_precision_loss`
//! - `cast_sign_loss`

extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate std;

mod convert_impls;
mod error;

pub mod convert;

/// Enhanced checked arithmetics.
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
/// [`checked_pow`](u32::checked_pow), etc.
///
/// Function names are shorter, so it's easier to keep the code readable.
/// There is only one rule to remember: replace "checked_" with "c" to get the corresponding improved function.
/// `checked_add` becomes `cadd`, `checked_pow` becomes `cpow`, and so on.
///
/// All functions from this crate return `Result` instead of `Option`, enabling the use of `?`
/// in functions returning `Result`.
/// ```
/// use cadd::{ops::{Cpow, cmul}, ext::U32Ext};
///
/// fn kinetic_energy(mass: u32, velocity: u32) -> cadd::Result<u32> {
///     cmul(mass, velocity.cpow(2)?)?.cdiv(2)
/// }
/// ```
/// In case of an overflow or another failure, the returned error contains a meaningful error message
/// and a backtrace (if enabled):
/// ```
/// # use cadd::{ops::cmul, ext::U32Ext};
/// # fn kinetic_energy(mass: u32, velocity: u32) -> cadd::Result<u32> {
/// #     cmul(mass, velocity.cpow(2)?)?.cdiv(2)
/// # }
/// # fn backtrace_enabled() -> bool {
/// #     match std::env::var("RUST_LIB_BACKTRACE") {
/// #         Ok(s) => s != "0",
/// #         Err(_) => match std::env::var("RUST_BACKTRACE") {
/// #             Ok(s) => s != "0",
/// #             Err(_) => false,
/// #         },
/// #     }
/// # }
/// let err_msg = kinetic_energy(10, 100_000).unwrap_err().to_string();
/// if backtrace_enabled() {
///     assert!(err_msg.starts_with(
///         "failed to compute pow(100000, 2): u32 overflow\nstack backtrace:\n"
///     ));
/// } else {
///     assert_eq!(err_msg, "failed to compute pow(100000, 2): u32 overflow");
/// }
/// ```
/// Both method style (`a.cadd(b)`) and function style (`cadd(a, b)`) APIs are available.
/// Free functions can make expressions more readable when there are multiple levels of nesting:
/// ```
/// use cadd::ops::{cadd, cmul};
///
/// fn f1(a1: u32, b1: u32, a2: u32, b2: u32) -> cadd::Result<u32> {
///     cadd(
///         cmul(a1, b1)?,
///         cmul(a2, b2)?,
///     )
/// }
/// ```
/// On the other hand, method style may be preferred for better chaining:
/// ```
/// use cadd::ops::{Cadd, Cmul, Cdiv};
///
/// fn f2(a1: u32, b1: u32, c1: u32, d1: u32) -> cadd::Result<u32> {
///     a1.cadd(b1)?
///        .cmul(c1)?
///        .cdiv(d1)
/// }
/// ```
/// Assignment functions ([`cadd_assign`](ops::Cadd::cadd_assign), [`csub_assign`](ops::Csub::csub_assign), etc.)
/// are also provided.
///
/// For binary operations, the traits use an associated type to specify the type of the second argument.
/// It provides better type inference: when the type of the first argument is known, the type of the second argument
/// becomes known as well.
///
/// See also: [crate level documentation](crate).
pub mod ops;

pub mod prelude;

pub use crate::error::Error;

use core::{fmt::Debug, num::NonZero};

/// `Result` with error type defaulting to `cadd::Error`.
pub type Result<T, E = Error> = core::result::Result<T, E>;

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
impl_is_negative_false!(core::time::Duration);
