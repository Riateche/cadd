//! Checked operations on numbers.
//!
//! Many operators on integer primitives (`a + b`, `a / b`, etc) and associated functions (`a.pow(b)`, `a.ilog(b)`, etc)
//! can overflow or fail under certain conditions. With debug assertions enabled (default when building in debug mode),
//! any such failures will be caught and converted into a panic. With debug assertions disabled
//! (default when building in release mode), some failures (like division by zero) will still result in a panic,
//! and overflows will silently return an overflown value, which is often an unexpected and incorrect result.
//! Therefore, to write the code that returns correct values on every valid input and correctly handles every invalid input,
//! it's highly recommended to use checked alternatives.
//!
//! Rust offers great capabilities for checked arithmetics. For every operation that can overflow or otherwise fail,
//! the standard library contains a function with the `checked_` prefix that returns `Option`. For example:
//! ```
//! assert_eq!(300_u32.checked_add(200_u32), Some(500));
//! assert_eq!(3_000_000_000_u32.checked_add(2_000_000_000_u32), None);
//! ```
//! However, writing code that uses checked functions can be quite cumbersome, especially if you use `Result`
//! throughout the code:
//! ```
//! # use std::error::Error;
//! fn calculate_trajectory(mass: u32, velocity: u32) -> Result<(), Box<dyn Error>> {
//!     let kinetic_energy = velocity
//!         .checked_pow(2)
//!         .and_then(|v| mass.checked_mul(v))
//!         .and_then(|v| v.checked_div(2))
//!         .ok_or_else(|| "mass or velocity too large")?;
//!     //...
//!     Ok(())
//! }
//! ```
//! It can be improved by moving all arithmetics into functions that return `Option` so that you can use `?`
//! for early returns, but it requires even more restructuring of the code.
//!
//! This crate offers a set of traits and functions for easy handling of checked arithmetics.
//! These traits and functions are modelled after the `checked_*` family of functions provided by the standard
//! library for primitive numeric types, such as [`checked_add`](u32::checked_add),
//! [`checked_pow`](u32::checked_pow), etc. These traits and functions offer a number of benefits
//! over the standard library functions:
//!
//! * They return `Result` instead of `Option`, enabling the use of `?` in functions returning `Result`.
//!   ```
//!   use cadd::ops::{Cpow, Cdiv, cmul};
//!
//!   fn kinetic_energy(mass: u32, velocity: u32) -> cadd::Result<u32> {
//!       cmul(mass, velocity.cpow(2)?)?.cdiv(2)
//!   }
//!   ```
//! * The error values they return provide a meaningful error message and a backtrace:
//!   ```
//!   # use cadd::ops::{Cpow, Cdiv, cmul};
//!   # fn kinetic_energy(mass: u32, velocity: u32) -> cadd::Result<u32> {
//!   #     cmul(mass, velocity.cpow(2)?)?.cdiv(2)
//!   # }
//!   # fn backtrace_enabled() -> bool {
//!   #     match std::env::var("RUST_LIB_BACKTRACE") {
//!   #         Ok(s) => s != "0",
//!   #         Err(_) => match std::env::var("RUST_BACKTRACE") {
//!   #             Ok(s) => s != "0",
//!   #             Err(_) => false,
//!   #         },
//!   #     }
//!   # }
//!   let err_msg = kinetic_energy(10, 100_000).unwrap_err().to_string();
//!   if backtrace_enabled() {
//!       assert!(err_msg.starts_with("overflow: pow(100000, 2)\nstack backtrace:\n"));
//!   } else {
//!       assert_eq!(err_msg, "overflow: pow(100000, 2)");
//!   }
//!   ```
//! * Both method style (`a.cadd(b)`) and function style (`cadd(a, b)`) APIs are available.
//!   Free functions can make expressions more readable when there are multiple levels of nesting:
//!   ```
//!   # use cadd::ops::{cadd, cmul};
//!   fn f1(a1: u32, b1: u32, a2: u32, b2: u32) -> cadd::Result<u32> {
//!       cadd(
//!           cmul(a1, b1)?,
//!           cmul(a2, b2)?,
//!       )
//!   }
//!   ```
//!   Method style may be preferred for better chaining:
//!   ```
//!   # use cadd::ops::{Cadd, Cmul, Cdiv};
//!   fn f2(a1: u32, b1: u32, c1: u32, d1: u32) -> cadd::Result<u32> {
//!       a1.cadd(b1)?
//!          .cmul(c1)?
//!          .cdiv(d1)
//!   }
//!   ```
//! * Function names are relatively short, so it's easier to keep the code readable.
//!   The names may look a bit cryptic at first, but there is really only one rule to remember:
//!   every function name is just the name of the unchecked alternative ([`add`](std::ops::Add::add),
//!   [`pow`](u32::pow), [`ilog`](u32::ilog), etc) with the "c" suffix that stands for "checked".

macro_rules! declare_binary_trait {
    ($trait_:ident, $trait_fn:ident, $doc:literal) => {
        #[doc = $doc]
        pub trait $trait_<Other = Self>: Sized {
            type Error;
            type Output;
            fn $trait_fn(self, b: Other) -> Result<Self::Output, Self::Error>;
        }

        #[doc = $doc]
        pub fn $trait_fn<T1, T2>(a: T1, b: T2) -> Result<T1::Output, T1::Error>
        where
            T1: $trait_<T2>,
        {
            a.$trait_fn(b)
        }
    };
}

macro_rules! declare_unary_trait {
    ($trait_:ident, $trait_fn:ident, $doc:literal) => {
        #[doc = $doc]
        pub trait $trait_: Sized {
            type Error;
            type Output;
            fn $trait_fn(self) -> Result<Self::Output, Self::Error>;
        }

        #[doc = $doc]
        pub fn $trait_fn<T1>(value: T1) -> Result<T1::Output, T1::Error>
        where
            T1: $trait_,
        {
            value.$trait_fn()
        }
    };
}

declare_binary_trait!(
    Cadd,
    cadd,
    "Addition: `a + b`. Returns an error on overflow."
);
declare_binary_trait!(
    Csub,
    csub,
    "Subtraction: `a - b`. Returns an error on overflow."
);
declare_unary_trait!(Cneg, cneg, "Negation: `-a`. Returns an error on overflow.");
declare_binary_trait!(
    Cmul,
    cmul,
    "Multiplication: `a * b`. Returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    Cdiv,
    cdiv,
    "Division: `a / b`. Returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    CdivEuclid,
    cdiv_euclid,
    "Euclidian division. Returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    Crem,
    crem,
    "Remainder: `a % b`. Returns an error on overflow or if the divisor is zero."
);
declare_binary_trait!(
    CremEuclid,
    crem_euclid,
    "Euclidian reminder. Returns an error on overflow or if the divisor is zero."
);

declare_binary_trait!(
    CILog,
    cilog,
    "Logarithm: <code>log<sub>b</sub> a</code>. Returns an error if the number is negative or zero, or if the base is less than 2."
);
declare_unary_trait!(
    CILog2,
    cilog2,
    "Base 2 logarithm: `ln a`. Returns an error if the number is negative or zero."
);
declare_unary_trait!(
    CILog10,
    cilog10,
    "Base 10 logarithm: <code>log<sub>10</sub> a</code>. Returns an error if the number is negative or zero."
);
declare_binary_trait!(
    Cshl,
    cshl,
    "Shift left: `a << b`. Returns an error if `b` is greater or equal to the number of bits in the type."
);
declare_binary_trait!(
    Cshr,
    cshr,
    "Shift right: `a >> b`. Returns an error if `b` is greater or equal to the number of bits in the type."
);
declare_binary_trait!(
    Cpow,
    cpow,
    "Exponentiation: <code>a<sup>b</sup></code>. Returns an error on overflow."
);
declare_unary_trait!(
    Cabs,
    cabs,
    "Absolute value: `|a|` (signed types only). Returns an error if `a == MIN`."
);
declare_unary_trait!(
    Cisqrt,
    cisqrt,
    "Square root: `√a` (signed types only). Returns an error if `a` is negative."
);
declare_binary_trait!(
    CnextMultipleOf,
    cnext_multiple_of,
    "Next multiple of `b`. Returns an error on overflow or if `b` is zero."
);
declare_unary_trait!(
    CnextPowerOfTwo,
    cnext_power_of_two,
    "Next power of 2. Returns an error on overflow."
);
