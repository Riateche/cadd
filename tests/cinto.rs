use {
    crate::util::assert_err,
    cadd::prelude::{Cinto, IntoType},
    std::ffi::CString,
};

mod util;

#[test]
fn test_int() {
    let x: i32 = 5;
    let y: u32 = x.cinto().unwrap();
    assert_eq!(y, 5);

    let x2: i32 = -5;
    let y2: Result<u32, _> = x2.cinto();

    assert_err(
        y2,
        "failed to convert value -5 from i32 to u32: value is out of bounds",
    );
}

#[test]
fn test_bytes_vec_to_string() {
    let a = vec![104, 101, 108, 108, 111];
    let b = a.cinto_type::<String>().unwrap();
    assert_eq!(b, "hello");

    let a2 = vec![104, 128, 108, 108, 111];
    assert_err(
        a2.cinto_type::<String>(),
        r#"failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 1; input: [104, 128, 108, 108, 111] (5 bytes); input as lossy utf-8: "h�llo""#,
    );

    let a3: Vec<u8> = [128].into_iter().chain(1..=100).collect();
    assert_err(
        a3.cinto_type::<String>(),
        r#"failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 0; input: [128, 1, 2, 3, 4, 5, 6, 7, 8, 9, .., 91, 92, 93, 94, 95, 96, 97, 98, 99, 100] (101 bytes); input as lossy utf-8: "�\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t".."[\\]^_`abcd""#,
    );

    let a4: Vec<u8> = [128]
        .into_iter()
        .chain(1..=100)
        .chain(1..=100)
        .chain(1..=100)
        .collect();
    assert_err(
        a4.cinto_type::<String>(),
        "failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 0; input: [128, 1, 2, 3, 4, 5, 6, 7, 8, 9, .., 91, 92, 93, 94, 95, 96, 97, 98, 99, 100] (301 bytes)",
    );
}

#[test]
fn test_bytes_slice_to_str() {
    let a: &[u8] = &[104, 101, 108, 108, 111];
    let b: &str = a.cinto().unwrap();
    assert_eq!(b, "hello");

    let a2: &[u8] = &[104, 128, 108, 108, 111];
    assert_err(
        a2.cinto_type::<&str>(),
        r#"failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 1; input: [104, 128, 108, 108, 111] (5 bytes); input as lossy utf-8: "h�llo""#,
    );

    let a3: Vec<u8> = [128].into_iter().chain(1..=100).collect();
    assert_err(
        (&*a3).cinto_type::<&str>(),
        r#"failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 0; input: [128, 1, 2, 3, 4, 5, 6, 7, 8, 9, .., 91, 92, 93, 94, 95, 96, 97, 98, 99, 100] (101 bytes); input as lossy utf-8: "�\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t".."[\\]^_`abcd""#,
    );

    let a4: Vec<u8> = [128]
        .into_iter()
        .chain(1..=100)
        .chain(1..=100)
        .chain(1..=100)
        .collect();
    assert_err(
        (&*a4).cinto_type::<&str>(),
        "failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 0; input: [128, 1, 2, 3, 4, 5, 6, 7, 8, 9, .., 91, 92, 93, 94, 95, 96, 97, 98, 99, 100] (301 bytes)",
    );
}

#[test]
fn test_cstring_to_string() {
    let a = CString::from_vec_with_nul(vec![104, 101, 108, 108, 111, 0]).unwrap();

    let c = a.as_c_str().cinto_type::<&str>().unwrap();
    assert_eq!(c, "hello");

    let b = a.cinto_type::<String>().unwrap();
    assert_eq!(b, "hello");

    let a2 = CString::from_vec_with_nul(vec![104, 128, 108, 108, 111, 0]).unwrap();

    assert_err(
        a2.as_c_str().cinto_type::<&str>(),
        r#"failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 1; input: [104, 128, 108, 108, 111] (5 bytes); input as lossy utf-8: "h�llo""#,
    );

    assert_err(
        a2.cinto_type::<String>(),
        r#"failed to convert bytes to string: invalid utf-8 sequence of 1 bytes from index 1; input: [104, 128, 108, 108, 111] (5 bytes); input as lossy utf-8: "h�llo""#,
    );
}

#[test]
#[cfg(unix)]
fn test_os_string_to_string() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    let a = OsString::from_vec(vec![104, 101, 108, 108, 111]);

    let c = a.as_os_str().cinto_type::<&str>().unwrap();
    assert_eq!(c, "hello");

    let b = a.cinto_type::<String>().unwrap();
    assert_eq!(b, "hello");

    let a2 = OsString::from_vec(vec![104, 128, 108, 108, 111]);

    assert_err(
        a2.as_os_str().cinto_type::<&str>(),
        r#"failed to convert OS string "h\x80llo" to utf-8: invalid utf-8 sequence of 1 bytes from index 1"#,
    );

    assert_err(
        a2.cinto_type::<String>(),
        r#"failed to convert OS string "h\x80llo" to utf-8: invalid utf-8 sequence of 1 bytes from index 1"#,
    );
}

#[test]
#[cfg(unix)]
fn test_path_to_string() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};

    let a = PathBuf::from(OsString::from_vec(vec![104, 101, 108, 108, 111]));

    let c = a.as_path().cinto_type::<&str>().unwrap();
    assert_eq!(c, "hello");

    let b = a.cinto_type::<String>().unwrap();
    assert_eq!(b, "hello");

    let a2 = PathBuf::from(OsString::from_vec(vec![104, 128, 108, 108, 111]));

    assert_err(
        a2.as_path().cinto_type::<&str>(),
        r#"failed to convert OS path "h\x80llo" to utf-8: invalid utf-8 sequence of 1 bytes from index 1"#,
    );

    assert_err(
        a2.cinto_type::<String>(),
        r#"failed to convert OS path "h\x80llo" to utf-8: invalid utf-8 sequence of 1 bytes from index 1"#,
    );
}

#[test]
fn slice_to_array() {
    let a: &[u32] = &[1, 2, 3, 4];
    let b: &[u32; 4] = a.cinto().unwrap();
    assert_eq!(b, &[1, 2, 3, 4]);
    let c: [u32; 4] = a.cinto().unwrap();
    assert_eq!(&c, &[1, 2, 3, 4]);

    let b2: cadd::Result<&[u32; 5]> = a.cinto();
    assert_err(b2, "expected 5 items, got [1, 2, 3, 4] (4 items)");
    let c2: cadd::Result<[u32; 5]> = a.cinto();
    assert_err(c2, "expected 5 items, got [1, 2, 3, 4] (4 items)");

    assert_err(
        a.cinto_type::<[u32; 2]>(),
        "expected 2 items, got [1, 2, 3, 4] (4 items)",
    );
    assert_err(
        a.cinto_type::<&[u32; 2]>(),
        "expected 2 items, got [1, 2, 3, 4] (4 items)",
    );

    let mut bufs2: Vec<String> = (1..=10).map(|i| format!("s{i}")).collect();
    let mut refs2: Vec<&mut str> = bufs2.iter_mut().map(|s| &mut **s).collect();
    let a2: &mut [&mut str] = &mut refs2;
    assert_eq!(
        a2.cinto_type::<&mut [&mut str; 10]>().unwrap(),
        &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10"]
    );
    assert_err(
        a2.cinto_type::<&mut [&mut str; 20]>(),
        r#"expected 20 items, got ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10"] (10 items)"#,
    );

    let bufs3: Vec<String> = (1..=30).map(|i| format!("s{i}")).collect();
    let refs3: Vec<&str> = bufs3.iter().map(|s| s.as_str()).collect();
    let a3: &[&str] = &refs3;
    assert_err(
        a3.cinto_type::<&[&str; 5]>(),
        r#"expected 5 items, got ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", .., "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30"] (30 items)"#,
    );
    assert_err(
        a3.cinto_type::<[&str; 5]>(),
        r#"expected 5 items, got ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", .., "s21", "s22", "s23", "s24", "s25", "s26", "s27", "s28", "s29", "s30"] (30 items)"#,
    );
}
