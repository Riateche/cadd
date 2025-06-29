use {
    crate::{prelude::*, Result},
    alloc::format,
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
    ENABLED.store(enabled as u8 + 1, Ordering::Relaxed);
    enabled
}

fn _inference1(y: u32) -> crate::Result<i32> {
    let x: i32 = y.cinto()?;
    cadd(x, y)
}

#[track_caller]
fn assert_err<T: Debug>(value: Result<T>, expected: &str) {
    let actual = value.expect_err("expected error").to_string();

    if backtrace_enabled() {
        assert!(actual.starts_with(&format!("{}\nstack backtrace:\n", expected)));
    } else {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test1() {
    assert_eq!(2u8.cadd(3u8).unwrap(), 5);
    assert_err(200u8.cadd(100u8), "overflow: 200 + 100");
    assert_err(
        (-5i32).cinto_type::<u32>(),
        "cannot convert value -5 from i32 to u32: value is out of bounds",
    );

    let _a = 2u32.to_non_zero().unwrap();
    let _b = non_zero(2u32).unwrap();
    assert_err(0u32.to_non_zero(), "unexpected zero value");
    assert_err(non_zero(0u32), "unexpected zero value");
}
