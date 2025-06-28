#![no_std]

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

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait ToNonZero {
    type NonZero;
    fn to_non_zero(self) -> Result<Self::NonZero>;
}
pub fn non_zero<T: ToNonZero>(a: T) -> Result<T::NonZero> {
    a.to_non_zero()
}

macro_rules! impl_to_non_zero {
    ($($ty:ident,)*) => {
        $(
            impl $crate::ToNonZero for $ty {
                type NonZero = ::core::num::NonZero<$ty>;
                fn to_non_zero(self) -> $crate::Result<Self::NonZero> {
                    ::core::num::NonZero::new(self).ok_or_else(|| $crate::Error::new("unexpected zero value".into()))
                }
            }
        )*
    }
}

impl_to_non_zero!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize,);

// TODO: String <-> OsString conversions
// TODO: &[u8] -> String utf8 conversions
// TODO: Path conversions
// TODO: ops for non-nan and finite floats (real_float crate?)
// TODO: readme
