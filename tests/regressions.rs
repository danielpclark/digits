// Regression tests for bugs fixed in 2.0.0.  Each test failed on 1.1.1.
use digits::prelude::*;
use digits::radices::*;
use digits::Radix;
use std::collections::HashSet;

#[test]
fn multiplies_in_hex() {
  let f = Digits::new(hexl_base(), "f");
  assert_eq!(f.mul(&f).to_s(), "e1"); // 1.1.1 gave "225"
  let ff = Digits::new(hexl_base(), "ff");
  assert_eq!(ff.mul(ff.propagate("2")).to_s(), "1fe"); // 1.1.1 gave "330"
}

#[test]
fn multiplies_with_non_numeric_characters() {
  let base = BaseCustom::<char>::new("ABCDEFGHIJ".chars().collect());
  let c = Digits::new(base, "C");
  assert_eq!(c.mul(c.propagate("D")).to_s(), "G"); // 1.1.1 panicked
}

#[test]
fn mut_mul_by_zero_is_zero() {
  let mut twelve = Digits::new(decimal_base(), "12");
  twelve.mut_mul(twelve.zero());
  assert_eq!(twelve.to_s(), "0"); // 1.1.1 gave "10"
}

#[test]
fn pow_of_zero_sets_one() {
  let mut seven = Digits::new(decimal_base(), "7");
  seven.pow(seven.zero());
  assert_eq!(seven.to_s(), "1"); // 1.1.1 left it at "7"

  let mut seven = Digits::new(decimal_base(), "7");
  seven ^= seven.zero();
  assert_eq!(seven.to_s(), "1");
}

#[test]
fn converts_numbers_wider_than_u64() {
  let two_to_the_64 = format!("1{}", "0".repeat(64));
  let binary = Digits::new(binary_base(), two_to_the_64);
  // 1.1.1 overflowed: "0" in release builds, a panic in debug builds
  assert_eq!(binary.decimal().to_s(), "18446744073709551616");
  assert_eq!(binary.hex().to_s(), "10000000000000000");

  let big = Digits::new(decimal_base(), "123456789012345678901234567890");
  assert_eq!(big.hex().decimal(), big);
}

#[test]
fn converts_between_mappings_of_the_same_size() {
  let upper = Digits::new(hex_base(), "FF");
  let lower = Digits::new_zero(hexl_base());
  // 1.1.1 only compared base sizes and returned "FF" in the uppercase mapping
  assert_eq!(Digits::from((lower.clone(), upper.clone())).to_s(), "ff");
  assert_eq!(Digits::from((hexl_base(), upper.clone())).to_s(), "ff");
  assert_eq!(upper.hexl().to_s(), "ff");
}

#[test]
fn equality_and_ordering_agree() {
  let padded = Digits::new(decimal_base(), "009");
  let plain = padded.propagate("9");
  // 1.1.1: `==` was false while `partial_cmp` returned Equal
  assert_eq!(padded, plain);
  assert_eq!(padded.partial_cmp(&plain), Some(std::cmp::Ordering::Equal));
  assert!(padded <= plain);
  assert!(padded >= plain);

  let mut set = HashSet::new();
  set.insert(padded);
  assert!(set.contains(&plain));
}

#[test]
fn different_mappings_are_not_comparable() {
  let decimal = Digits::new(decimal_base(), "1");
  let letters = Digits::new(BaseCustom::<char>::new("ABCDEFGHIJ".chars().collect()), "B");
  // 1.1.1 panicked here
  assert_eq!(decimal.partial_cmp(&letters), None);
  assert!(decimal != letters);
}

#[test]
fn adjacency_limit_is_not_truncated() {
  let mut one = Digits::new(decimal_base(), "1");
  // 1.1.1 cast the limit to u8, so 256 behaved like 0 and skipped to "2"
  assert_eq!(one.next_non_adjacent(256).to_s(), "2");
  let mut eleven = Digits::new(decimal_base(), "10");
  assert_eq!(eleven.next_non_adjacent(256).to_s(), "11");
  assert_eq!(eleven.next_non_adjacent(usize::MAX).to_s(), "12");
}

#[test]
fn display_writes_the_number() {
  let forty_two = Digits::new(decimal_base(), "0042");
  assert_eq!(format!("{}", forty_two), "0042");
  assert_eq!(forty_two.to_string(), "0042");
  assert_eq!(format!("{:>6}", forty_two), "  0042");
  assert_eq!(
    format!("{:?}", forty_two),
    "Digits { value: \"0042\", base: 10, mapping: \"0123456789\" }"
  );
}

#[test]
fn accepts_str_slices() {
  let base10 = decimal_base();
  let nine = Digits::new(base10, "9");
  let forty_two = nine.propagate("42");
  assert_eq!(String::from(&forty_two), "42");
  let owned: String = forty_two.into();
  assert_eq!(owned, "42");
}

#[test]
#[should_panic(expected = "'x' is not a character of this numeric base")]
fn rejects_unknown_characters_clearly() {
  let _ = Digits::new(decimal_base(), "12x");
}

#[test]
fn handles_very_long_numbers_without_recursion() {
  // 1.1.1 overflowed the stack on numbers of a few thousand digits.  Run on a
  // small stack to show nothing recurses per digit.
  std::thread::Builder::new()
    .stack_size(64 * 1024)
    .spawn(|| {
      let mut nines = Digits::new(decimal_base(), "9".repeat(200_000));
      nines.succ();
      assert_eq!(nines.length(), 200_001);
      assert_eq!(nines.pinky(), '0');
      nines.pred_till_zero();
      assert_eq!(nines.length(), 200_001);
      assert_eq!(nines.max_adjacent(), 199_999);
      let copy = nines.clone();
      assert!(copy == nines);
      drop(copy);
    })
    .unwrap()
    .join()
    .unwrap();
}

#[test]
fn multiplies_large_numbers_quickly() {
  let a = Digits::new(decimal_base(), "7".repeat(2_000));
  let b = a.propagate("3".repeat(2_000));
  let product = a.mul(&b);
  // 777...7 * 333...3 = 2592...2591 pattern; check the shape and both ends
  assert_eq!(product.length(), 4_000);
  assert!(product.to_s().starts_with("259"));
  assert!(product.to_s().ends_with("41"));

  let mut two = Digits::new(decimal_base(), "2");
  two.pow(two.gen(1_000u64));
  assert_eq!(two.length(), 302);
  assert!(two.to_s().starts_with("10715086071862673"));
}

#[test]
fn operators_take_references() {
  let a = Digits::new(decimal_base(), "12");
  let b = a.propagate("3");
  assert_eq!((&a + &b).to_s(), "15");
  assert_eq!((&a * &b).to_s(), "36");
  assert_eq!((&a ^ &b).to_s(), "1728");
  assert_eq!((a.clone() + &b).to_s(), "15");
  assert_eq!((&a + b.clone()).to_s(), "15");
  let mut c = a.clone();
  c += &b;
  c *= &b;
  c ^= &b.one();
  assert_eq!(c.to_s(), "45");
  // operands are untouched
  assert_eq!(a.to_s(), "12");
  assert_eq!(b.to_s(), "3");
}

#[test]
fn mutating_methods_chain() {
  let mut n = Digits::new(decimal_base(), "0008");
  assert_eq!(n.succ().succ().to_s(), "0010");
  let one = n.one();
  assert_eq!(n.pred_till_zero().mut_add(&one).mut_add(one).to_s(), "0011");
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}
  assert_send_sync::<Digits>();

  let start = Digits::new(decimal_base(), "0000");
  let handles: Vec<_> = (0..4u64)
    .map(|offset| {
      let mut counter = start.clone();
      std::thread::spawn(move || {
        counter.mut_add(counter.gen(offset * 1000));
        for _ in 0..999 {
          counter.succ();
        }
        counter.to_s()
      })
    })
    .collect();
  let ends: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
  assert_eq!(ends, vec!["0999", "1999", "2999", "3999"]);
}
