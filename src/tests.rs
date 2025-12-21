use {
    crate::{ops_ext::U8Ext, prelude::*, Result},
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
    assert_eq!(2u8.cadd(3).unwrap(), 5);
    assert_err(200u8.cadd(100), "failed to compute 200 + 100: overflow");
    assert_err(
        (-5i32).cinto_type::<u32>(),
        "failed to convert value -5 from i32 to u32: value is out of bounds",
    );

    let _a = 2u32.to_non_zero().unwrap();
    let _b = non_zero(2u32).unwrap();
    assert_err(0u32.to_non_zero(), "unexpected zero value");
    assert_err(non_zero(0u32), "unexpected zero value");
}

#[test]
fn test_u8_ext() {
    assert_eq!(2_u8.cadd(3).unwrap(), 5);
    assert_err(2_u8.cadd(255), "failed to compute 2 + 255: overflow");

    assert_eq!(2_u8.cadd_signed(3).unwrap(), 5);
    assert_eq!(2_u8.cadd_signed(-1).unwrap(), 1);
    assert_err(254_u8.cadd_signed(4), "failed to compute 254 + 4: overflow");
    assert_err(2_u8.cadd_signed(-3), "failed to compute 2 + (-3): overflow");

    assert_eq!(2_u8.csub(1).unwrap(), 1);
    assert_err(2_u8.csub(3), "failed to compute 2 - 3: overflow");

    assert_eq!(2_u8.csub_signed(1).unwrap(), 1);
    assert_eq!(2_u8.csub_signed(-1).unwrap(), 3);
    assert_err(2_u8.csub_signed(3), "failed to compute 2 - 3: overflow");
    assert_err(
        254_u8.csub_signed(-3),
        "failed to compute 254 - (-3): overflow",
    );

    assert_eq!(2_u8.csigned_diff(1).unwrap(), 1);
    assert_eq!(2_u8.csigned_diff(3).unwrap(), -1);
    assert_err(
        2_u8.csigned_diff(200),
        "failed to compute 2 - 200: overflow",
    );
    assert_err(
        254_u8.csigned_diff(1),
        "failed to compute 254 - 1: overflow",
    );

    assert_eq!(2_u8.cmul(3).unwrap(), 6);
    assert_err(2_u8.cmul(150), "failed to compute 2 * 150: overflow");

    assert_eq!(2_u8.cdiv(3).unwrap(), 0);
    assert_eq!(200_u8.cdiv(10).unwrap(), 20);
    assert_err(2_u8.cdiv(0), "failed to compute 2 / 0: division by zero");

    assert_eq!(2_u8.cdiv_euclid(3).unwrap(), 0);
    assert_eq!(200_u8.cdiv_euclid(10).unwrap(), 20);
    assert_err(
        2_u8.cdiv_euclid(0),
        "failed to compute div_euclid(2, 0): division by zero",
    );

    assert_eq!(2_u8.crem(3).unwrap(), 2);
    assert_eq!(200_u8.crem(10).unwrap(), 0);
    assert_err(2_u8.crem(0), "failed to compute 2 % 0: division by zero");

    assert_eq!(2_u8.crem_euclid(3).unwrap(), 2);
    assert_eq!(200_u8.crem_euclid(10).unwrap(), 0);
    assert_err(
        2_u8.crem_euclid(0),
        "failed to compute rem_euclid(2, 0): division by zero",
    );

    assert_eq!(9_u8.cilog(3).unwrap(), 2);
    assert_err(
        9_u8.cilog(1),
        "failed to compute ilog(9, 1): base is less than 2",
    );
    assert_err(
        9_u8.cilog(0),
        "failed to compute ilog(9, 0): base is less than 2",
    );
    assert_err(
        0_u8.cilog(3),
        "failed to compute ilog(0, 3): first argument is not positive",
    );

    assert_eq!(9_u8.cilog2().unwrap(), 3);
    assert_err(
        0_u8.cilog2(),
        "failed to compute ilog2(0): argument is not positive",
    );

    assert_eq!(9_u8.cilog10().unwrap(), 0);
    assert_eq!(100_u8.cilog10().unwrap(), 2);
    assert_err(
        0_u8.cilog10(),
        "failed to compute ilog10(0): argument is not positive",
    );

    assert_eq!(0_u8.cneg().unwrap(), 0);
    assert_err(1_u8.cneg(), "failed to compute -(1): overflow");

    assert_eq!(3_u8.cshl(0).unwrap(), 3);
    assert_eq!(3_u8.cshl(1).unwrap(), 6);
    assert_eq!(3_u8.cshl(7).unwrap(), 128);
    assert_eq!(4_u8.cshl(7).unwrap(), 0);
    assert_err(
        3_u8.cshl(8),
        "failed to compute 3 << 8: shift amount is too large",
    );

    assert_eq!(3_u8.cshr(0).unwrap(), 3);
    assert_eq!(3_u8.cshr(1).unwrap(), 1);
    assert_eq!(3_u8.cshr(7).unwrap(), 0);
    assert_err(
        3_u8.cshr(8),
        "failed to compute 3 >> 8: shift amount is too large",
    );

    assert_eq!(3_u8.cpow(3).unwrap(), 27);
    assert_err(3_u8.cpow(100), "failed to compute pow(3, 100): overflow");

    assert_eq!(3_u8.cnext_multiple_of(16).unwrap(), 16);
    assert_err(
        129_u8.cnext_multiple_of(128),
        "failed to compute next_multiple_of(129, 128): overflow",
    );
    assert_err(
        129_u8.cnext_multiple_of(0),
        "failed to compute next_multiple_of(129, 0): multiplier is zero",
    );

    assert_eq!(3_u8.cnext_power_of_two().unwrap(), 4);
    assert_err(
        129_u8.cnext_power_of_two(),
        "failed to compute next_power_of_two(129): overflow",
    );
}
