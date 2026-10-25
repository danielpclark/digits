extern crate digits;
use digits::prelude::*;
use digits::Radix;

fn num(chars: &str, value: &str) -> Digits {
  Digits::new(BaseCustom::<char>::new(chars.chars().collect()), value.to_string())
}

#[test]
fn multiplies_single_hex_digits() {
  assert_eq!(num("0123456789abcdef", "f").mul(num("0123456789abcdef", "f")).to_s(), "e1");
}

#[test]
fn multiplies_multi_digit_hex() {
  assert_eq!(num("0123456789abcdef", "ff").mul(num("0123456789abcdef", "ff")).to_s(), "fe01");
}

#[test]
fn raises_hex_to_a_power() {
  let mut c = num("0123456789abcdef", "c");
  assert_eq!(c.pow(num("0123456789abcdef", "3")).to_s(), "6c0");
}

#[test]
fn multiplies_in_bases_smaller_than_ten() {
  assert_eq!(num("0123", "3").mul(num("0123", "3")).to_s(), "21");
  assert_eq!(num("012", "2").mul(num("012", "2")).to_s(), "11");
  assert_eq!(num("01234567", "7").mul(num("01234567", "7")).to_s(), "61");
}

#[test]
fn multiplies_in_a_base_without_arabic_numerals() {
  assert_eq!(num("abcdefghij", "j").mul(num("abcdefghij", "j")).to_s(), "ib");
}

#[test]
fn mut_mul_and_operators_agree_in_hex() {
  let mut x = num("0123456789abcdef", "ff");
  x.mut_mul(num("0123456789abcdef", "ff"));
  assert_eq!(x.to_s(), "fe01");
  assert_eq!((num("0123456789abcdef", "ff") * num("0123456789abcdef", "ff")).to_s(), "fe01");
}

#[test]
fn down_casting_into_octal_uses_correct_multiplication() {
  assert_eq!(num("0123456789", "77").octal().to_s(), "115");
}

#[test]
fn decimal_results_are_unchanged() {
  assert_eq!(num("0123456789", "11").mul(num("0123456789", "2")).to_s(), "22");
  assert_eq!(num("0123456789", "99").mul(num("0123456789", "99")).to_s(), "9801");
  let mut x = num("0123456789", "12");
  assert_eq!(x.pow(num("0123456789", "3")).to_s(), "1728");
}

// Property check: mul agrees with native u128 arithmetic across many bases.
#[test]
fn multiplication_matches_native_arithmetic_in_every_base() {
  let bases = ["01", "012", "0123", "01234567", "0123456789", "0123456789abcdef",
               "abcdefghij", "ZYXWVUTSRQ", "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"];
  let mut seed: u64 = 0x2545F4914F6CDD1D;
  let mut rnd = || { seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; seed };
  for chars in bases.iter() {
    let base = BaseCustom::<char>::new(chars.chars().collect());
    let radix = base.base as u128;
    for _ in 0..40 {
      let a = (rnd() % 5000) as u64;
      let b = (rnd() % 300) as u64;
      let expected = base.gen(0); // placeholder to keep types simple
      let _ = expected;
      let product = (a as u128) * (b as u128);
      // encode expected product in this base without using Digits
      let mut digits = vec![];
      let mut p = product;
      if p == 0 { digits.push(0); }
      while p > 0 { digits.push((p % radix) as u64); p /= radix; }
      digits.reverse();
      let expected: String = digits.iter().map(|&d| *base.nth(d as usize).unwrap()).collect();
      let got = Digits::new(base.clone(), base.gen(a)).mul(Digits::new(base.clone(), base.gen(b))).to_s();
      assert_eq!(got, expected, "{} * {} in base {:?}", a, b, chars);
    }
  }
}
