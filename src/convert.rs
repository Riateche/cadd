/// Extention trait that enables `.into_type::<T>()` syntax. Also works for `cinto`, `try_into`, `saturating_into`.
///
/// When you replace unchecked type casts (e.g. `number as u32`) with an infallible conversion
/// (`number.into()`) or a fallible conversion (`number.try_into()?`), you may often encounter
/// type inference errors. The easiest way to solve it is to use `From` or `TryFrom` instead
/// so that you can specify the target type: `u32::from(number)`, `u32::try_from(number)?`.
///
/// However, if you don't want to rewrite your whole expression, you can use this extension trait
/// to specify the target type at the end: `number.into_type::<u32>()`, `number.cinto_type::<u32>()?`.
///
/// This trait is implemented for all types. However, each method has its own type bound that requires
/// the corresponding conversion trait to be implemented.
pub trait IntoType {
    /// An alternative to [`.into()`](std::convert::Into) that allows specifying the target type.
    fn into_type<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    /// An alternative to [`.try_into()`](std::convert::TryInto) that allows specifying the target type.
    fn try_into_type<T>(self) -> Result<T, Self::Error>
    where
        Self: TryInto<T>,
    {
        self.try_into()
    }

    /// An alternative to [`.cinto()`](Cinto) that allows specifying the target type.
    fn cinto_type<T>(self) -> Result<T, Self::Error>
    where
        Self: Cinto<T>,
    {
        self.cinto()
    }

    /// An alternative to [`.saturating_into()`](SaturatingInto) that allows specifying the target type.
    fn saturating_into_type<T>(self) -> T
    where
        Self: SaturatingInto<T>,
    {
        self.saturating_into()
    }
}

impl<T> IntoType for T {}

/// Checked conversion from `F` to `Self`.
///
/// This is semantically the same as [TryFrom](std::convert::TryFrom). However, `Cfrom`
/// aims to provide a rich error message, as opposed to many implementations of `TryFrom` in `std`
/// that provide minimal informations in errors.
///
/// Similar to `TryFrom`, it's recommended to always implement `Cfrom` instead of [`Cinto`](Cinto).
/// The corresponding `Cinto` implementation will be covered by the blanket impl.
pub trait Cfrom<F>: Sized {
    type Error;
    fn cfrom(from: F) -> Result<Self, Self::Error>;
}

/// Checked conversion from `Self` to `I`.
///
/// This trait is automatically implemented when `I` implements `Cfrom<Self>`.
///
/// See [`Cfrom`](Cfrom) for main documentation.
///
/// In order to help with type inference,
/// the [`IntoType`](IntoType) extension trait provides `.cinto_type::<T>()` syntax.
pub trait Cinto<I>: Sized {
    type Error;
    fn cinto(self) -> Result<I, Self::Error>;
}

impl<F, I> Cinto<I> for F
where
    I: Cfrom<F>,
{
    type Error = <I as Cfrom<F>>::Error;
    fn cinto(self) -> Result<I, Self::Error> {
        I::cfrom(self)
    }
}

/// Saturating conversion of a number from `F` to `Self`.
///
/// If the value being converted is out of bounds for the target type,
/// the closest representable value is returned. Consequently, if the value is out of bounds,
/// this conversion always returns `Self::MIN` or `Self::MAX`.
///
/// Similar to [`TryFrom`](std::convert::TryFrom), it's recommended to always implement
/// `SaturatingFrom` instead of [`SaturatingInto`](Cinto).
/// The corresponding `SaturatingInto` implementation will be covered by the blanket impl.
pub trait SaturatingFrom<F>: Sized {
    fn saturating_from(from: F) -> Self;
}

/// Saturating conversion of a number from `Self` to `I`.
///
/// This trait is automatically implemented when `I` implements `SaturatingFrom<Self>`.
///
/// See [`SaturatingFrom`](SaturatingFrom) for main documentation.
///
/// In order to help with type inference,
/// the [`IntoType`](IntoType) extension trait provides `.saturating_into_type::<T>()` syntax.
pub trait SaturatingInto<I>: Sized {
    fn saturating_into(self) -> I;
}

impl<F, I> SaturatingInto<I> for F
where
    I: SaturatingFrom<F>,
{
    fn saturating_into(self) -> I {
        I::saturating_from(self)
    }
}
