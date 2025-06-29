mod array;
mod num;

use {
    crate::convert::Cfrom,
    alloc::{ffi::CString, string::String, vec::Vec},
    core::num::NonZero,
    std::ffi::OsStr,
};

// delegate to TryFrom
macro_rules! impl_cfrom {
    ($(($from:ty, $to:ty),)*) => {
        $(
            impl $crate::convert::Cfrom<$from> for $to {
                type Error = $crate::Error;
                #[inline]
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
pub(crate) use impl_cfrom;

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
);

// TODO: float to/from int?
// TODO: float to/from bool?

impl Cfrom<CString> for String {
    type Error = crate::Error;
    #[inline]
    fn cfrom(from: CString) -> crate::Result<Self> {
        from.try_into()
            .map_err(|from| crate::Error::new(alloc::format!("not a utf-8 string: {from:?}")))
    }
}

impl Cfrom<Vec<u8>> for String {
    type Error = crate::Error;
    #[inline]
    fn cfrom(from: Vec<u8>) -> crate::Result<Self> {
        from.try_into()
            .map_err(|from| crate::Error::new(alloc::format!("not a utf-8 string: {from:?}")))
    }
}

impl<'a> Cfrom<&'a OsStr> for &'a str {
    type Error = crate::Error;
    #[inline]
    fn cfrom(from: &'a OsStr) -> crate::Result<Self> {
        from.try_into()
            .map_err(|err| crate::Error::new(alloc::format!("not a utf-8 string: {from:?}: {err}")))
    }
}
