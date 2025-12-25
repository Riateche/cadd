mod array;
mod num;
#[cfg(feature = "std")]
mod os_str;

use {
    crate::{
        convert::{Cfrom, IntoType},
        Error, Result,
    },
    alloc::{ffi::CString, format, string::String, vec::Vec},
    core::{
        ffi::CStr,
        fmt::{Debug, Write},
        num::NonZero,
        str::Utf8Error,
    },
};

// delegate to TryFrom
macro_rules! impl_cfrom_int {
    ($(($from:ty, $to:ty),)*) => {
        $(
            impl $crate::convert::Cfrom<$from> for $to {
                type Error = $crate::Error;
                #[inline]
                fn cfrom(from: $from) -> $crate::Result<Self> {
                    ::core::convert::TryFrom::try_from(from)
                        .map_err(|_| $crate::Error::new(
                            ::alloc::format!(
                                "failed to convert value {:?} from {} to {}: value is out of bounds",
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
pub(crate) use impl_cfrom_int;

impl_cfrom_int!(
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

const MAX_PRINT_SLICE_ITEMS: usize = 20;
const MAX_LOSSY_CONVERT_BYTES: usize = 200;
struct LimitedSliceDebug<'a, T> {
    data: &'a [T],
    items_text: &'static str,
}

struct NonExhaustive;
impl Debug for NonExhaustive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "..")
    }
}

impl<'a, T: Debug> Debug for LimitedSliceDebug<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.data.len() <= MAX_PRINT_SLICE_ITEMS {
            write!(f, "{:?}", self.data)?;
        } else {
            let mut list = f.debug_list();
            list.entries(&self.data[..MAX_PRINT_SLICE_ITEMS / 2]);
            list.entry(&NonExhaustive);
            list.entries(&self.data[self.data.len() - MAX_PRINT_SLICE_ITEMS / 2..]);
            list.finish()?;
        }
        write!(f, " ({} {})", self.data.len(), self.items_text)
    }
}

struct LimitedStrDebug<'a>(&'a str);
impl<'a> Debug for LimitedStrDebug<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let chars: Vec<_> = self.0.chars().collect();
        if chars.len() <= MAX_PRINT_SLICE_ITEMS {
            write!(f, "{:?}", self.0)
        } else {
            let start: String = chars[..MAX_PRINT_SLICE_ITEMS / 2].iter().collect();
            let end: String = chars[chars.len() - MAX_PRINT_SLICE_ITEMS / 2..]
                .iter()
                .collect();
            write!(f, "{:?}..{:?}", start, end)
        }
    }
}

fn bytes_to_str_error(input: &[u8], err: Utf8Error) -> Error {
    let mut text = format!(
        "failed to convert bytes to string: {}; input: {:?}",
        err,
        LimitedSliceDebug {
            data: input,
            items_text: "bytes"
        },
    );
    if input.len() <= MAX_LOSSY_CONVERT_BYTES {
        write!(
            text,
            "; input as lossy utf-8: {:?}",
            LimitedStrDebug(&String::from_utf8_lossy(input))
        )
        .unwrap();
    }
    Error::new(text)
}

impl Cfrom<Vec<u8>> for String {
    type Error = Error;
    #[inline]
    fn cfrom(from: Vec<u8>) -> Result<Self> {
        from.try_into_type::<String>()
            .map_err(|err| bytes_to_str_error(err.as_bytes(), err.utf8_error()))
    }
}

impl<'a> Cfrom<&'a [u8]> for &'a str {
    type Error = Error;
    #[inline]
    fn cfrom(from: &'a [u8]) -> Result<Self> {
        str::from_utf8(from).map_err(|err| bytes_to_str_error(from, err))
    }
}

impl Cfrom<CString> for String {
    type Error = Error;
    #[inline]
    fn cfrom(from: CString) -> Result<Self> {
        from.try_into_type::<String>().map_err(|error| {
            let utf8_error = error.utf8_error();
            bytes_to_str_error(error.into_cstring().as_bytes(), utf8_error)
        })
    }
}

impl<'a> Cfrom<&'a CStr> for &'a str {
    type Error = Error;
    #[inline]
    fn cfrom(from: &'a CStr) -> Result<Self> {
        from.to_str()
            .map_err(|err| bytes_to_str_error(from.to_bytes(), err))
    }
}
