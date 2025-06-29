#[cfg(feature = "std")]
use std::backtrace::{Backtrace, BacktraceStatus};

use {
    alloc::{boxed::Box, string::String},
    core::fmt::{self, Debug, Display, Formatter},
};

/// A general error with a message and a backtrace (if enabled).
pub struct Error(Box<ErrorInner>);

struct ErrorInner {
    message: String,
    #[cfg(feature = "std")]
    backtrace: Backtrace,
}

impl Error {
    /// Creates a new error and captures the backtrace (if enabled).
    pub fn new(message: String) -> Self {
        Self(Box::new(ErrorInner {
            message,
            #[cfg(feature = "std")]
            backtrace: Backtrace::capture(),
        }))
    }

    /// Description of the error.
    pub fn message(&self) -> &str {
        &self.0.message
    }

    /// Backtrace to where the error was created.
    #[cfg(feature = "std")]
    pub fn backtrace(&self) -> &Backtrace {
        &self.0.backtrace
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
