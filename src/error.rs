#[cfg(feature = "std")]
use std::backtrace::{Backtrace, BacktraceStatus};

use {
    alloc::{boxed::Box, string::String},
    core::fmt::{self, Debug, Display, Formatter},
};

#[non_exhaustive]
pub struct Error(Box<ErrorInner>);

struct ErrorInner {
    message: String,
    #[cfg(feature = "std")]
    backtrace: Backtrace,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self(Box::new(ErrorInner {
            message,
            #[cfg(feature = "std")]
            backtrace: Backtrace::capture(),
        }))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.message)?;
        #[cfg(feature = "std")]
        if self.0.backtrace.status() == BacktraceStatus::Captured {
            write!(f, "\nstack backtrace:\n{}", self.0.backtrace)?;
        }
        Ok(())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for Error {}
