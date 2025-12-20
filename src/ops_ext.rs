//!Extension traits for enhanced checked arithmetics
use {
    crate::{private::Sealed, Error, Result},
    alloc::format,
    core::{num::NonZero, time::Duration},
};
///Enhanced checked arithmetics functions for [`NonZero<u8>`]
pub trait NonZeroU8Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_add`]
    fn cadd(self, other: u8) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<u8> {}
impl NonZeroU8Ext for NonZero<u8> {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u8) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u16>`]
pub trait NonZeroU16Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_add`]
    fn cadd(self, other: u16) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<u16> {}
impl NonZeroU16Ext for NonZero<u16> {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u16) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u32>`]
pub trait NonZeroU32Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_add`]
    fn cadd(self, other: u32) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<u32> {}
impl NonZeroU32Ext for NonZero<u32> {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u32) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u64>`]
pub trait NonZeroU64Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_add`]
    fn cadd(self, other: u64) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<u64> {}
impl NonZeroU64Ext for NonZero<u64> {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u64) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u128>`]
pub trait NonZeroU128Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_add`]
    fn cadd(self, other: u128) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<u128> {}
impl NonZeroU128Ext for NonZero<u128> {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u128) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<usize>`]
pub trait NonZeroUsizeExt: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_add`]
    fn cadd(self, other: usize) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<usize> {}
impl NonZeroUsizeExt for NonZero<usize> {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: usize) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i8>`]
pub trait NonZeroI8Ext: Sealed {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<i8> {}
impl NonZeroI8Ext for NonZero<i8> {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i16>`]
pub trait NonZeroI16Ext: Sealed {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<i16> {}
impl NonZeroI16Ext for NonZero<i16> {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i32>`]
pub trait NonZeroI32Ext: Sealed {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<i32> {}
impl NonZeroI32Ext for NonZero<i32> {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i64>`]
pub trait NonZeroI64Ext: Sealed {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<i64> {}
impl NonZeroI64Ext for NonZero<i64> {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i128>`]
pub trait NonZeroI128Ext: Sealed {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<i128> {}
impl NonZeroI128Ext for NonZero<i128> {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<isize>`]
pub trait NonZeroIsizeExt: Sealed {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
}
impl Sealed for NonZero<isize> {}
impl NonZeroIsizeExt for NonZero<isize> {
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`i8`]
pub trait I8Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u8) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u8) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i8::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i8::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i8::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i8::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i8 {}
impl I8Ext for i8 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u8) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u8) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i8::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i8::checked_isqrt`]
    #[inline]
    #[track_caller]
    fn cisqrt(self) -> Result<Self> {
        self.checked_isqrt().ok_or_else(|| {
            Error::new(format!(
                "failed to compute isqrt({:?}): argument is negative",
                self
            ))
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i8::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i8::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i8::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`i16`]
pub trait I16Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u16) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u16) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i16::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i16::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i16::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i16::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i16 {}
impl I16Ext for i16 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u16) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u16) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i16::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i16::checked_isqrt`]
    #[inline]
    #[track_caller]
    fn cisqrt(self) -> Result<Self> {
        self.checked_isqrt().ok_or_else(|| {
            Error::new(format!(
                "failed to compute isqrt({:?}): argument is negative",
                self
            ))
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i16::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i16::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i16::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`i32`]
pub trait I32Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u32) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u32) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i32::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i32::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i32::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i32::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i32 {}
impl I32Ext for i32 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u32) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u32) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i32::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i32::checked_isqrt`]
    #[inline]
    #[track_caller]
    fn cisqrt(self) -> Result<Self> {
        self.checked_isqrt().ok_or_else(|| {
            Error::new(format!(
                "failed to compute isqrt({:?}): argument is negative",
                self
            ))
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i32::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i32::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i32::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`i64`]
pub trait I64Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u64) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u64) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i64::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i64::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i64::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i64::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i64 {}
impl I64Ext for i64 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u64) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u64) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i64::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i64::checked_isqrt`]
    #[inline]
    #[track_caller]
    fn cisqrt(self) -> Result<Self> {
        self.checked_isqrt().ok_or_else(|| {
            Error::new(format!(
                "failed to compute isqrt({:?}): argument is negative",
                self
            ))
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i64::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i64::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i64::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`i128`]
pub trait I128Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u128) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u128) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i128::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i128::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i128::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i128::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i128 {}
impl I128Ext for i128 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u128) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u128) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`i128::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`i128::checked_isqrt`]
    #[inline]
    #[track_caller]
    fn cisqrt(self) -> Result<Self> {
        self.checked_isqrt().ok_or_else(|| {
            Error::new(format!(
                "failed to compute isqrt({:?}): argument is negative",
                self
            ))
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`i128::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i128::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`i128::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`isize`]
pub trait IsizeExt: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add_unsigned`]
    fn cadd_unsigned(self, other: usize) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub_unsigned`]
    fn csub_unsigned(self, other: usize) -> Result<Self>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`isize::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`isize::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`isize::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`isize::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for isize {}
impl IsizeExt for isize {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: usize) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: usize) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`isize::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked absolute value: computes `|a|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs()
            .ok_or_else(|| Error::new(format!("failed to compute abs({:?}): overflow", self)))
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked square root: computes `√a`, returning an error if `a` is negative.
    ///
    ///Wrapper for [`isize::checked_isqrt`]
    #[inline]
    #[track_caller]
    fn cisqrt(self) -> Result<Self> {
        self.checked_isqrt().ok_or_else(|| {
            Error::new(format!(
                "failed to compute isqrt({:?}): argument is negative",
                self
            ))
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`isize::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`isize::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`isize::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u8`]
pub trait U8Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add_signed`]
    fn cadd_signed(self, other: i8) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub_signed`]
    fn csub_signed(self, other: i8) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i8>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u8::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u8::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u8::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u8::checked_next_multiple_of`]
    fn cnext_multiple_of(self, other: Self) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
}
impl Sealed for u8 {}
impl U8Ext for u8 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i8) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i8) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i8> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u8::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u8::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u8::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u8::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u8::checked_next_multiple_of`]
    #[inline]
    #[track_caller]
    fn cnext_multiple_of(self, other: Self) -> Result<Self> {
        self.checked_next_multiple_of(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): multiplier is zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u16`]
pub trait U16Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add_signed`]
    fn cadd_signed(self, other: i16) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub_signed`]
    fn csub_signed(self, other: i16) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i16>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u16::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u16::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u16::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u16::checked_next_multiple_of`]
    fn cnext_multiple_of(self, other: Self) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
}
impl Sealed for u16 {}
impl U16Ext for u16 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i16) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i16) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i16> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u16::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u16::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u16::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u16::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u16::checked_next_multiple_of`]
    #[inline]
    #[track_caller]
    fn cnext_multiple_of(self, other: Self) -> Result<Self> {
        self.checked_next_multiple_of(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): multiplier is zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u32`]
pub trait U32Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add_signed`]
    fn cadd_signed(self, other: i32) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub_signed`]
    fn csub_signed(self, other: i32) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i32>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u32::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u32::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u32::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u32::checked_next_multiple_of`]
    fn cnext_multiple_of(self, other: Self) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
}
impl Sealed for u32 {}
impl U32Ext for u32 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i32) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i32) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i32> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u32::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u32::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u32::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u32::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u32::checked_next_multiple_of`]
    #[inline]
    #[track_caller]
    fn cnext_multiple_of(self, other: Self) -> Result<Self> {
        self.checked_next_multiple_of(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): multiplier is zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u64`]
pub trait U64Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add_signed`]
    fn cadd_signed(self, other: i64) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub_signed`]
    fn csub_signed(self, other: i64) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i64>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u64::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u64::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u64::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u64::checked_next_multiple_of`]
    fn cnext_multiple_of(self, other: Self) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
}
impl Sealed for u64 {}
impl U64Ext for u64 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i64) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i64) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i64> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u64::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u64::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u64::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u64::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u64::checked_next_multiple_of`]
    #[inline]
    #[track_caller]
    fn cnext_multiple_of(self, other: Self) -> Result<Self> {
        self.checked_next_multiple_of(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): multiplier is zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u128`]
pub trait U128Ext: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add_signed`]
    fn cadd_signed(self, other: i128) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub_signed`]
    fn csub_signed(self, other: i128) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i128>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u128::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u128::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u128::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u128::checked_next_multiple_of`]
    fn cnext_multiple_of(self, other: Self) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
}
impl Sealed for u128 {}
impl U128Ext for u128 {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i128) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i128) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i128> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`u128::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`u128::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u128::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`u128::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`u128::checked_next_multiple_of`]
    #[inline]
    #[track_caller]
    fn cnext_multiple_of(self, other: Self) -> Result<Self> {
        self.checked_next_multiple_of(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): multiplier is zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`usize`]
pub trait UsizeExt: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add_signed`]
    fn cadd_signed(self, other: isize) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub_signed`]
    fn csub_signed(self, other: isize) -> Result<Self>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<isize>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_div`]
    fn cdiv(self, other: Self) -> Result<Self>;
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_div_euclid`]
    fn cdiv_euclid(self, other: Self) -> Result<Self>;
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_rem`]
    fn crem(self, other: Self) -> Result<Self>;
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_rem_euclid`]
    fn crem_euclid(self, other: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`usize::checked_ilog`]
    fn cilog(self, other: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`usize::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`usize::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_pow`]
    fn cpow(self, other: u32) -> Result<Self>;
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`usize::checked_next_multiple_of`]
    fn cnext_multiple_of(self, other: Self) -> Result<Self>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
}
impl Sealed for usize {}
impl UsizeExt for usize {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: isize) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: isize) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<isize> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: Self) -> Result<Self> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian division: computes `a.div_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, other: Self) -> Result<Self> {
        self.checked_div_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `a % b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, other: Self) -> Result<Self> {
        self.checked_rem(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} % {:?}: overflow", self, other)
                }
            })
        })
    }
    ///Checked euclidian reminder: computes `a.rem_euclid(b)`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`usize::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, other: Self) -> Result<Self> {
        self.checked_rem_euclid(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>b</sub> a</code>, returning an error if the number is negative or zero, or if the base is less than 2.
    ///
    ///Wrapper for [`usize::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, other: Self) -> Result<u32> {
        self.checked_ilog(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln a`, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`usize::checked_ilog2`]
    #[inline]
    #[track_caller]
    fn cilog2(self) -> Result<u32> {
        self.checked_ilog2().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog2({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> a</code>, returning an error if the number is negative or zero.
    ///
    ///Wrapper for [`usize::checked_ilog10`]
    #[inline]
    #[track_caller]
    fn cilog10(self) -> Result<u32> {
        self.checked_ilog10().ok_or_else(|| {
            Error::new(format!(
                "failed to compute ilog10({:?}): argument is not positive",
                self
            ))
        })
    }
    ///Checked negation: computes `-a`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg()
            .ok_or_else(|| Error::new(format!("failed to compute -{:?}: overflow", self)))
    }
    ///Checked shift left: computes `a << b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked shift right: computes `a >> b`, returning an error if `b` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                self, other
            ))
        })
    }
    ///Checked exponentiation: computes <code>a<sup>b</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, other: u32) -> Result<Self> {
        self.checked_pow(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): overflow",
                self, other
            ))
        })
    }
    ///Checked next multiple of `b`, returning an error if overflow occured or if `b` is zero.
    ///
    ///Wrapper for [`usize::checked_next_multiple_of`]
    #[inline]
    #[track_caller]
    fn cnext_multiple_of(self, other: Self) -> Result<Self> {
        self.checked_next_multiple_of(other).ok_or_else(|| {
            Error::new({
                if other < 2 {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): multiplier is zero",
                        self, other
                    )
                } else {
                    format!(
                        "failed to compute next_multiple_of({:?}, {:?}): overflow",
                        self, other
                    )
                }
            })
        })
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): overflow",
                self
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`Duration`]
pub trait DurationExt: Sealed {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_add`]
    fn cadd(self, other: Duration) -> Result<Duration>;
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_sub`]
    fn csub(self, other: Duration) -> Result<Duration>;
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_mul`]
    fn cmul(self, other: u32) -> Result<Duration>;
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`Duration::checked_div`]
    fn cdiv(self, other: u32) -> Result<Duration>;
}
impl Sealed for Duration {}
impl DurationExt for Duration {
    ///Checked addition: computes `a + b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Duration) -> Result<Duration> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked subtraction:  computes `a - b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Duration) -> Result<Duration> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked multiplication: computes `a * b`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: u32) -> Result<Duration> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: overflow",
                self, other
            ))
        })
    }
    ///Checked division: computes `a / b`, returning an error if overflow occured or if the divisor is zero.
    ///
    ///Wrapper for [`Duration::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, other: u32) -> Result<Duration> {
        self.checked_div(other).ok_or_else(|| {
            Error::new({
                if other == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self, other
                    )
                } else {
                    format!("failed to compute {:?} / {:?}: overflow", self, other)
                }
            })
        })
    }
}
