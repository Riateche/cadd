use {crate::util::assert_err, cadd::ext::U8Ext};

mod util;

#[test]
fn test_u8_ext() {
    assert_eq!(2_u8.cadd(3).unwrap(), 5);
    assert_err(2_u8.cadd(255), "failed to compute 2 + 255: u8 overflow");

    {
        let mut a: u8 = 2;
        a.cadd_assign(3).unwrap();
        assert_eq!(a, 5);
        assert_err(a.cadd_assign(255), "failed to compute 5 + 255: u8 overflow");
    }

    assert_eq!(2_u8.cadd_signed(3).unwrap(), 5);
    assert_eq!(2_u8.cadd_signed(-1).unwrap(), 1);
    assert_err(
        254_u8.cadd_signed(4),
        "failed to compute add_signed(254, 4): u8 overflow",
    );
    assert_err(
        2_u8.cadd_signed(-3),
        "failed to compute add_signed(2, -3): u8 overflow",
    );

    assert_eq!(2_u8.csub(1).unwrap(), 1);
    assert_err(2_u8.csub(3), "failed to compute 2 - 3: u8 overflow");

    {
        let mut a: u8 = 2;
        a.csub_assign(1).unwrap();
        assert_eq!(a, 1);
        assert_err(a.csub_assign(3), "failed to compute 1 - 3: u8 overflow");
    }

    assert_eq!(2_u8.csub_signed(1).unwrap(), 1);
    assert_eq!(2_u8.csub_signed(-1).unwrap(), 3);
    assert_err(
        2_u8.csub_signed(3),
        "failed to compute sub_signed(2, 3): u8 overflow",
    );
    assert_err(
        254_u8.csub_signed(-3),
        "failed to compute sub_signed(254, -3): u8 overflow",
    );

    assert_eq!(2_u8.csigned_diff(1).unwrap(), 1);
    assert_eq!(2_u8.csigned_diff(3).unwrap(), -1);
    assert_err(
        2_u8.csigned_diff(200),
        "failed to compute signed_diff(2, 200): i8 overflow",
    );
    assert_err(
        254_u8.csigned_diff(1),
        "failed to compute signed_diff(254, 1): i8 overflow",
    );

    assert_eq!(2_u8.cmul(3).unwrap(), 6);
    assert_err(2_u8.cmul(150), "failed to compute 2 * 150: u8 overflow");

    {
        let mut a: u8 = 2;
        a.cmul_assign(3).unwrap();
        assert_eq!(a, 6);
        assert_err(a.cmul_assign(150), "failed to compute 6 * 150: u8 overflow");
    }

    assert_eq!(2_u8.cdiv(3).unwrap(), 0);
    assert_eq!(200_u8.cdiv(10).unwrap(), 20);
    assert_err(2_u8.cdiv(0), "failed to compute 2 / 0: division by zero");

    {
        let mut a: u8 = 2;
        a.cdiv_assign(3).unwrap();
        assert_eq!(a, 0);
        assert_err(
            a.cdiv_assign(0),
            "failed to compute 0 / 0: division by zero",
        );
    }

    assert_eq!(2_u8.cdiv_euclid(3).unwrap(), 0);
    assert_eq!(200_u8.cdiv_euclid(10).unwrap(), 20);
    assert_err(
        2_u8.cdiv_euclid(0),
        "failed to compute div_euclid(2, 0): division by zero",
    );

    assert_eq!(2_u8.crem(3).unwrap(), 2);
    assert_eq!(200_u8.crem(10).unwrap(), 0);
    assert_err(2_u8.crem(0), "failed to compute 2 % 0: division by zero");

    {
        let mut a: u8 = 2;
        a.crem_assign(3).unwrap();
        assert_eq!(a, 2);
        assert_err(
            a.crem_assign(0),
            "failed to compute 2 % 0: division by zero",
        );
    }

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
    assert_err(1_u8.cneg(), "failed to compute -(1): u8 overflow");

    assert_eq!(3_u8.cshl(0).unwrap(), 3);
    assert_eq!(3_u8.cshl(1).unwrap(), 6);
    assert_eq!(3_u8.cshl(7).unwrap(), 128);
    assert_eq!(4_u8.cshl(7).unwrap(), 0);
    assert_err(
        3_u8.cshl(8),
        "failed to compute 3 << 8: shift amount is too large",
    );

    {
        let mut a: u8 = 3;
        a.cshl_assign(1).unwrap();
        assert_eq!(a, 6);
        assert_err(
            a.cshl_assign(8),
            "failed to compute 6 << 8: shift amount is too large",
        );
    }

    assert_eq!(3_u8.cshr(0).unwrap(), 3);
    assert_eq!(3_u8.cshr(1).unwrap(), 1);
    assert_eq!(3_u8.cshr(7).unwrap(), 0);
    assert_err(
        3_u8.cshr(8),
        "failed to compute 3 >> 8: shift amount is too large",
    );

    {
        let mut a: u8 = 3;
        a.cshr_assign(1).unwrap();
        assert_eq!(a, 1);
        assert_err(
            a.cshr_assign(8),
            "failed to compute 1 >> 8: shift amount is too large",
        );
    }

    assert_eq!(3_u8.cpow(3).unwrap(), 27);
    assert_err(3_u8.cpow(100), "failed to compute pow(3, 100): u8 overflow");

    assert_eq!(3_u8.cnext_multiple_of(16).unwrap(), 16);
    assert_err(
        129_u8.cnext_multiple_of(128),
        "failed to compute next_multiple_of(129, 128): u8 overflow",
    );
    assert_err(
        129_u8.cnext_multiple_of(0),
        "failed to compute next_multiple_of(129, 0): multiplier is zero",
    );

    assert_eq!(3_u8.cnext_power_of_two().unwrap(), 4);
    assert_err(
        129_u8.cnext_power_of_two(),
        "failed to compute next_power_of_two(129): u8 overflow",
    );
}
