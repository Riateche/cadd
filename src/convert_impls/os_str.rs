use {
    crate::{
        convert::{Cfrom, IntoType},
        Error, Result,
    },
    alloc::{format, string::String},
    core::fmt::Write,
    std::{
        ffi::{OsStr, OsString},
        path::{Path, PathBuf},
    },
};

impl<'a> Cfrom<&'a OsStr> for &'a str {
    type Error = Error;
    #[inline]
    fn cfrom(from: &'a OsStr) -> Result<Self> {
        from.try_into().map_err(|err| {
            Error::new(format!(
                "failed to convert OS string {from:?} to utf-8: {err}"
            ))
        })
    }
}

impl Cfrom<OsString> for String {
    type Error = Error;
    #[inline]
    fn cfrom(from: OsString) -> Result<Self> {
        from.into_string().map_err(|from| {
            // `OsString` doesn't expose API to get `Utf8Error`, but we can get it
            // using `impl TryFrom<&OsStr> for &str`.
            let mut text = format!("failed to convert OS string {from:?} to utf-8");
            if let Err(err) = from.as_os_str().try_into_type::<&str>() {
                write!(text, ": {err}").unwrap();
            }
            Error::new(text)
        })
    }
}

impl<'a> Cfrom<&'a Path> for &'a str {
    type Error = Error;
    #[inline]
    fn cfrom(from: &'a Path) -> Result<Self> {
        from.as_os_str().try_into().map_err(|err| {
            Error::new(format!(
                "failed to convert OS path {from:?} to utf-8: {err}"
            ))
        })
    }
}

impl Cfrom<PathBuf> for String {
    type Error = Error;
    #[inline]
    fn cfrom(from: PathBuf) -> Result<Self> {
        from.into_os_string()
            .into_string()
            .map_err(|from: OsString| {
                let from = PathBuf::from(from);
                // `OsString` doesn't expose API to get `Utf8Error`, but we can get it
                // using `impl TryFrom<&OsStr> for &str`.
                let mut text = format!("failed to convert OS path {from:?} to utf-8");
                if let Err(err) = from.as_os_str().try_into_type::<&str>() {
                    write!(text, ": {err}").unwrap();
                }
                Error::new(text)
            })
    }
}
