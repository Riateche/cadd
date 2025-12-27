//! Test utils.

use {
    cadd::{prelude::IntoType, Result},
    core::sync::atomic::{AtomicU8, Ordering},
    std::{env, fmt::Debug, string::ToString},
};

fn backtrace_enabled() -> bool {
    // Cache the result of reading the environment variables to make
    // backtrace captures speedy, because otherwise reading environment
    // variables every time can be somewhat slow.
    static ENABLED: AtomicU8 = AtomicU8::new(0);
    match ENABLED.load(Ordering::Relaxed) {
        0 => {}
        1 => return false,
        _ => return true,
    }
    let enabled = match env::var("RUST_LIB_BACKTRACE") {
        Ok(s) => s != "0",
        Err(_) => match env::var("RUST_BACKTRACE") {
            Ok(s) => s != "0",
            Err(_) => false,
        },
    };
    #[expect(clippy::arithmetic_side_effects, reason = "never overflows")]
    ENABLED.store(enabled.into_type::<u8>() + 1, Ordering::Relaxed);
    enabled
}

/// Checks that `value` is an expected error.
#[expect(clippy::expect_used, reason = "tests")]
#[track_caller]
pub fn assert_err<T: Debug>(value: Result<T>, expected: &str) {
    let actual = value.expect_err("expected error").to_string();

    if backtrace_enabled() {
        assert!(actual.starts_with(&format!("{}\nstack backtrace:\n", expected)));
    } else {
        assert_eq!(actual, expected);
    }
}
