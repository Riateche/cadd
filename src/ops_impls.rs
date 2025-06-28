#[cfg(feature = "std")]
use std::time::{Instant, SystemTime};
use {
    alloc::format,
    core::{num::NonZero, time::Duration},
};

macro_rules! impl_binary_op {
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, msg=$msg:literal for $t1:ty, $t2:ty, $out:ty) => {
        impl $crate::ops::$trait_<$t2> for $t1 {
            type Output = $out;
            type Error = $crate::Error;
            fn $trait_fn(self, b: $t2) -> $crate::Result<$out> {
                self.$source_fn(b)
                    .ok_or_else(|| crate::Error::new(format!($msg, self, b)))
            }
        }
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, err=$err:expr, for $t1:ty, $t2:ty, $out:ty) => {
        impl $crate::ops::$trait_<$t2> for $t1 {
            type Output = $out;
            type Error = $crate::Error;
            fn $trait_fn(self, b: $t2) -> $crate::Result<$out> {
                self.$source_fn(b)
                    .ok_or_else(|| crate::Error::new(($err)(self, b)))
            }
        }
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, msg=$msg:literal for $t1:ty) => {
        impl_binary_op!($trait_, $trait_fn, $source_fn, msg=$msg for $t1, $t1, $t1);
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, err=$err:expr, for $t1:ty) => {
        impl_binary_op!($trait_, $trait_fn, $source_fn, err=$err, for $t1, $t1, $t1);
    };
}

macro_rules! impl_binary_ops {
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, msg=$msg:literal for $(($($t1:tt)*),)+) => {
        $(
            impl_binary_op!($trait_, $trait_fn, $source_fn, msg=$msg for $($t1)*);
        )*
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, err=$err:expr, for $(($($t1:tt)*),)+) => {
        $(
            impl_binary_op!($trait_, $trait_fn, $source_fn, err=$err, for $($t1)*);
        )*
    };
}

macro_rules! impl_unary_op {
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, msg=$msg:literal for $t1:ty, $out:ty) => {
        impl $crate::ops::$trait_ for $t1 {
            type Output = $out;
            type Error = $crate::Error;
            fn $trait_fn(self) -> $crate::Result<$out> {
                self.$source_fn()
                    .ok_or_else(|| crate::Error::new(format!($msg, self)))
            }
        }
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, err=$err:expr, for $t1:ty, $out:ty) => {
        impl $crate::ops::$trait_ for $t1 {
            type Output = $out;
            type Error = $crate::Error;
            fn $trait_fn(self) -> $crate::Result<$out> {
                self.$source_fn()
                    .ok_or_else(|| crate::Error::new(($err)(self)))
            }
        }
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, msg=$msg:literal for $t1:ty) => {
        impl_unary_op!($trait_, $trait_fn, $source_fn, msg=$msg for $t1, $t1);
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, err=$err:expr, for $t1:ty) => {
        impl_unary_op!($trait_, $trait_fn, $source_fn, err=$err, for $t1, $t1);
    };
}

macro_rules! impl_unary_ops {
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, msg=$msg:literal for $(($($t1:tt)*),)+) => {
        $(
            impl_unary_op!($trait_, $trait_fn, $source_fn, msg=$msg for $($t1)*);
        )*
    };
    ($trait_:ident, $trait_fn:ident, $source_fn:ident, err=$err:expr, for $(($($t1:tt)*),)+) => {
        $(
            impl_unary_op!($trait_, $trait_fn, $source_fn, err=$err, for $($t1)*);
        )*
    };
}

impl_binary_ops!(
    Cadd, cadd, checked_add, msg="overflow: {:?} + {:?}"
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
    (Duration),
    (NonZero<u8>, u8, NonZero<u8>),
    (NonZero<u16>, u16, NonZero<u16>),
    (NonZero<u32>, u32, NonZero<u32>),
    (NonZero<u64>, u64, NonZero<u64>),
    (NonZero<u128>, u128, NonZero<u128>),
    (NonZero<usize>, usize, NonZero<usize>),
);
#[cfg(feature = "std")]
impl_binary_ops!(
    Cadd, cadd, checked_add, msg="overflow: {:?} + {:?}"
    for
    (Instant, Duration, Instant),
    (SystemTime, Duration, SystemTime),
);

impl_binary_ops!(
    Cadd, cadd, checked_add_signed, msg="overflow: {} + {}"
    for
    (u8, i8, u8),
    (u16, i16, u16),
    (u32, i32, u32),
    (u64, i64, u64),
    (u128, i128, u128),
    (usize, isize, usize),
);

impl_binary_ops!(
    Cadd, cadd, checked_add_unsigned, msg="overflow: {} + {}"
    for
    (i8, u8, i8),
    (i16, u16, i16),
    (i32, u32, i32),
    (i64, u64, i64),
    (i128, u128, i128),
    (isize, usize, isize),
);

impl_binary_ops!(
    Csub, csub, checked_sub, msg="overflow: {:?} - {:?}"
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
    (Duration),
);
#[cfg(feature = "std")]
impl_binary_ops!(
    Csub, csub, checked_sub, msg="overflow: {:?} - {:?}"
    for
    (Instant, Duration, Instant),
    (SystemTime, Duration, SystemTime),
);

impl_binary_ops!(
    Csub, csub, checked_sub_unsigned, msg="overflow: {} + {}"
    for
    (i8, u8, i8),
    (i16, u16, i16),
    (i32, u32, i32),
    (i64, u64, i64),
    (i128, u128, i128),
    (isize, usize, isize),
);

impl_binary_ops!(
    Cmul, cmul, checked_mul, msg="overflow: {:?} * {:?}"
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
    (NonZero<u8>), (NonZero<u16>), (NonZero<u32>), (NonZero<u64>), (NonZero<u128>), (NonZero<usize>),
    (NonZero<i8>), (NonZero<i16>), (NonZero<i32>), (NonZero<i64>), (NonZero<i128>), (NonZero<isize>),
    (Duration, u32, Duration),
);

impl_unary_ops!(
    Cneg, cneg, checked_neg, msg="overflow: -{}"
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
    (NonZero<i8>), (NonZero<i16>), (NonZero<i32>), (NonZero<i64>), (NonZero<i128>), (NonZero<isize>),
);

impl_binary_ops!(
    Cdiv, cdiv, checked_div, err=|a, b| {
        if b == 0 {
            format!("division by zero: {a:?} / {b:?}")
        } else {
            format!("overflow: {a:?} / {b:?}")
        }
    },
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
    (Duration, u32, Duration),
);

impl_binary_ops!(
    CdivEuclid, cdiv_euclid, checked_div_euclid, err=|a, b| {
        if b == 0 {
            format!("division by zero: div_euclid({a:?}, {b:?})")
        } else {
            format!("overflow: div_euclid({a:?}, {b:?})")
        }
    },
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
);

impl_binary_ops!(
    Crem, crem, checked_rem, err=|a, b| {
        if b == 0 {
            format!("division by zero: {a:?} % {b:?}")
        } else {
            format!("overflow: {a:?} % {b:?}")
        }
    },
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
);

impl_binary_ops!(
    CremEuclid, crem_euclid, checked_rem_euclid, err=|a, b| {
        if b == 0 {
            format!("division by zero: rem_euclid({a:?}, {b:?})")
        } else {
            format!("overflow: rem_euclid({a:?}, {b:?})")
        }
    },
    for (u8), (i8), (u16), (i16), (u32), (i32), (u64), (i64), (u128), (i128), (usize), (isize),
);

impl_binary_ops!(
    CILog, cilog, checked_ilog, err=|a, b| {
        if b < 2 {
            format!("base is less than 2: ilog({a}, {b})")
        } else {
            format!("number is not positive: ilog({a}, {b})")
        }
    },
    for
    (u8, u8, u32),
    (u16, u16, u32),
    (u32, u32, u32),
    (u64, u64, u32),
    (u128, u128, u32),
    (usize, usize, u32),
    (i8, i8, u32),
    (i16, i16, u32),
    (i32, i32, u32),
    (i64, i64, u32),
    (i128, i128, u32),
    (isize, isize, u32),
);

impl_unary_ops!(
    CILog2, cilog2, checked_ilog2, msg="number is not positive: ilog2({})"
    for
    (u8, u32),
    (u16, u32),
    (u32, u32),
    (u64, u32),
    (u128, u32),
    (usize, u32),
    (i8, u32),
    (i16, u32),
    (i32, u32),
    (i64, u32),
    (i128, u32),
    (isize, u32),
);

impl_unary_ops!(
    CILog10, cilog10, checked_ilog10, msg="number is not positive: ilog10({})"
    for
    (u8, u32),
    (u16, u32),
    (u32, u32),
    (u64, u32),
    (u128, u32),
    (usize, u32),
    (i8, u32),
    (i16, u32),
    (i32, u32),
    (i64, u32),
    (i128, u32),
    (isize, u32),
);

impl_binary_ops!(
    Cshl, cshl, checked_shl, msg="shift amount is too large: {} << {}"
    for
    (u8, u32, u8),
    (u16, u32, u16),
    (u32, u32, u32),
    (u64, u32, u64),
    (u128, u32, u128),
    (usize, u32, usize),
    (i8, u32, i8),
    (i16, u32, i16),
    (i32, u32, i32),
    (i64, u32, i64),
    (i128, u32, i128),
    (isize, u32, isize),
);

impl_binary_ops!(
    Cshr, cshr, checked_shr, msg="shift amount is too large: {} >> {}"
    for
    (u8, u32, u8),
    (u16, u32, u16),
    (u32, u32, u32),
    (u64, u32, u64),
    (u128, u32, u128),
    (usize, u32, usize),
    (i8, u32, i8),
    (i16, u32, i16),
    (i32, u32, i32),
    (i64, u32, i64),
    (i128, u32, i128),
    (isize, u32, isize),
);

impl_binary_ops!(
    Cpow, cpow, checked_pow, msg="overflow: pow({}, {})"
    for
    (u8, u32, u8),
    (u16, u32, u16),
    (u32, u32, u32),
    (u64, u32, u64),
    (u128, u32, u128),
    (usize, u32, usize),
    (i8, u32, i8),
    (i16, u32, i16),
    (i32, u32, i32),
    (i64, u32, i64),
    (i128, u32, i128),
    (isize, u32, isize),
);

impl_unary_ops!(
    Cabs, cabs, checked_abs, msg="overflow: abs({})"
    for
    (i8), (i16), (i32), (i64), (i128), (isize),
    (NonZero<i8>), (NonZero<i16>), (NonZero<i32>), (NonZero<i64>), (NonZero<i128>), (NonZero<isize>),
);

// impl_unary_ops!(
//     Cisqrt, cisqrt, checked_isqrt, msg="number is negative: isqrt({})"
//     for
//     (i8), (i16), (i32), (i64), (i128), (isize),
// );

impl_binary_ops!(
    CnextMultipleOf, cnext_multiple_of, checked_next_multiple_of, err=|a, b| {
        if b == 0 {
            format!("multiplier is zero: next_multiple_of({a}, {b})")
        } else {
            format!("overflow: next_multiple_of({a}, {b})")
        }
    },
    for (u8), (u16), (u32), (u64), (u128), (usize),
);

impl_unary_ops!(
    CnextPowerOfTwo, cnext_power_of_two, checked_next_power_of_two, msg="overflow: next_power_of_two({})"
    for (u8), (u16), (u32), (u64), (u128), (usize),
    (NonZero<u8>), (NonZero<u16>), (NonZero<u32>), (NonZero<u64>), (NonZero<u128>), (NonZero<usize>),
);
