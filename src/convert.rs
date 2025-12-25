//! Converting values to another type.

/// Extention trait that enables `.into_type::<T>()` syntax. Also works for
/// [`cinto`](Cinto),
/// [`try_into`](TryInto),
/// [`saturating_into`](SaturatingInto).
///
/// When you replace unchecked type casts (e.g. `number as u32`) with an infallible conversion
/// (`number.into()`) or a fallible conversion (`number.try_into()?`), you may often encounter
/// type inference errors if the context doesn't have enough information about the target type:
/// ```
/// fn f1(input: u32) -> u64 {
///     10 + (input as u64) // Compiles
/// }
/// ```
/// ```compile_fail
/// fn f1(input: u32) -> Result<u64, std::num::TryFromIntError> {
///     let a = 10 + input.try_into()?; // Doesn't compile
///     Ok(a)
/// }
/// ```
///  The easiest way to solve it in `std` is to use `From` or `TryFrom` instead
/// so that you can specify the target type:
/// ```
/// fn f1(input: u32) -> Result<u64, std::num::TryFromIntError> {
///     let a = 10 + u64::try_from(input)?; // Compiles
///     Ok(a)
/// }
/// ```
/// This can cause unnecessary friction because it requires some rearrangement of the code and reduces its
/// readability. The `IntoType` trait provides an alternative way to do it:
/// ```
/// use cadd::convert::IntoType;
/// fn f1(input: u32) -> Result<u64, std::num::TryFromIntError> {
///     let a = 10 + input.try_into_type::<u64>()?; // Compiles
///     Ok(a)
/// }
/// ```
///
/// This trait is implemented for all types. However, each method has its own type bound that requires
/// the corresponding conversion trait to be implemented.
pub trait IntoType {
    /// An alternative to [`.into()`](std::convert::Into) that allows specifying the target type.
    /// ```
    /// use cadd::convert::IntoType;
    /// assert_eq!(2u32.into_type::<u64>(), 2);
    /// ```
    #[inline]
    fn into_type<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    /// An alternative to [`.try_into()`](std::convert::TryInto) that allows specifying the target type.
    /// ```
    /// use cadd::convert::IntoType;
    /// assert!((-2i32).try_into_type::<u32>().is_err());
    /// assert_eq!(2i32.try_into_type::<u32>().unwrap(), 2);
    /// ```
    #[inline]
    fn try_into_type<T>(self) -> Result<T, Self::Error>
    where
        Self: TryInto<T>,
    {
        self.try_into()
    }

    /// An alternative to [`.cinto()`](Cinto) that allows specifying the target type.
    /// ```
    /// use cadd::convert::IntoType;
    /// assert!((-2i32).cinto_type::<u32>().is_err());
    /// assert_eq!(2i32.cinto_type::<u32>().unwrap(), 2);
    /// ```
    #[inline]
    fn cinto_type<T>(self) -> Result<T, Self::Error>
    where
        Self: Cinto<T>,
    {
        self.cinto()
    }

    /// An alternative to [`.saturating_into()`](SaturatingInto) that allows specifying the target type.
    /// ```
    /// use cadd::convert::IntoType;
    /// assert_eq!(300_u32.saturating_into_type::<u8>(), 255);
    /// ```
    #[inline]
    fn saturating_into_type<T>(self) -> T
    where
        Self: SaturatingInto<T>,
    {
        self.saturating_into()
    }
}

impl<T: ?Sized> IntoType for T {}

/// Checked conversion from `Input` to `Self`.
///
/// This is semantically the same as [`TryFrom`]. However, `Cfrom`
/// aims to provide a rich error message, as opposed to many implementations of `TryFrom` in `std`
/// that provide minimal informations in errors.
///
/// [`Cinto`] trait provides an alternative way to do the same conversion.
/// Similar to `TryFrom`, it's recommended to always implement `Cfrom` instead of [`Cinto`].
/// The corresponding `Cinto` implementation will be covered by the blanket impl.
///
/// Notable implementations:
///
/// * Fallible conversions between integer types:
///   ```
///   use cadd::convert::{Cfrom, Cinto, IntoType};
///
///   // Output type can be inferred from context:
///   let a = 15_i32;
///   let b: u32 = a.cinto().unwrap();
///   assert_eq!(b, 15);
///
///   // It's also possible to specify output type explicitly:
///   let c = a.cinto_type::<u32>().unwrap();
///   assert_eq!(c, 15);
///
///   // The error contains the input value, input type and output type:
///   let d = -15_i32;
///   let err = d.cinto_type::<u32>().unwrap_err();
///   assert!(err.to_string().contains(
///       "failed to convert value -15 from i32 to u32: value is out of bounds"
///   ));
///
///   // Using `Cfrom` directly:
///   let e = u32::cfrom(a).unwrap();
///   assert_eq!(e, 15);
///   ```
/// * Conversions from slices to fixed arrays:
///   ```
///   # use cadd::convert::{Cfrom, Cinto, IntoType};
///   let a: &[u32] = &[1, 2, 3, 4];
///   let b: &[u32; 4] = a.cinto().unwrap();
///   assert!(
///       a.cinto_type::<&[u32; 5]>().unwrap_err().to_string().contains(
///           "expected 5 items, got [1, 2, 3, 4] (4 items)"
///       )
///   );
///   ```
/// * Conversions from `CString`, `&CStr`, `OsString`, `&OsStr`, `PathBuf`, `&Path` to `String` and `&str`:
///   ```
///   # use {cadd::convert::{Cfrom, Cinto, IntoType}, std::ffi::CString};
///   let a = CString::from_vec_with_nul(vec![104, 101, 108, 108, 111, 0]).unwrap();
///   assert_eq!(a.cinto_type::<String>().unwrap(), "hello");
///
///   let a2 = CString::from_vec_with_nul(vec![104, 128, 108, 108, 111, 0]).unwrap();
///   assert!(
///       a2.clone().cinto_type::<String>().unwrap_err().to_string().contains(
///           "failed to convert bytes to string: \
///           invalid utf-8 sequence of 1 bytes from index 1; \
///           input: [104, 128, 108, 108, 111] (5 bytes); \
///           input as lossy utf-8: \"h�llo\""
///       )
///   );
///   ```
#[allow(missing_docs)]
pub trait Cfrom<Input>: Sized {
    type Error;
    fn cfrom(from: Input) -> Result<Self, Self::Error>;
}

/// Checked conversion from `Self` to `Output`.
///
/// This trait is automatically implemented when `Output` implements `Cfrom<Self>`.
///
/// See [`Cfrom`] for main documentation.
///
/// In order to help with type inference,
/// the [`IntoType`] extension trait provides `.cinto_type::<T>()` syntax.
#[allow(missing_docs)]
pub trait Cinto<Output>: Sized {
    type Error;
    fn cinto(self) -> Result<Output, Self::Error>;
}

impl<Input, Output> Cinto<Output> for Input
where
    Output: Cfrom<Input>,
{
    type Error = <Output as Cfrom<Input>>::Error;
    #[inline]
    fn cinto(self) -> Result<Output, Self::Error> {
        Output::cfrom(self)
    }
}

/// Saturating conversion of a number from `Input` to `Self`.
///
/// If the value being converted is out of bounds for the target type,
/// the closest representable value is returned. Consequently, if the value is out of bounds,
/// this conversion always returns `Self::MIN` or `Self::MAX`.
/// ```
/// use cadd::convert::SaturatingFrom;
///
/// assert_eq!(u8::saturating_from(300_u32), 255);
/// assert_eq!(u8::saturating_from(200_u32), 200);
/// assert_eq!(u8::saturating_from(-300_i32), 0);
/// assert_eq!(i8::saturating_from(-300_i32), -128);
/// ```
/// [`SaturatingInto`] trait provides an alternative way to do the same conversion.
/// Similar to [`TryFrom`], it's recommended to always implement
/// `SaturatingFrom` instead of [`SaturatingInto`](Cinto).
/// The corresponding `SaturatingInto` implementation will be covered by the blanket impl.
pub trait SaturatingFrom<Input>: Sized {
    #[allow(missing_docs)]
    fn saturating_from(from: Input) -> Self;
}

/// Saturating conversion of a number from `Self` to `Output`.
///
/// This trait is automatically implemented when `Output` implements `SaturatingFrom<Self>`.
///
/// See [`SaturatingFrom`] for main documentation.
///
/// In order to help with type inference,
/// the [`IntoType`] extension trait provides `.saturating_into_type::<T>()` syntax.
///
/// ```
/// use cadd::convert::{SaturatingInto, IntoType};
///
/// let v: u8 = 300_u32.saturating_into();
/// assert_eq!(v, 255);
/// // Or with `IntoType` extension trait:
/// assert_eq!(300_u32.saturating_into_type::<u8>(), 255);
///
/// // More examples:
/// assert_eq!(200_u32.saturating_into_type::<u8>(), 200);
/// assert_eq!((-300_i32).saturating_into_type::<u8>(), 0);
/// assert_eq!((-300_i32).saturating_into_type::<i8>(), -128);
/// ```
pub trait SaturatingInto<Output>: Sized {
    #[allow(missing_docs)]
    fn saturating_into(self) -> Output;
}

impl<Input, Output> SaturatingInto<Output> for Input
where
    Output: SaturatingFrom<Input>,
{
    #[inline]
    fn saturating_into(self) -> Output {
        Output::saturating_from(self)
    }
}

/// Conversion from an integer type to the corresponding [`NonZero`](std::num::NonZero) type.
///
/// If the value is zero, it returns an error with a backtrace.
#[allow(missing_docs)]
pub trait ToNonZero {
    type Error;
    type NonZero;
    fn to_non_zero(self) -> Result<Self::NonZero, Self::Error>;
}

/// Conversion from an integer type to the corresponding [`NonZero`](std::num::NonZero) type.
///
/// If the value is zero, it returns an error with a backtrace.
#[inline]
pub fn non_zero<T: ToNonZero>(a: T) -> crate::Result<T::NonZero, T::Error> {
    a.to_non_zero()
}

macro_rules! impl_to_non_zero {
    ($($ty:ident,)*) => {
        $(
            impl $crate::convert::ToNonZero for $ty {
                type Error = $crate::Error;
                type NonZero = ::core::num::NonZero<$ty>;
                #[inline]
                fn to_non_zero(self) -> $crate::Result<Self::NonZero> {
                    ::core::num::NonZero::new(self).ok_or_else(|| $crate::Error::new("unexpected zero value".into()))
                }
            }
        )*
    }
}

impl_to_non_zero!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize,);
