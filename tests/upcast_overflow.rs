extern crate digits;
use digits::prelude::*;
use digits::Radix;

fn binary(value: &str) -> Digits {
  Digits::new(BaseCustom::<char>::new("01".chars().collect()), value.to_string())
}

fn decimal(value: &str) -> Digits {
  Digits::new(BaseCustom::<char>::new("0123456789".chars().collect()), value.to_string())
}

#[test]
fn up_casts_values_beyond_u64_max() {
  // 2^64 does not fit in a u64; master returned "0" in release and panicked in debug.
  assert_eq!(binary(&format!("1{}", "0".repeat(64))).decimal().to_s(), "18446744073709551616");
  assert_eq!(binary(&format!("1{}", "0".repeat(70))).decimal().to_s(), "1180591620717411303424");
}

#[test]
fn up_casts_mixed_digits_beyond_u64_max() {
  // 2^64 + 2^63 + 1
  let value = format!("11{}1", "0".repeat(62));
  assert_eq!(binary(&value).decimal().to_s(), "27670116110564327425");
}

#[test]
fn up_casts_into_non_decimal_targets_beyond_u64_max() {
  // 10^20 in hex is 56BC75E2D63100000; the target base multiplication is never used.
  assert_eq!(decimal(&format!("1{}", "0".repeat(20))).hex().to_s(), "56BC75E2D63100000");
}

#[test]
fn values_within_u64_are_unchanged() {
  assert_eq!(binary(&format!("1{}", "0".repeat(63))).decimal().to_s(), "9223372036854775808");
  assert_eq!(decimal("255").hex().to_s(), "FF");
  assert_eq!(decimal("4096").hex().to_s(), "1000");
  assert_eq!(binary("1111101000").decimal().to_s(), "1000");
  assert_eq!(decimal("0").hex().to_s(), "0");
}
