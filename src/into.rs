use crate::Result;

/// Extention trait that enables `.into_type::<T>()` and `.cinto_type::<T>()` syntax.
pub trait IntoType {
    /// An alternative to `.into()` that allows specifying the target type.
    fn into_type<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    /// An alternative to `.cinto()` that allows specifying the target type.
    fn cinto_type<T>(self) -> Result<T>
    where
        Self: Cinto<T>,
    {
        self.cinto()
    }

    /// An alternative to `.saturating_into()` that allows specifying the target type.
    fn saturating_into_type<T>(self) -> T
    where
        Self: SaturatingInto<T>,
    {
        self.saturating_into()
    }
}

impl<T> IntoType for T {}

pub trait Cfrom<F>: Sized {
    fn cfrom(from: F) -> Result<Self>;
}

pub trait Cinto<I>: Sized {
    fn cinto(self) -> Result<I>;
}

impl<F, I> Cinto<I> for F
where
    I: Cfrom<F>,
{
    fn cinto(self) -> Result<I> {
        I::cfrom(self)
    }
}

pub trait SaturatingFrom<F>: Sized {
    fn saturating_from(from: F) -> Self;
}

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
