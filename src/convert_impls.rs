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

// to/from usize
#[rustfmt::skip]
impl_cfrom!(
    (i8, usize),
    (i16, usize),
    (u32, usize), (u32, isize),
    (i32, usize), (i32, isize),
    (u64, usize), (u64, isize),
    (i64, usize), (i64, isize),
    (u128, usize), (u128, isize),
    (i128, usize), (i128, isize),
    (usize, u128), (usize, i128), (usize, u64), (usize, i64), (usize, u32), (usize, i32), (usize, u16), (usize, i16), (usize, u8), (usize, i8),
    (isize, u128), (isize, i128), (isize, u64), (isize, i64), (isize, u32), (isize, i32), (isize, u16), (isize, i16), (isize, u8), (isize, i8),
);

// to smaller type
#[rustfmt::skip]
impl_cfrom!(
    (u16, u8),
    (i16, i8),
    (u32, u16), (u32, u8),
    (i32, i16), (i32, i8),
    (u64, u32), (u64, u16), (u64, u8),
    (i64, i32), (i64, i16), (i64, i8),
    (u128, u64), (u128, u32), (u128, u16), (u128, u8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8),
);

// from signed to unsigned
#[rustfmt::skip]
impl_cfrom!(
    (i8, u8), (i8, u16), (i8, u32), (i8, u64), (i8, u128),
    (i16, u8), (i16, u16), (i16, u32), (i16, u64), (i16, u128),
    (i32, u8), (i32, u16), (i32, u32), (i32, u64), (i32, u128),
    (i64, u8), (i64, u16), (i64, u32), (i64, u64), (i64, u128),
    (i128, u8), (i128, u16), (i128, u32), (i128, u64), (i128, u128),
);

// from unsigned to signed
#[rustfmt::skip]
impl_cfrom!(
    (u8, i8),
    (u16, i8), (u16, i16),
    (u32, i8), (u32, i16), (u32, i32),
    (u64, i8), (u64, i16), (u64, i32), (u64, i64),
    (u128, i8), (u128, i16), (u128, i32), (u128, i64), (u128, i128),
);

macro_rules! impl_saturating_from {
    ($(($from:ty, $to:ty),)*) => {
        $(
            impl $crate::convert::SaturatingFrom<$from> for $to {
                fn saturating_from(from: $from) -> $crate::Result<Self> {
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
