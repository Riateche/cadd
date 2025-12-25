use {
    crate::util::assert_err,
    cadd::{
        ext::U8Ext,
        ops::cadd_unsigned,
        prelude::{non_zero, Cinto, IntoType, ToNonZero},
    },
};

mod util;

#[test]
fn test1() {
    assert_eq!(2u8.cadd(3).unwrap(), 5);
    assert_err(200u8.cadd(100), "failed to compute 200 + 100: u8 overflow");
    assert_err(
        (-5i32).cinto_type::<u32>(),
        "failed to convert value -5 from i32 to u32: value is out of bounds",
    );

    let _a = 2u32.to_non_zero().unwrap();
    let _b = non_zero(2u32).unwrap();
    assert_err(0u32.to_non_zero(), "unexpected zero value");
    assert_err(non_zero(0u32), "unexpected zero value");
}

fn _inference1(y: u32) -> cadd::Result<i32> {
    let x: i32 = y.cinto()?;
    cadd_unsigned(x, y)
}
