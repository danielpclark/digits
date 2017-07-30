extern crate digits;
use digits::{BaseCustom,Digits};

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
  let num1 = Digits::new_one(&base10);
  assert_eq!(num1.clone().add(num1.clone()).to_s(), "2");
  assert_eq!(num1.to_s(), "1");
}

#[test]
fn it_can_mut_add() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num1 = Digits::new_one(&base10);
  let num2 = num1.propagate("2".to_string());
  num1.mut_add(num2);
  assert_eq!(num1.to_s(), "3");
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
  let one = Digits::new_one(&base10);
  assert_eq!(one.is_zero(), false);
  let zero = Digits::new(&base10, "0000".to_string());
  assert_eq!(zero.is_zero(), true);
  let thousand = Digits::new(&base10, "01000".to_string());
  assert_eq!(thousand.is_zero(), false);
}

#[test]
fn eleven_times_one_with_mul_method() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let one = Digits::new_one(&base10);
  let eleven = one.propagate("11".to_string());
  assert_eq!(eleven.mul(one).to_s(), "11");
}

#[test]
fn one_times_eleven_with_mul_method() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let one = Digits::new_one(&base10);
  let eleven = one.propagate("11".to_string());
  assert_eq!(one.mul(eleven).to_s(), "11");
  assert_eq!(one.to_s(), "1");
}

#[test]
fn cdlxxxii_times_xxxviii_with_mul_method() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let cdlxxxii = Digits::new(&base10, "482".to_string());
  let xxxviii = cdlxxxii.propagate("38".to_string());
  assert_eq!(cdlxxxii.mul(xxxviii).to_s(), "18316");
  assert_eq!(cdlxxxii.to_s(), "482");
}

#[test]
fn cdlxxxii_times_xxxviii_with_mut_mul_method() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut cdlxxxii = Digits::new(&base10, "482".to_string());
  let xxxviii = cdlxxxii.propagate("38".to_string());
  cdlxxxii.mut_mul(xxxviii);
  assert_eq!(cdlxxxii.to_s(), "18316");
}

#[test]
fn it_multiplies_powers_of_with_pow() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut ten = Digits::new(&base10, "10".to_string());
  let two = ten.propagate("2".to_string());
  assert_eq!(ten.pow(two).to_s(), "100");
  let two = ten.propagate("2".to_string());
  assert_eq!(ten.pow(two).to_s(), "10000");
}

#[test]
fn it_succs() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut nine = Digits::new(&base10, "9".to_string());
  assert_eq!(nine.succ().to_s(), "10");
  assert_eq!(nine.succ().to_s(), "11");
  assert_eq!(nine.succ().to_s(), "12");
}

#[test]
fn it_preds_till_zero() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut ten = Digits::new(&base10, "10".to_string());
  assert_eq!(ten.pred_till_zero().to_s(), "9");
  assert_eq!(ten.pred_till_zero().to_s(), "8");
  assert_eq!(ten.pred_till_zero().to_s(), "7");
  assert_eq!(ten.pred_till_zero().to_s(), "6");
  assert_eq!(ten.pred_till_zero().to_s(), "5");
  assert_eq!(ten.pred_till_zero().to_s(), "4");
  assert_eq!(ten.pred_till_zero().to_s(), "3");
  assert_eq!(ten.pred_till_zero().to_s(), "2");
  assert_eq!(ten.pred_till_zero().to_s(), "1");
  assert_eq!(ten.pred_till_zero().to_s(), "0");
  assert_eq!(ten.pred_till_zero().to_s(), "0");
}

#[test]
fn one_mul_two_to_the_power_of_three() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut two = Digits::new(&base10, "2".to_string());
  let one = Digits::new_one(&base10);
  let three = Digits::new(&base10, "3".to_string());
  assert_eq!(one.mul(two.pow(three)).to_s(), "8");
}

#[test]
fn two_mut_add_eight() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let two = Digits::new(&base10, "2".to_string());
  let eight = Digits::new(&base10, "8".to_string());
  let mut result = Digits::new_zero(&base10);
  result.mut_add(two);
  result.mut_add(eight);
  assert_eq!(result.to_s(), "10");
}

#[test]
fn it_multiplies_by_zero() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let one = Digits::new_one(&base10);
  let zero = Digits::new_zero(&base10);
  assert_eq!(zero.mul(one.clone()).to_s(), "0");
  assert_eq!(one.mul(zero).to_s(), "0");
}

#[test]
fn it_adds_zero() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let one = Digits::new_one(&base10);
  let zero = Digits::new_zero(&base10);
  assert_eq!(zero.clone().add(zero.clone()).to_s(), "0");
  assert_eq!(one.add(zero).to_s(), "1");
}

#[test]
fn get_digits_from_u64() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let ten = Digits::from((&base10, 10_u64));
  assert_eq!(ten.to_s(), "10");
}

#[test]
fn get_digits_from_digits_with_base() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let binary = BaseCustom::<char>::new("01".chars().collect());
  
  let bin_ten = Digits::new(&binary, "1010".to_string());

  let ten = Digits::from((&base10, bin_ten));
  assert_eq!(ten.to_s(), "10");
}

#[test]
fn get_digits_from_digits_with_digits() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let binary = BaseCustom::<char>::new("01".chars().collect());

  let digits_to_get_base_from = Digits::new_zero(&base10);
  
  let bin_ten = Digits::new(&binary, "1010".to_string());

  let ten = Digits::from((digits_to_get_base_from, bin_ten));
  assert_eq!(ten.to_s(), "10");
}

#[test]
fn get_digits_from_digits_with_same_digits_base() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let eleven = Digits::new(&base10, "11".to_string());
  let one = Digits::new_one(&base10);
  
  let result = Digits::from((one, eleven.clone()));
  assert_eq!(result, eleven);
}

#[test]
fn it_gets_power_of_zero_and_one() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut ten = Digits::new(&base10, "10".to_string());
  let one = ten.one();
  assert_eq!(ten.pow(one).to_s(), "10");
  let zero = ten.zero();
  assert_eq!(ten.pow(zero).to_s(), "1");
}
