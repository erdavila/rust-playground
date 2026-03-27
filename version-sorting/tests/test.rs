use std::cmp::Ordering;

use rand::seq::SliceRandom;
use test_case::test_case;
use version_sorting::version_sorting;

#[test]
fn test_version_sorting() {
    let expected = Vec::from([
        "_ZYXW", "_abcd", "A2", "ABCD", "Z_YXW", "ZY_XW", "ZY_XW", "ZYXW", "ZYXW_", "a1", "abcd",
        "u_zzz", "u8", "u16", "u32", "u64", "u128", "u256", "ua", "usize", "uz", "v000", "v00",
        "v0", "v0s", "v00t", "v0u", "v001", "v01", "v1", "v009", "v09", "v9", "v010", "v10",
        "w005s09t", "w5s009t", "x64", "x86", "x86_32", "x86_64", "x86_128", "x87", "zyxw",
    ]);
    let mut entries = expected.clone();

    let mut rng = rand::rng();
    entries.shuffle(&mut rng);

    entries.sort_by(version_sorting);

    assert_eq!(entries, expected);
}

#[test_case("_ZYXW", "_abcd")]
#[test_case("_abcd", "A2")]
#[test_case("A2", "ABCD")]
#[test_case("ABCD", "Z_YXW")]
#[test_case("Z_YXW", "ZY_XW")]
#[test_case("ZY_XW", "ZY_XW")]
#[test_case("ZY_XW", "ZYXW")]
#[test_case("ZYXW", "ZYXW_")]
#[test_case("ZYXW_", "a1")]
#[test_case("a1", "abcd")]
#[test_case("abcd", "u_zzz")]
#[test_case("u_zzz", "u8")]
#[test_case("u8", "u16")]
#[test_case("u16", "u32")]
#[test_case("u32", "u64")]
#[test_case("u64", "u128")]
#[test_case("u128", "u256")]
#[test_case("u256", "ua")]
#[test_case("ua", "usize")]
#[test_case("usize", "uz")]
#[test_case("uz", "v000")]
#[test_case("v000", "v00")]
#[test_case("v00", "v0")]
#[test_case("v0", "v0s")]
#[test_case("v0s", "v00t")]
#[test_case("v00t", "v0u")]
#[test_case("v0u", "v001")]
#[test_case("v001", "v01")]
#[test_case("v01", "v1")]
#[test_case("v1", "v009")]
#[test_case("v009", "v09")]
#[test_case("v09", "v9")]
#[test_case("v9", "v010")]
#[test_case("v010", "v10")]
#[test_case("v10", "w005s09t")]
#[test_case("w005s09t", "w5s009t")]
#[test_case("w5s009t", "x64")]
#[test_case("x64", "x86")]
#[test_case("x86", "x86_32")]
#[test_case("x86_32", "x86_64")]
#[test_case("x86_64", "x86_128")]
#[test_case("x86_128", "x87")]
#[test_case("x87", "zyxw")]
fn pairs(a: &str, b: &str) {
    let ord = version_sorting(&a, &b);

    if a == b {
        assert_eq!(ord, Ordering::Equal);
    } else {
        assert_eq!(ord, Ordering::Less);
    }
}
