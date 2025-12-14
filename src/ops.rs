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
//!
//! See also: [crate level documentation](crate).

macro_rules! impl_fn_literal {
    (cadd) => {
        "checked_add"
    };
    (csub) => {
        "checked_sub"
    };
    (cneg) => {
        "checked_neg"
    };
    (cmul) => {
        "checked_mul"
    };
    (cdiv) => {
        "checked_div"
    };
    (cdiv_euclid) => {
        "checked_div_euclid"
    };
    (crem) => {
        "checked_rem"
    };
    (crem_euclid) => {
        "checked_rem_euclid"
    };
    (cilog) => {
        "checked_ilog"
    };
    (cilog2) => {
        "checked_ilog2"
    };
    (cilog10) => {
        "checked_ilog10"
    };
    (cshl) => {
        "checked_shl"
    };
    (cshr) => {
        "checked_shr"
    };
    (cpow) => {
        "checked_pow"
    };
    (cabs) => {
        "checked_abs"
    };
    (cisqrt) => {
        "checked_isqrt"
    };
    (cnext_multiple_of) => {
        "checked_next_multiple_of"
    };
    (cnext_power_of_two) => {
        "checked_next_power_of_two"
    };
}

macro_rules! doc_text {
    (cadd) => {
        "Checked addition: computes `a + b`, returning an error if overflow occured."
    };
    (csub) => {
        "Checked subtraction:  computes`a - b`, returning an error if overflow occured."
    };
    (cneg) => {
        "Checked negation: computes `-a`, returning an error if overflow occured."
    };
    (cmul) => {
        "Checked multiplication: computes `a * b`, returning an error if overflow occured or if the divisor is zero."
    };
    (cdiv) => {
        "Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero."
    };
    (cdiv_euclid) => {
        "Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero."
    };
    (crem) => {
        "Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero."
    };
    (crem_euclid) => {
        "Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero."
    };
    (cilog) => {
        "Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2."
    };
    (cilog2) => {
        "Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero."
    };
    (cilog10) => {
        "Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero."
    };
    (cshl) => {
        "Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type."
    };
    (cshr) => {
        "Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type."
    };
    (cpow) => {
        "Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured."
    };
    (cabs) => {
        "Checked absolute value: computes `|a|` (signed types only), returning an error if `a == MIN`."
    };
    (cisqrt) => {
        "Checked square root: computes `√a` (signed types only), returning an error if `a` is negative."
    };
    (cnext_multiple_of) => {
        "Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero."
    };
    (cnext_power_of_two) => {
        "Checked next power of 2, returning an error if overflow occured."
    };
}

macro_rules! wrapper_doc_with_link {
    (($($ty:tt)*), ($($impl_fn:tt)*)) => {
        concat!(
            "\n\nWrapper for [`",
            $($ty)*,
            "::",
            impl_fn_literal!($($impl_fn)*),
            "`]."
        )
    };
}

pub(crate) use {doc_text, impl_fn_literal, wrapper_doc_with_link};

macro_rules! declare_binary_trait {
    ($trait_:ident, $trait_fn:ident, $doc_alias:literal) => {
        #[doc = doc_text!($trait_fn)]
        // Cannot use `impl_fn_literal!($trait_fn)` here because macros don't expand in this position.
        #[doc(alias = $doc_alias)]
        #[allow(missing_docs)]
        pub trait $trait_<Other = Self>: Sized {
            type Error;
            type Output;
            #[doc = concat!(doc_text!($trait_fn), "\n\nWrapper for `", impl_fn_literal!($trait_fn), "`.")]
            fn $trait_fn(a: Self, b: Other) -> Result<Self::Output, Self::Error>;
        }

        #[doc = concat!(doc_text!($trait_fn), "\n\nWrapper for `", impl_fn_literal!($trait_fn), "`.")]
        #[doc(alias = $doc_alias)]
        #[inline]
        pub fn $trait_fn<T1, T2>(a: T1, b: T2) -> Result<T1::Output, T1::Error>
        where
            T1: $trait_<T2>,
        {
            $trait_::$trait_fn(a, b)
        }
    };
}

macro_rules! declare_unary_trait {
    ($trait_:ident, $trait_fn:ident, $doc_alias:literal) => {
        #[doc = doc_text!($trait_fn)]
        #[doc(alias = $doc_alias)]
        #[allow(missing_docs)]
        pub trait $trait_: Sized {
            type Error;
            type Output;
            #[doc = concat!(doc_text!($trait_fn), "\n\nWrapper for `", impl_fn_literal!($trait_fn), "`.")]
            fn $trait_fn(a: Self) -> Result<Self::Output, Self::Error>;
        }

        #[doc = concat!(doc_text!($trait_fn), "\n\nWrapper for `", impl_fn_literal!($trait_fn), "`.")]
        #[doc(alias = $doc_alias)]
        #[inline]
        pub fn $trait_fn<T1>(value: T1) -> Result<T1::Output, T1::Error>
        where
            T1: $trait_,
        {
            $trait_::$trait_fn(value)
        }
    };
}

declare_binary_trait!(Cadd, cadd, "checked_add");
declare_binary_trait!(Csub, csub, "checked_sub");
declare_unary_trait!(Cneg, cneg, "checked_neg");
declare_binary_trait!(Cmul, cmul, "checked_mul");
declare_binary_trait!(Cdiv, cdiv, "checked_div");
declare_binary_trait!(CdivEuclid, cdiv_euclid, "checked_div_euclid");
declare_binary_trait!(Crem, crem, "checked_rem");
declare_binary_trait!(CremEuclid, crem_euclid, "checked_rem_euclid");

declare_binary_trait!(CILog, cilog, "checked_ilog");
declare_unary_trait!(CILog2, cilog2, "checked_ilog2");
declare_unary_trait!(CILog10, cilog10, "checked_ilog10");
declare_binary_trait!(Cshl, cshl, "checked_shl");
declare_binary_trait!(Cshr, cshr, "checked_shr");
declare_binary_trait!(Cpow, cpow, "checked_pow");
declare_unary_trait!(Cabs, cabs, "checked_abs");
declare_unary_trait!(Cisqrt, cisqrt, "checked_isqrt");
declare_binary_trait!(
    CnextMultipleOf,
    cnext_multiple_of,
    "checked_next_multiple_of"
);
declare_unary_trait!(
    CnextPowerOfTwo,
    cnext_power_of_two,
    "checked_next_power_of_two"
);
