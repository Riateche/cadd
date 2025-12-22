//!Extension traits for enhanced checked arithmetics
use {
    crate::{private::Sealed, Error, MaybeParens, Result},
    alloc::format,
    core::{num::NonZero, time::Duration},
};
///Enhanced checked arithmetics functions for [`NonZero<u8>`]
pub trait NonZeroU8Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_add`]
    fn cadd(self, other: u8) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_add`]
    fn cadd_assign(&mut self, other: u8) -> Result<()>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<u8> {}
impl NonZeroU8Ext for NonZero<u8> {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u8) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u8>"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: u8) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "NonZero<u8>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u8>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u8>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<u8>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u16>`]
pub trait NonZeroU16Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_add`]
    fn cadd(self, other: u16) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_add`]
    fn cadd_assign(&mut self, other: u16) -> Result<()>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<u16> {}
impl NonZeroU16Ext for NonZero<u16> {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u16) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u16>"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: u16) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "NonZero<u16>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u16>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u16>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<u16>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u32>`]
pub trait NonZeroU32Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_add`]
    fn cadd(self, other: u32) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_add`]
    fn cadd_assign(&mut self, other: u32) -> Result<()>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<u32> {}
impl NonZeroU32Ext for NonZero<u32> {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u32) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u32>"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "NonZero<u32>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u32>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u32>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<u32>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u64>`]
pub trait NonZeroU64Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_add`]
    fn cadd(self, other: u64) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_add`]
    fn cadd_assign(&mut self, other: u64) -> Result<()>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<u64> {}
impl NonZeroU64Ext for NonZero<u64> {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u64) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u64>"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: u64) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "NonZero<u64>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u64>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u64>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<u64>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<u128>`]
pub trait NonZeroU128Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_add`]
    fn cadd(self, other: u128) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_add`]
    fn cadd_assign(&mut self, other: u128) -> Result<()>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<u128> {}
impl NonZeroU128Ext for NonZero<u128> {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: u128) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u128>"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: u128) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "NonZero<u128>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<u128>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<u128>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<u128>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<usize>`]
pub trait NonZeroUsizeExt: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_add`]
    fn cadd(self, other: usize) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_add`]
    fn cadd_assign(&mut self, other: usize) -> Result<()>;
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_next_power_of_two`]
    fn cnext_power_of_two(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<usize> {}
impl NonZeroUsizeExt for NonZero<usize> {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: usize) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<usize>"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: usize) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked next power of 2, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_next_power_of_two`]
    #[inline]
    #[track_caller]
    fn cnext_power_of_two(self) -> Result<Self> {
        self.checked_next_power_of_two().ok_or_else(|| {
            Error::new(format!(
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "NonZero<usize>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<usize>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<usize>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<usize>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i8>`]
pub trait NonZeroI8Ext: Sealed {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<i8> {}
impl NonZeroI8Ext for NonZero<i8> {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "NonZero<i8>"
            ))
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "NonZero<i8>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<i8>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i8>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<i8>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i16>`]
pub trait NonZeroI16Ext: Sealed {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<i16> {}
impl NonZeroI16Ext for NonZero<i16> {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "NonZero<i16>"
            ))
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "NonZero<i16>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<i16>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i16>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<i16>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i32>`]
pub trait NonZeroI32Ext: Sealed {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<i32> {}
impl NonZeroI32Ext for NonZero<i32> {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "NonZero<i32>"
            ))
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "NonZero<i32>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<i32>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i32>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<i32>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i64>`]
pub trait NonZeroI64Ext: Sealed {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<i64> {}
impl NonZeroI64Ext for NonZero<i64> {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "NonZero<i64>"
            ))
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "NonZero<i64>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<i64>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i64>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<i64>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<i128>`]
pub trait NonZeroI128Ext: Sealed {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<i128> {}
impl NonZeroI128Ext for NonZero<i128> {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "NonZero<i128>"
            ))
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "NonZero<i128>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<i128>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<i128>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<i128>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`NonZero<isize>`]
pub trait NonZeroIsizeExt: Sealed {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
}
impl Sealed for NonZero<isize> {}
impl NonZeroIsizeExt for NonZero<isize> {
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "NonZero<isize>"
            ))
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "NonZero<isize>"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "NonZero<isize>"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`NonZero<isize>::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "NonZero<isize>"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`i8`]
pub trait I8Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u8) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u8) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
    ///
    ///Wrapper for [`i8::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i8::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i8::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i8::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i8 {}
impl I8Ext for i8 {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i8"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u8) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_unsigned({:?}, {:?}): {} overflow",
                self, other, "i8"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i8"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u8) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_unsigned({:?}, {:?}): {} overflow",
                self, other, "i8"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i8"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i8"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i8"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i8"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i8::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i8"
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "i8"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i8::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "i8"
            ))
        })
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i8::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "i8"
            ))
        })
    }
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
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
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i8::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u16) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u16) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
    ///
    ///Wrapper for [`i16::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i16::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i16::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i16::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i16 {}
impl I16Ext for i16 {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i16"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u16) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_unsigned({:?}, {:?}): {} overflow",
                self, other, "i16"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i16"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u16) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_unsigned({:?}, {:?}): {} overflow",
                self, other, "i16"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i16"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i16"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i16"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i16"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i16::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i16"
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "i16"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i16::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "i16"
            ))
        })
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i16::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "i16"
            ))
        })
    }
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
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
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i16::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u32) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u32) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
    ///
    ///Wrapper for [`i32::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i32::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i32::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i32::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i32 {}
impl I32Ext for i32 {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i32"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u32) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_unsigned({:?}, {:?}): {} overflow",
                self, other, "i32"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i32"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u32) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_unsigned({:?}, {:?}): {} overflow",
                self, other, "i32"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i32"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i32"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i32"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i32"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i32::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i32"
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "i32"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i32::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "i32"
            ))
        })
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i32::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "i32"
            ))
        })
    }
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
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
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i32::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u64) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u64) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
    ///
    ///Wrapper for [`i64::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i64::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i64::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i64::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i64 {}
impl I64Ext for i64 {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i64"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u64) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_unsigned({:?}, {:?}): {} overflow",
                self, other, "i64"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i64"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u64) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_unsigned({:?}, {:?}): {} overflow",
                self, other, "i64"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i64"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i64"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i64"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i64"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i64::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i64"
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "i64"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i64::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "i64"
            ))
        })
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i64::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "i64"
            ))
        })
    }
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
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
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i64::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add_unsigned`]
    fn cadd_unsigned(self, other: u128) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub_unsigned`]
    fn csub_unsigned(self, other: u128) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
    ///
    ///Wrapper for [`i128::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i128::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i128::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`i128::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for i128 {}
impl I128Ext for i128 {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i128"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: u128) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_unsigned({:?}, {:?}): {} overflow",
                self, other, "i128"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i128"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: u128) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_unsigned({:?}, {:?}): {} overflow",
                self, other, "i128"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "i128"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i128"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i128"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "i128"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`i128::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "i128"
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "i128"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`i128::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "i128"
            ))
        })
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`i128::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "i128"
            ))
        })
    }
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
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
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`i128::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add_unsigned`]
    fn cadd_unsigned(self, other: usize) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub_unsigned`]
    fn csub_unsigned(self, other: usize) -> Result<Self>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_abs`]
    fn cabs(self) -> Result<Self>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
    ///
    ///Wrapper for [`isize::checked_isqrt`]
    fn cisqrt(self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`isize::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`isize::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`isize::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
}
impl Sealed for isize {}
impl IsizeExt for isize {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "isize"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_add_unsigned`]
    #[inline]
    #[track_caller]
    fn cadd_unsigned(self, other: usize) -> Result<Self> {
        self.checked_add_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_unsigned({:?}, {:?}): {} overflow",
                self, other, "isize"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "isize"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_unsigned(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_sub_unsigned`]
    #[inline]
    #[track_caller]
    fn csub_unsigned(self, other: usize) -> Result<Self> {
        self.checked_sub_unsigned(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_unsigned({:?}, {:?}): {} overflow",
                self, other, "isize"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "isize"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "isize"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "isize"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "isize"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`isize::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "isize"
                    )
                }
            })
        })
    }
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "isize"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`isize::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked absolute value: computes `|self|`, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_abs`]
    #[inline]
    #[track_caller]
    fn cabs(self) -> Result<Self> {
        self.checked_abs().ok_or_else(|| {
            Error::new(format!(
                "failed to compute abs({:?}): {} overflow",
                self, "isize"
            ))
        })
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`isize::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "isize"
            ))
        })
    }
    ///Checked square root: computes `√self`, returning an error if `self` is negative.
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
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`isize::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add_signed`]
    fn cadd_signed(self, other: i8) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub_signed`]
    fn csub_signed(self, other: i8) -> Result<Self>;
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i8>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u8::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u8::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u8::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u8"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i8) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_signed({:?}, {:?}): {} overflow",
                self, other, "u8"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u8"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i8) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_signed({:?}, {:?}): {} overflow",
                self, other, "u8"
            ))
        })
    }
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i8> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute signed_diff({:?}, {:?}): {} overflow",
                self, other, "i8"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u8"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u8"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u8"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u8"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u8::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u8"
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u8::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "u8"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u8::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u8::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "u8"
            ))
        })
    }
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
                        "failed to compute next_multiple_of({:?}, {:?}): {} overflow",
                        self, other, "u8"
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
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "u8"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u16`]
pub trait U16Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add_signed`]
    fn cadd_signed(self, other: i16) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub_signed`]
    fn csub_signed(self, other: i16) -> Result<Self>;
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i16>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u16::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u16::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u16::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u16"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i16) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_signed({:?}, {:?}): {} overflow",
                self, other, "u16"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u16"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i16) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_signed({:?}, {:?}): {} overflow",
                self, other, "u16"
            ))
        })
    }
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i16> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute signed_diff({:?}, {:?}): {} overflow",
                self, other, "i16"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u16"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u16"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u16"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u16"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u16::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u16"
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u16::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "u16"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u16::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u16::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "u16"
            ))
        })
    }
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
                        "failed to compute next_multiple_of({:?}, {:?}): {} overflow",
                        self, other, "u16"
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
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "u16"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u32`]
pub trait U32Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add_signed`]
    fn cadd_signed(self, other: i32) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub_signed`]
    fn csub_signed(self, other: i32) -> Result<Self>;
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i32>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u32::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u32::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u32::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u32"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i32) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_signed({:?}, {:?}): {} overflow",
                self, other, "u32"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u32"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i32) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_signed({:?}, {:?}): {} overflow",
                self, other, "u32"
            ))
        })
    }
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i32> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute signed_diff({:?}, {:?}): {} overflow",
                self, other, "i32"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u32"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u32"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u32"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u32"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u32::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u32"
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u32::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "u32"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u32::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u32::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "u32"
            ))
        })
    }
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
                        "failed to compute next_multiple_of({:?}, {:?}): {} overflow",
                        self, other, "u32"
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
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "u32"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u64`]
pub trait U64Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add_signed`]
    fn cadd_signed(self, other: i64) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub_signed`]
    fn csub_signed(self, other: i64) -> Result<Self>;
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i64>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u64::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u64::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u64::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u64"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i64) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_signed({:?}, {:?}): {} overflow",
                self, other, "u64"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u64"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i64) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_signed({:?}, {:?}): {} overflow",
                self, other, "u64"
            ))
        })
    }
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i64> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute signed_diff({:?}, {:?}): {} overflow",
                self, other, "i64"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u64"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u64"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u64"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u64"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u64::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u64"
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u64::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "u64"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u64::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u64::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "u64"
            ))
        })
    }
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
                        "failed to compute next_multiple_of({:?}, {:?}): {} overflow",
                        self, other, "u64"
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
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "u64"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`u128`]
pub trait U128Ext: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add_signed`]
    fn cadd_signed(self, other: i128) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub_signed`]
    fn csub_signed(self, other: i128) -> Result<Self>;
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<i128>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u128::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u128::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`u128::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u128"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: i128) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_signed({:?}, {:?}): {} overflow",
                self, other, "u128"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u128"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: i128) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_signed({:?}, {:?}): {} overflow",
                self, other, "u128"
            ))
        })
    }
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<i128> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute signed_diff({:?}, {:?}): {} overflow",
                self, other, "i128"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "u128"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u128"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u128"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "u128"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`u128::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "u128"
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`u128::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "u128"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`u128::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`u128::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "u128"
            ))
        })
    }
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
                        "failed to compute next_multiple_of({:?}, {:?}): {} overflow",
                        self, other, "u128"
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
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "u128"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`usize`]
pub trait UsizeExt: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add`]
    fn cadd(self, other: Self) -> Result<Self>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add`]
    fn cadd_assign(&mut self, other: Self) -> Result<()>;
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add_signed`]
    fn cadd_signed(self, other: isize) -> Result<Self>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub`]
    fn csub(self, other: Self) -> Result<Self>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub`]
    fn csub_assign(&mut self, other: Self) -> Result<()>;
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub_signed`]
    fn csub_signed(self, other: isize) -> Result<Self>;
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_signed_diff`]
    fn csigned_diff(self, other: Self) -> Result<isize>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_mul`]
    fn cmul(self, other: Self) -> Result<Self>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_mul`]
    fn cmul_assign(&mut self, other: Self) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_div`]
    fn cdiv(self, divisor: Self) -> Result<Self>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_div`]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_div_euclid`]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_rem`]
    fn crem(self, divisor: Self) -> Result<Self>;
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_rem`]
    fn crem_assign(&mut self, divisor: Self) -> Result<()>;
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_rem_euclid`]
    fn crem_euclid(self, divisor: Self) -> Result<Self>;
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`usize::checked_ilog`]
    fn cilog(self, base: Self) -> Result<u32>;
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`usize::checked_ilog2`]
    fn cilog2(self) -> Result<u32>;
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
    ///
    ///Wrapper for [`usize::checked_ilog10`]
    fn cilog10(self) -> Result<u32>;
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_neg`]
    fn cneg(self) -> Result<Self>;
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shl`]
    fn cshl(self, other: u32) -> Result<Self>;
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shl`]
    fn cshl_assign(&mut self, other: u32) -> Result<()>;
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shr`]
    fn cshr(self, other: u32) -> Result<Self>;
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shr`]
    fn cshr_assign(&mut self, other: u32) -> Result<()>;
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_pow`]
    fn cpow(self, power: u32) -> Result<Self>;
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "usize"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked addition: computes `add_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_add_signed`]
    #[inline]
    #[track_caller]
    fn cadd_signed(self, other: isize) -> Result<Self> {
        self.checked_add_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute add_signed({:?}, {:?}): {} overflow",
                self, other, "usize"
            ))
        })
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "usize"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Self) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `sub_signed(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_sub_signed`]
    #[inline]
    #[track_caller]
    fn csub_signed(self, other: isize) -> Result<Self> {
        self.checked_sub_signed(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute sub_signed({:?}, {:?}): {} overflow",
                self, other, "usize"
            ))
        })
    }
    ///Checked subtraction:  computes `signed_diff(self, other)`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_signed_diff`]
    #[inline]
    #[track_caller]
    fn csigned_diff(self, other: Self) -> Result<isize> {
        self.checked_signed_diff(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute signed_diff({:?}, {:?}): {} overflow",
                self, other, "isize"
            ))
        })
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: Self) -> Result<Self> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "usize"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: Self) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: Self) -> Result<Self> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "usize"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
    ///Checked euclidian division: computes `div_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_div_euclid`]
    #[inline]
    #[track_caller]
    fn cdiv_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_div_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute div_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "usize"
                    )
                }
            })
        })
    }
    ///Checked remainder: computes `self % divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem(self, divisor: Self) -> Result<Self> {
        self.checked_rem(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} % {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} % {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "usize"
                    )
                }
            })
        })
    }
    ///Checked remainder assigement: executes `self %= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_rem`]
    #[inline]
    #[track_caller]
    fn crem_assign(&mut self, divisor: Self) -> Result<()> {
        *self = self.crem(divisor)?;
        Ok(())
    }
    ///Checked euclidian reminder: computes `rem_euclid(self, divisor)`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`usize::checked_rem_euclid`]
    #[inline]
    #[track_caller]
    fn crem_euclid(self, divisor: Self) -> Result<Self> {
        self.checked_rem_euclid(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): division by zero",
                        self, divisor
                    )
                } else {
                    format!(
                        "failed to compute rem_euclid({:?}, {:?}): {} overflow",
                        self, divisor, "usize"
                    )
                }
            })
        })
    }
    ///Checked logarithm: computes <code>log<sub>base</sub> self</code>, returning an error if `self` is negative or zero, or if `base` is less than 2.
    ///
    ///Wrapper for [`usize::checked_ilog`]
    #[inline]
    #[track_caller]
    fn cilog(self, base: Self) -> Result<u32> {
        self.checked_ilog(base).ok_or_else(|| {
            Error::new({
                if base < 2 {
                    format!(
                        "failed to compute ilog({:?}, {:?}): base is less than 2",
                        self, base
                    )
                } else {
                    format!(
                        "failed to compute ilog({:?}, {:?}): first argument is not positive",
                        self, base
                    )
                }
            })
        })
    }
    ///Checked base 2 logarithm: computes `ln self`, returning an error if `self` is negative or zero.
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
    ///Checked base 10 logarithm: computes <code>log<sub>10</sub> self</code>, returning an error if `self` is negative or zero.
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
    ///Checked negation: computes `-self`, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_neg`]
    #[inline]
    #[track_caller]
    fn cneg(self) -> Result<Self> {
        self.checked_neg().ok_or_else(|| {
            Error::new(format!(
                "failed to compute -({:?}): {} overflow",
                self, "usize"
            ))
        })
    }
    ///Checked shift left: computes `self << other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl(self, other: u32) -> Result<Self> {
        self.checked_shl(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} << {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift left assigement: executes `self <<= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shl`]
    #[inline]
    #[track_caller]
    fn cshl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshl(other)?;
        Ok(())
    }
    ///Checked shift right: computes `self >> other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr(self, other: u32) -> Result<Self> {
        self.checked_shr(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} >> {:?}: shift amount is too large",
                MaybeParens(self),
                MaybeParens(other)
            ))
        })
    }
    ///Checked shift right assigement: executes `self >>= other`, returning an error if `other` is greater or equal to the number of bits in the type.
    ///
    ///Wrapper for [`usize::checked_shr`]
    #[inline]
    #[track_caller]
    fn cshr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cshr(other)?;
        Ok(())
    }
    ///Checked exponentiation: computes <code>self<sup>power</sup></code>, returning an error if overflow occured.
    ///
    ///Wrapper for [`usize::checked_pow`]
    #[inline]
    #[track_caller]
    fn cpow(self, power: u32) -> Result<Self> {
        self.checked_pow(power).ok_or_else(|| {
            Error::new(format!(
                "failed to compute pow({:?}, {:?}): {} overflow",
                self, power, "usize"
            ))
        })
    }
    ///Checked next multiple of `other`, returning an error if overflow occured or if `other` is zero.
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
                        "failed to compute next_multiple_of({:?}, {:?}): {} overflow",
                        self, other, "usize"
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
                "failed to compute next_power_of_two({:?}): {} overflow",
                self, "usize"
            ))
        })
    }
}
///Enhanced checked arithmetics functions for [`Duration`]
pub trait DurationExt: Sealed {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_add`]
    fn cadd(self, other: Duration) -> Result<Duration>;
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_add`]
    fn cadd_assign(&mut self, other: Duration) -> Result<()>;
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_sub`]
    fn csub(self, other: Duration) -> Result<Duration>;
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_sub`]
    fn csub_assign(&mut self, other: Duration) -> Result<()>;
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_mul`]
    fn cmul(self, other: u32) -> Result<Duration>;
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_mul`]
    fn cmul_assign(&mut self, other: u32) -> Result<()>;
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`Duration::checked_div`]
    fn cdiv(self, divisor: u32) -> Result<Duration>;
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`Duration::checked_div`]
    fn cdiv_assign(&mut self, divisor: u32) -> Result<()>;
}
impl Sealed for Duration {}
impl DurationExt for Duration {
    ///Checked addition: computes `self + other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd(self, other: Duration) -> Result<Duration> {
        self.checked_add(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} + {:?}: {} overflow",
                self,
                MaybeParens(other),
                "Duration"
            ))
        })
    }
    ///Checked addition assigement: executes `self += other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_add`]
    #[inline]
    #[track_caller]
    fn cadd_assign(&mut self, other: Duration) -> Result<()> {
        *self = self.cadd(other)?;
        Ok(())
    }
    ///Checked subtraction:  computes `self - other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub(self, other: Duration) -> Result<Duration> {
        self.checked_sub(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} - {:?}: {} overflow",
                self,
                MaybeParens(other),
                "Duration"
            ))
        })
    }
    ///Checked subtraction assigement:  executes `self -= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_sub`]
    #[inline]
    #[track_caller]
    fn csub_assign(&mut self, other: Duration) -> Result<()> {
        *self = self.csub(other)?;
        Ok(())
    }
    ///Checked multiplication: computes `self * other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul(self, other: u32) -> Result<Duration> {
        self.checked_mul(other).ok_or_else(|| {
            Error::new(format!(
                "failed to compute {:?} * {:?}: {} overflow",
                self,
                MaybeParens(other),
                "Duration"
            ))
        })
    }
    ///Checked multiplication assigement: executes `self *= other`, returning an error if overflow occured.
    ///
    ///Wrapper for [`Duration::checked_mul`]
    #[inline]
    #[track_caller]
    fn cmul_assign(&mut self, other: u32) -> Result<()> {
        *self = self.cmul(other)?;
        Ok(())
    }
    ///Checked division: computes `self / divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`Duration::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv(self, divisor: u32) -> Result<Duration> {
        self.checked_div(divisor).ok_or_else(|| {
            Error::new({
                if divisor == 0 {
                    format!(
                        "failed to compute {:?} / {:?}: division by zero",
                        self,
                        MaybeParens(divisor)
                    )
                } else {
                    format!(
                        "failed to compute {:?} / {:?}: {} overflow",
                        self,
                        MaybeParens(divisor),
                        "Duration"
                    )
                }
            })
        })
    }
    ///Checked division assigement: executes `self /= divisor`, returning an error if overflow occured or if `divisor` is zero.
    ///
    ///Wrapper for [`Duration::checked_div`]
    #[inline]
    #[track_caller]
    fn cdiv_assign(&mut self, divisor: u32) -> Result<()> {
        *self = self.cdiv(divisor)?;
        Ok(())
    }
}
