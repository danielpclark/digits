extern crate digits;
use digits::Digits;
extern crate base_custom;
use base_custom::BaseCustom;

#[test]
fn it_can_return_string_result() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num = Digits::new(&base10, "12345".to_string());
  assert_eq!(num.to_s(), "12345");
  assert_eq!(num.to_string(), "12345");
  let s: String = num.into();
  assert_eq!(s, "12345");
}

#[test]
fn it_can_add() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num1 = Digits::new(&base10, "1".to_string());
  assert_eq!(num1.clone().add(num1).to_s(), "2");
}

#[test]
fn it_can_give_length() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num1 = Digits::new(&base10, "1111111".to_string());
  assert_eq!(num1.length(), 7);
}

#[test]
fn it_can_add_with_carry_over() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num13 = Digits::new(&base10, "13".to_string());
  let num29 = Digits::new(&base10, "29".to_string());
  assert_eq!(num13.clone().add(num29).to_s(), "42");
}

#[test]
fn it_can_add_with_multiple_carry_over() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let numa = Digits::new(&base10, "999".to_string());
  let numb = Digits::new(&base10, "999".to_string());
  assert_eq!(numa.clone().add(numb).to_s(), "1998");
}

#[test]
fn it_can_go_beyond_u64_max() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num_u64_max = Digits::new(&base10, "18446744073709551615".to_string());
  let one = Digits::new(&base10, "1".to_string());
  assert_eq!(num_u64_max.clone().add(one).to_s(), "18446744073709551616");
}

#[test]
fn it_can_provide_zero() {
  let base3 = BaseCustom::<char>::new("ABC".chars().collect());
  let num = Digits::new(&base3, "BA".to_string());
  assert_eq!(num.zero().pinky(), 'A');
  let zero = Digits::new_zero(&base3);
  assert_eq!(zero.pinky(), 'A');
}

#[test]
fn it_can_provide_one() {
  let base3 = BaseCustom::<char>::new("ABC".chars().collect());
  let num = Digits::new(&base3, "BA".to_string());
  assert_eq!(num.one().pinky(), 'B');
  let one = Digits::new_one(&base3);
  assert_eq!(one.pinky(), 'B');
}

#[test]
fn it_can_prove_zero() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let one = Digits::new(&base10, "1".to_string());
  assert_eq!(one.is_zero(), false);
  let zero = Digits::new(&base10, "0000".to_string());
  assert_eq!(zero.is_zero(), true);
  let thousand = Digits::new(&base10, "01000".to_string());
  assert_eq!(thousand.is_zero(), false);
}
