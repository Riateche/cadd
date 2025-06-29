use {
    crate::convert::Cfrom,
    alloc::{ffi::CString, string::String, vec::Vec},
    core::{fmt::Debug, num::NonZero},
    std::ffi::OsStr,
};

// delegate to TryFrom
macro_rules! impl_cfrom {
    ($(($from:ty, $to:ty),)*) => {
        $(
            impl $crate::convert::Cfrom<$from> for $to {
                type Error = $crate::Error;
                fn cfrom(from: $from) -> $crate::Result<Self> {
                    ::core::convert::TryFrom::try_from(from)
                        .map_err(|_| $crate::Error::new(
                            ::alloc::format!(
                                "cannot convert value {:?} from {} to {}: value is out of bounds",
                                from,
                                ::core::any::type_name::<$from>(),
                                ::core::any::type_name::<$to>(),
                            )
                        ))
                }
            }
        )*
    }
}

impl_cfrom!(
    // char
    (char, u16),
    (char, u8),
    (u32, char),
    // integer -> non-zero integer
    (u8, NonZero<u8>),
    (u16, NonZero<u16>),
    (u32, NonZero<u32>),
    (u64, NonZero<u64>),
    (u128, NonZero<u128>),
    (usize, NonZero<usize>),
    (i8, NonZero<i8>),
    (i16, NonZero<i16>),
    (i32, NonZero<i32>),
    (i64, NonZero<i64>),
    (i128, NonZero<i128>),
    (isize, NonZero<isize>),
    //
);

macro_rules! impl_nonzero_int_cfrom_nonzero_int {
    ($source:ty => $($target:ty),+) => {
        impl_cfrom!(
            $((NonZero<$source>, NonZero<$target>),)*
        );
    };
}

// unsigned non-zero integer -> unsigned non-zero integer
impl_nonzero_int_cfrom_nonzero_int!(u16 => u8);
impl_nonzero_int_cfrom_nonzero_int!(u32 => u8, u16, usize);
impl_nonzero_int_cfrom_nonzero_int!(u64 => u8, u16, u32, usize);
impl_nonzero_int_cfrom_nonzero_int!(u128 => u8, u16, u32, u64, usize);
impl_nonzero_int_cfrom_nonzero_int!(usize => u8, u16, u32, u64, u128);

// signed non-zero integer -> signed non-zero integer
impl_nonzero_int_cfrom_nonzero_int!(i16 => i8);
impl_nonzero_int_cfrom_nonzero_int!(i32 => i8, i16, isize);
impl_nonzero_int_cfrom_nonzero_int!(i64 => i8, i16, i32, isize);
impl_nonzero_int_cfrom_nonzero_int!(i128 => i8, i16, i32, i64, isize);
impl_nonzero_int_cfrom_nonzero_int!(isize => i8, i16, i32, i64, i128);

// unsigned non-zero integer -> signed non-zero integer
impl_nonzero_int_cfrom_nonzero_int!(u8 => i8);
impl_nonzero_int_cfrom_nonzero_int!(u16 => i8, i16, isize);
impl_nonzero_int_cfrom_nonzero_int!(u32 => i8, i16, i32, isize);
impl_nonzero_int_cfrom_nonzero_int!(u64 => i8, i16, i32, i64, isize);
impl_nonzero_int_cfrom_nonzero_int!(u128 => i8, i16, i32, i64, i128, isize);
impl_nonzero_int_cfrom_nonzero_int!(usize => i8, i16, i32, i64, i128, isize);

// signed non-zero integer -> unsigned non-zero integer
impl_nonzero_int_cfrom_nonzero_int!(i8 => u8, u16, u32, u64, u128, usize);
impl_nonzero_int_cfrom_nonzero_int!(i16 => u8, u16, u32, u64, u128, usize);
impl_nonzero_int_cfrom_nonzero_int!(i32 => u8, u16, u32, u64, u128, usize);
impl_nonzero_int_cfrom_nonzero_int!(i64 => u8, u16, u32, u64, u128, usize);
impl_nonzero_int_cfrom_nonzero_int!(i128 => u8, u16, u32, u64, u128, usize);
impl_nonzero_int_cfrom_nonzero_int!(isize => u8, u16, u32, u64, u128, usize);

// The macros and invokations below are modelled after `std::convert::num`.

// no possible bounds violation
macro_rules! impl_cfrom_unbounded {
    ($source:ty => $($target:ty),+) => {$(
        impl $crate::convert::Cfrom<$source> for $target {
            type Error = $crate::Error;
            #[inline]
            fn cfrom(u: $source) -> $crate::Result<Self> {
                Ok(u as Self)
            }
        }

        impl $crate::convert::SaturatingFrom<$source> for $target {
            fn saturating_from(u: $source) -> Self {
                u as Self
            }
        }
    )*}
}

// only negative bounds
macro_rules! impl_cfrom_lower_bounded {
    ($source:ty => $($target:ty),+) => {$(
        impl $crate::convert::Cfrom<$source> for $target {
            type Error = $crate::Error;
            #[inline]
            fn cfrom(u: $source) -> $crate::Result<Self> {
                if u >= 0 {
                    Ok(u as Self)
                } else {
                    Err($crate::Error::new(
                        ::alloc::format!(
                            "cannot convert value {:?} from {} to {}: value is out of bounds",
                            u,
                            ::core::any::type_name::<$source>(),
                            ::core::any::type_name::<$target>(),
                        )
                    ))
                }
            }
        }

        impl $crate::convert::SaturatingFrom<$source> for $target {
            fn saturating_from(u: $source) -> Self {
                if u >= 0 {
                    u as Self
                } else {
                    0
                }
            }
        }
    )*}
}

// unsigned to signed (only positive bound)
macro_rules! impl_cfrom_upper_bounded {
    ($source:ty => $($target:ty),+) => {$(
        impl $crate::convert::Cfrom<$source> for $target {
            type Error = $crate::Error;
            #[inline]
            fn cfrom(u: $source) -> $crate::Result<Self> {
                if u > (Self::MAX as $source) {
                    Err($crate::Error::new(
                        ::alloc::format!(
                            "cannot convert value {:?} from {} to {}: value is out of bounds",
                            u,
                            ::core::any::type_name::<$source>(),
                            ::core::any::type_name::<$target>(),
                        )
                    ))
                } else {
                    Ok(u as Self)
                }
            }
        }

        impl $crate::convert::SaturatingFrom<$source> for $target {
            fn saturating_from(u: $source) -> Self {
                if u > (Self::MAX as $source) {
                    Self::MAX
                } else {
                    u as Self
                }
            }
        }
    )*}
}

// all other cases
macro_rules! impl_cfrom_both_bounded {
    ($source:ty => $($target:ty),+) => {$(
        impl $crate::convert::Cfrom<$source> for $target {
            type Error = $crate::Error;
            #[inline]
            fn cfrom(u: $source) -> $crate::Result<Self> {
                let min = Self::MIN as $source;
                let max = Self::MAX as $source;
                if u < min || u > max {
                    Err($crate::Error::new(
                        ::alloc::format!(
                            "cannot convert value {:?} from {} to {}: value is out of bounds",
                            u,
                            ::core::any::type_name::<$source>(),
                            ::core::any::type_name::<$target>(),
                        )
                    ))
                } else {
                    Ok(u as Self)
                }
            }
        }

        impl $crate::convert::SaturatingFrom<$source> for $target {
            fn saturating_from(u: $source) -> Self {
                let min = Self::MIN as $source;
                let max = Self::MAX as $source;
                if u < min {
                    Self::MIN
                } else if u > max {
                    Self::MAX
                } else {
                    u as Self
                }
            }
        }
    )*}
}

macro_rules! rev {
    ($mac:ident, $source:ty => $($target:ty),+) => {$(
        $mac!($target => $source);
    )*}
}

// unsigned integer -> unsigned integer
impl_cfrom_upper_bounded!(u16 => u8);
impl_cfrom_upper_bounded!(u32 => u8, u16);
impl_cfrom_upper_bounded!(u64 => u8, u16, u32);
impl_cfrom_upper_bounded!(u128 => u8, u16, u32, u64);

// signed integer -> signed integer
impl_cfrom_both_bounded!(i16 => i8);
impl_cfrom_both_bounded!(i32 => i8, i16);
impl_cfrom_both_bounded!(i64 => i8, i16, i32);
impl_cfrom_both_bounded!(i128 => i8, i16, i32, i64);

// unsigned integer -> signed integer
impl_cfrom_upper_bounded!(u8 => i8);
impl_cfrom_upper_bounded!(u16 => i8, i16);
impl_cfrom_upper_bounded!(u32 => i8, i16, i32);
impl_cfrom_upper_bounded!(u64 => i8, i16, i32, i64);
impl_cfrom_upper_bounded!(u128 => i8, i16, i32, i64, i128);

// signed integer -> unsigned integer
impl_cfrom_lower_bounded!(i8 => u8, u16, u32, u64, u128);
impl_cfrom_both_bounded!(i16 => u8);
impl_cfrom_lower_bounded!(i16 => u16, u32, u64, u128);
impl_cfrom_both_bounded!(i32 => u8, u16);
impl_cfrom_lower_bounded!(i32 => u32, u64, u128);
impl_cfrom_both_bounded!(i64 => u8, u16, u32);
impl_cfrom_lower_bounded!(i64 => u64, u128);
impl_cfrom_both_bounded!(i128 => u8, u16, u32, u64);
impl_cfrom_lower_bounded!(i128 => u128);

// usize/isize
impl_cfrom_upper_bounded!(usize => isize);
impl_cfrom_lower_bounded!(isize => usize);

#[cfg(target_pointer_width = "16")]
mod ptr_try_from_impls {
    use super::TryFromIntError;

    impl_cfrom_upper_bounded!(usize => u8);
    impl_cfrom_unbounded!(usize => u16, u32, u64, u128);
    impl_cfrom_upper_bounded!(usize => i8, i16);
    impl_cfrom_unbounded!(usize => i32, i64, i128);

    impl_cfrom_both_bounded!(isize => u8);
    impl_cfrom_lower_bounded!(isize => u16, u32, u64, u128);
    impl_cfrom_both_bounded!(isize => i8);
    impl_cfrom_unbounded!(isize => i16, i32, i64, i128);

    rev!(impl_cfrom_upper_bounded, usize => u32, u64, u128);
    rev!(impl_cfrom_lower_bounded, usize => i8, i16);
    rev!(impl_cfrom_both_bounded, usize => i32, i64, i128);

    rev!(impl_cfrom_upper_bounded, isize => u16, u32, u64, u128);
    rev!(impl_cfrom_both_bounded, isize => i32, i64, i128);
}

#[cfg(target_pointer_width = "32")]
mod ptr_try_from_impls {
    use super::TryFromIntError;

    impl_cfrom_upper_bounded!(usize => u8, u16);
    impl_cfrom_unbounded!(usize => u32, u64, u128);
    impl_cfrom_upper_bounded!(usize => i8, i16, i32);
    impl_cfrom_unbounded!(usize => i64, i128);

    impl_cfrom_both_bounded!(isize => u8, u16);
    impl_cfrom_lower_bounded!(isize => u32, u64, u128);
    impl_cfrom_both_bounded!(isize => i8, i16);
    impl_cfrom_unbounded!(isize => i32, i64, i128);

    rev!(impl_cfrom_unbounded, usize => u32);
    rev!(impl_cfrom_upper_bounded, usize => u64, u128);
    rev!(impl_cfrom_lower_bounded, usize => i8, i16, i32);
    rev!(impl_cfrom_both_bounded, usize => i64, i128);

    rev!(impl_cfrom_unbounded, isize => u16);
    rev!(impl_cfrom_upper_bounded, isize => u32, u64, u128);
    rev!(impl_cfrom_unbounded, isize => i32);
    rev!(impl_cfrom_both_bounded, isize => i64, i128);
}

#[cfg(target_pointer_width = "64")]
mod ptr_try_from_impls {
    impl_cfrom_upper_bounded!(usize => u8, u16, u32);
    impl_cfrom_unbounded!(usize => u64, u128);
    impl_cfrom_upper_bounded!(usize => i8, i16, i32, i64);
    impl_cfrom_unbounded!(usize => i128);

    impl_cfrom_both_bounded!(isize => u8, u16, u32);
    impl_cfrom_lower_bounded!(isize => u64, u128);
    impl_cfrom_both_bounded!(isize => i8, i16, i32);
    impl_cfrom_unbounded!(isize => i64, i128);

    rev!(impl_cfrom_unbounded, usize => u32, u64);
    rev!(impl_cfrom_upper_bounded, usize => u128);
    rev!(impl_cfrom_lower_bounded, usize => i8, i16, i32, i64);
    rev!(impl_cfrom_both_bounded, usize => i128);

    rev!(impl_cfrom_unbounded, isize => u16, u32);
    rev!(impl_cfrom_upper_bounded, isize => u64, u128);
    rev!(impl_cfrom_unbounded, isize => i32, i64);
    rev!(impl_cfrom_both_bounded, isize => i128);
}

// TODO: float to/from int?
// TODO: float to/from bool?

impl Cfrom<CString> for String {
    type Error = crate::Error;
    fn cfrom(from: CString) -> crate::Result<Self> {
        from.try_into()
            .map_err(|from| crate::Error::new(alloc::format!("not a utf-8 string: {from:?}")))
    }
}

impl Cfrom<Vec<u8>> for String {
    type Error = crate::Error;
    fn cfrom(from: Vec<u8>) -> crate::Result<Self> {
        from.try_into()
            .map_err(|from| crate::Error::new(alloc::format!("not a utf-8 string: {from:?}")))
    }
}

impl<'a> Cfrom<&'a OsStr> for &'a str {
    type Error = crate::Error;
    fn cfrom(from: &'a OsStr) -> crate::Result<Self> {
        from.try_into()
            .map_err(|err| crate::Error::new(alloc::format!("not a utf-8 string: {from:?}: {err}")))
    }
}

impl<'a, T: Debug, const N: usize> Cfrom<&'a [T]> for &'a [T; N] {
    type Error = crate::Error;

    fn cfrom(from: &'a [T]) -> Result<Self, Self::Error> {
        from.try_into().map_err(|_| slice_to_array_error(N, from))
    }
}

impl<'a, T: Debug, const N: usize> Cfrom<&'a mut [T]> for &'a mut [T; N] {
    type Error = crate::Error;

    fn cfrom(from: &'a mut [T]) -> Result<Self, Self::Error> {
        match (&mut *from).try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(slice_to_array_error(N, from)),
        }
    }
}

fn slice_to_array_error<T: Debug>(target_len: usize, value: &[T]) -> crate::Error {
    crate::Error::new(alloc::format!(
        "expected slice of length {}, got length {}: {:?}",
        target_len,
        value.len(),
        SliceLimitedDebug(value),
    ))
}

struct SliceLimitedDebug<'a, T>(&'a [T]);

impl<'a, T: Debug> Debug for SliceLimitedDebug<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const MAX_ITEMS: usize = 32;
        if self.0.len() > MAX_ITEMS {
            let mut list = f.debug_list();
            for item in &self.0[0..MAX_ITEMS / 2] {
                list.entry(item);
            }
            // TODO: avoid quotes in "..."
            list.entry(&"...");
            for item in &self.0[self.0.len() - MAX_ITEMS / 2..] {
                list.entry(item);
            }
            list.finish()
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}
