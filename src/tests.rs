use {
    crate::{cadd, Cadd, Cinto, IntoType, Result, ToNonZero},
    std::{fmt::Debug, string::ToString},
};

fn _inference1(y: u32) -> crate::Result<i32> {
    let x: i32 = y.cinto()?;
    cadd(x, y)
}

#[track_caller]
fn assert_err<T: Debug>(value: Result<T>, err: &str) {
    assert_eq!(value.expect_err("expected error").to_string(), err);
}

#[test]
fn test1() {
    assert_eq!(2u8.cadd(3u8).unwrap(), 5);
    assert_err(200u8.cadd(100u8), "overflow: 200 + 100");

    assert_err(
        (-5i32).cinto_type::<u32>(),
        "cannot convert value -5 from i32 to u32: value is out of bounds",
    );

    assert_err(0u32.to_non_zero(), "unexpected zero value");
}
