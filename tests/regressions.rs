// Regression tests for bugs in 1.1.1.  Each test failed (or hung) on 1.1.1.
extern crate digits;
use digits::prelude::*;
use digits::radices::*;
use digits::Radix;

fn num(base: BaseCustom<char>, s: &str) -> Digits {
  Digits::new(base, s.to_string())
}

#[test]
fn multiplies_in_hex() {
  let f = num(hexl_base(), "f");
  assert_eq!(f.mul(f.clone()).to_s(), "e1"); // 1.1.1 gave "225"
  let ff = num(hexl_base(), "ff");
  assert_eq!(ff.mul(ff.propagate("2".to_string())).to_s(), "1fe"); // 1.1.1 gave "330"
}

#[test]
fn multiplies_with_non_numeric_characters() {
  let base = BaseCustom::<char>::new("ABCDEFGHIJ".chars().collect());
  let c = num(base, "C");
  assert_eq!(c.mul(c.propagate("D".to_string())).to_s(), "G"); // 1.1.1 panicked
}

#[test]
fn mut_mul_by_zero_is_zero() {
  let mut twelve = num(decimal_base(), "12");
  let zero = twelve.zero();
  assert_eq!(twelve.mut_mul(zero).to_s(), "0"); // 1.1.1 gave "10"
}

#[test]
fn pow_of_zero_sets_one() {
  let mut seven = num(decimal_base(), "7");
  let zero = seven.zero();
  seven.pow(zero);
  assert_eq!(seven.to_s(), "1"); // 1.1.1 left it at "7"
}

#[test]
fn converts_numbers_wider_than_u64() {
  let binary = num(binary_base(), &format!("1{}", "0".repeat(64)));
  // 1.1.1 overflowed: "0" in release builds, a panic in debug builds
  assert_eq!(binary.decimal().to_s(), "18446744073709551616");
  let big = num(decimal_base(), "123456789012345678901234567890");
  assert_eq!(big.hex().decimal(), big);
}

#[test]
fn converts_between_mappings_of_the_same_size() {
  let upper = num(hex_base(), "FF");
  // 1.1.1 only compared base sizes and returned "FF" in the uppercase mapping
  assert_eq!(Digits::from((Digits::new_zero(hexl_base()), upper.clone())).to_s(), "ff");
  assert_eq!(upper.hexl().to_s(), "ff");
}

#[test]
fn adjacency_limit_is_not_truncated() {
  let mut one = num(decimal_base(), "1");
  // 1.1.1 cast the limit to u8, so 256 behaved like 0 and skipped to "2"
  assert_eq!(one.next_non_adjacent(256).to_s(), "2");
  let mut ten = num(decimal_base(), "10");
  assert_eq!(ten.next_non_adjacent(256).to_s(), "11");
}

#[test]
fn handles_very_long_numbers_without_recursion() {
  // 1.1.1 overflowed the stack on numbers of a few thousand digits.  Run on a
  // small stack to show nothing recurses per digit.
  std::thread::Builder::new()
    .stack_size(64 * 1024)
    .spawn(|| {
      let mut nines = num(decimal_base(), &"9".repeat(200_000));
      nines.succ();
      assert_eq!(nines.length(), 200_001);
      nines.pred_till_zero();
      assert_eq!(nines.max_adjacent(), 199_999);
      assert!(nines.clone() == nines);
    })
    .unwrap()
    .join()
    .unwrap();
}

#[test]
fn multiplies_large_numbers() {
  let a = num(decimal_base(), &"7".repeat(2_000));
  let product = a.mul(a.propagate("3".repeat(2_000)));
  assert_eq!(product.length(), 4_000);
  assert!(product.to_s().starts_with("259"));
  let mut two = num(decimal_base(), "2");
  let exponent = two.gen(1_000u64);
  two.pow(exponent);
  assert!(two.to_s().starts_with("10715086071862673"));
}

#[test]
fn single_character_bases_hold_zero_and_panic_instead_of_hanging() {
  // `['a', 'a']` is a base of one character; 1.1.1 looped forever counting up.
  let one_unit = || BaseCustom::<char>::new(vec!['a', 'a']);
  assert!(num(one_unit(), "aaa").is_zero());
  let counted = std::panic::catch_unwind(|| num(one_unit(), "a").succ().to_s());
  assert!(counted.is_err());
}

#[test]
fn zero_padding_follows_1_1_1() {
  let n = |s: &str| num(decimal_base(), s);
  assert_eq!(n("1").add(n("0011")).to_s(), "0012");
  assert_eq!(n("1").mut_add(n("0011")).to_s(), "12");
  assert_eq!(n("0099").succ().to_s(), "0100");
  assert_eq!(n("0002").mul(n("3")).to_s(), "6");
  assert_eq!(n("0002").mut_mul(n("3")).to_s(), "0006");
  assert_eq!(n("03").pow(n("2")).to_s(), "09");
  assert_eq!(n("010").pow(n("1")).to_s(), "010");
  assert_eq!(n("0009").hex().to_s(), "9");
  assert!(n("009") != n("9"));
  assert_eq!(n("009").partial_cmp(&n("9")), Some(std::cmp::Ordering::Equal));
}
