extern crate digits;
use digits::{BaseCustom,Digits};

#[test]
fn is_will_reverse() {
  use digits::Reverse;
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "0008".to_string());
  num.reverse();
  assert_eq!(num.to_s(), "8000");
  let mut num = Digits::new(&base10, "998".to_string());
  num.reverse();
  assert_eq!(num.to_s(), "899");
  let mut num = Digits::new(&base10, "5000".to_string());
  num.reverse();
  assert_eq!(num.to_s(), "0005");
}

#[test]
fn is_adjacent_limit() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num = Digits::new(&base10, "0008".to_string());
  assert_eq!(num.is_valid_adjacent(0), false);
  assert_eq!(num.is_valid_adjacent(1), false);
  assert_eq!(num.is_valid_adjacent(2), true);
  let num = Digits::new(&base10, "998".to_string());
  assert_eq!(num.is_valid_adjacent(0), false);
  assert_eq!(num.is_valid_adjacent(1), true);
  assert_eq!(num.is_valid_adjacent(2), true);
}

#[test]
fn it_preps_one_adjacent_character_in_prep_non_adjacent() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "0008".to_string());
  assert_eq!(num.prep_non_adjacent(1).to_s(), "0009".to_string());
  let mut num = Digits::new(&base10, "0000".to_string());
  assert_eq!(num.prep_non_adjacent(1).to_s(), "0009".to_string());
  let mut num = Digits::new(&base10, "9999".to_string());
  assert_eq!(num.prep_non_adjacent(1).to_s(), "10009".to_string());
}

#[test]
fn it_allows_one_adjacent_character_in_step_non_adjacent() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "009".to_string());
  assert_eq!(num.step_non_adjacent(1).to_s(), "010".to_string());
}

#[test]
fn as_mapping_result() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num = Digits::new(&base10, "0123456789".to_string());
  assert_eq!(num.as_mapping_vec(), vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn zero_fill_edge_case_for_minimal_adjacent_stepping() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "0000".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "0010".to_string());
  let mut num = Digits::new(&base10, "2222".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "2230".to_string());
  let mut num = Digits::new(&base10, "9999".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "10010".to_string());
  let mut num = Digits::new(&base10, "55555".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "55600".to_string());
}

#[test]
fn it_shows_the_base_size() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let num = Digits::new(&base10, "".to_string());
  assert_eq!(num.base(), 10);
}

#[should_panic]
#[test]
fn should_panic_when_base_too_low_for_non_adjacent_stepping() {
  let base2 = BaseCustom::<char>::new("01".chars().collect());
  let mut num = Digits::new(&base2, "101010".to_string());
  assert_eq!(num.next_non_adjacent(0).to_s(), "1010101".to_string());
}

#[test]
fn it_avoids_adjacent_characters_in_step() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "198".to_string());
  assert_eq!(num.next_non_adjacent(0).to_s(), "201".to_string());
  let mut num = Digits::new(&base10, "1098".to_string());
  assert_eq!(num.next_non_adjacent(0).to_s(), "1201".to_string());
}

#[test]
fn it_allows_one_adjacent_character_in_step() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "998".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "1001".to_string());
  let mut num = Digits::new(&base10, "1009".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "1010".to_string());
  let mut num = Digits::new(&base10, "99899".to_string());
  assert_eq!(num.next_non_adjacent(1).to_s(), "100100".to_string());
}

#[test]
fn it_allows_two_adjacent_characters_in_step() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "9998".to_string());
  assert_eq!(num.next_non_adjacent(2).to_s(), "10001".to_string());
  let mut num = Digits::new(&base10, "9998999".to_string());
  assert_eq!(num.next_non_adjacent(2).to_s(), "10001000".to_string());
}

#[test]
fn it_counts_maximum_adjacent_characters() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let builder = Digits::new(&base10, "".to_string());
  let num = builder.new_mapped(vec![1,0,5,5,5,5,5,5,5,2,1,1,1,1]).ok().unwrap();
  assert_eq!(num.max_adjacent(), 6); // 7 - 1
}

#[test]
fn it_right_counts_character_base_index_matches() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let builder = Digits::new(&base10, "".to_string());
  let num = builder.new_mapped(vec![1,0,2,1,1,1,1]).ok().unwrap();
  assert_eq!(num.rcount(1), 4);
}

#[test]
fn it_mapps_to_correct_from_zero_numeric_chars() {
  let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  let builder = Digits::new(&base16, "".to_string());
  let num = builder.new_mapped(vec![1,0,2,1]).ok().unwrap();
  assert_eq!(num.to_s(), "1021");
}

#[test]
fn it_errs_correctly_for_max_map_range() {
  let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  let builder = Digits::new(&base16, "".to_string());
  let num = builder.new_mapped(vec![15]).ok().unwrap();
  assert_eq!(num.to_s(), "f");
  let num = builder.new_mapped(vec![16]);
  assert_eq!(num, Err("Character mapping out of range!"));
}

#[test]
fn it_zero_fills() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "".to_string());
  num.zero_fill(4);
  assert_eq!(num.to_s(), "0000".to_string());

  let mut num = Digits::new(&base10, "1".to_string());
  num.zero_fill(4);
  assert_eq!(num.to_s(), "0001".to_string());

  let mut num = Digits::new(&base10, "012".to_string());
  num.zero_fill(4);
  assert_eq!(num.to_s(), "0012".to_string());

  let mut num = Digits::new(&base10, "12345".to_string());
  num.zero_fill(4);
  assert_eq!(num.to_s(), "12345".to_string());
}

#[test]
fn it_trims_zeros() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "0000".to_string());
  num.zero_trim();
  assert_eq!(num.to_s(), "0".to_string());

  let mut num = Digits::new(&base10, "0001".to_string());
  num.zero_trim();
  assert_eq!(num.to_s(), "1".to_string());

  let mut num = Digits::new(&base10, "0012".to_string());
  num.zero_trim();
  assert_eq!(num.to_s(), "12".to_string());

  let mut num = Digits::new(&base10, "12345".to_string());
  num.zero_trim();
  assert_eq!(num.to_s(), "12345".to_string());
}

#[test]
fn normal_addition_preserves_zero_padding(){
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num1 = Digits::new(&base10, "0001".to_string());
  let num2 = Digits::new(&base10, "0011".to_string());
  num1 += num2;
  assert_eq!(num1.to_s(), "0012");
}

#[test]
fn preserve_zero_padding_increment(){
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "0001".to_string());
  assert_eq!(num.succ().to_s(), "0002");
}

#[test]
fn preserve_zero_padding_decrement(){
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num = Digits::new(&base10, "1000".to_string());
  assert_eq!(num.pred_till_zero().to_s(), "0999");
}

#[test]
fn it_no_longer_panics_when_add_performed_with_different_bases() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let base2 = BaseCustom::<char>::new("01".chars().collect());
  let num1 = Digits::new(&base10, "1".to_string());
  let num2 = Digits::new(&base2, "1".to_string());
  num1.clone().add(num2.clone());
}

#[test]
fn it_no_longer_panics_when_mut_add_performed_with_different_bases() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let base2 = BaseCustom::<char>::new("01".chars().collect());
  let num1 = Digits::new(&base10, "1".to_string());
  let num2 = Digits::new(&base2, "1".to_string());
  num1.clone().mut_add(num2.clone());
}

#[test]
fn it_no_longer_panics_when_mul_performed_with_different_bases() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let base2 = BaseCustom::<char>::new("01".chars().collect());
  let num1 = Digits::new(&base10, "9".to_string());
  let num2 = Digits::new(&base2, "00001001".to_string());
  assert_eq!(num1.clone().mul(num2.clone()).to_s(), "81");
}

#[test]
fn it_no_longer_panics_when_mut_mul_performed_with_different_bases() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let base2 = BaseCustom::<char>::new("01".chars().collect());
  let num1 = Digits::new(&base10, "9".to_string());
  let num2 = Digits::new(&base2, "00001001".to_string());
  assert_eq!(num1.clone().mut_mul(num2.clone()).to_s(), "81");
}

#[test]
fn it_wont_panic_when_pow_performed_with_different_bases() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let base2 = BaseCustom::<char>::new("01".chars().collect());
  let num1 = Digits::new(&base10, "1".to_string());
  let num2 = Digits::new(&base2, "1".to_string());
  assert_eq!(num1.clone().pow(num2.clone()).to_s(), "1");
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
  let mut nines = Digits::new(&base10, "9999".to_string());
  assert_eq!(nines.succ().to_s(), "10000");
}

#[test]
fn it_preds_till_zero() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut ten = Digits::new(&base10, "10".to_string());
  assert_eq!(ten.pred_till_zero().to_s(), "09");
  assert_eq!(ten.pred_till_zero().to_s(), "08");
  assert_eq!(ten.pred_till_zero().to_s(), "07");
  assert_eq!(ten.pred_till_zero().to_s(), "06");
  assert_eq!(ten.pred_till_zero().to_s(), "05");
  assert_eq!(ten.pred_till_zero().to_s(), "04");
  assert_eq!(ten.pred_till_zero().to_s(), "03");
  assert_eq!(ten.pred_till_zero().to_s(), "02");
  assert_eq!(ten.pred_till_zero().to_s(), "01");
  assert_eq!(ten.pred_till_zero().to_s(), "00");
  assert_eq!(ten.pred_till_zero().to_s(), "00");
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

#[test]
fn it_can_add_with_plus_symbol(){
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let a = Digits::new(&base10, "1".to_string());
  let b = Digits::new(&base10, "1".to_string());
  assert_eq!((a + b).to_s(), "2");
}

#[test]
fn it_can_add_assign() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut num1 = Digits::new_one(&base10);
  let num2 = num1.propagate("2".to_string());
  num1 += num2;
  assert_eq!(num1.to_s(), "3");
}

#[test]
fn cdlxxxii_times_xxxviii_with_times_symbol() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let cdlxxxii = Digits::new(&base10, "482".to_string());
  let xxxviii = cdlxxxii.propagate("38".to_string());
  assert_eq!((cdlxxxii * xxxviii).to_s(), "18316");
}

#[test]
fn cdlxxxii_times_xxxviii_with_times_equals_symbol() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut cdlxxxii = Digits::new(&base10, "482".to_string());
  let xxxviii = cdlxxxii.propagate("38".to_string());
  cdlxxxii *= xxxviii;
  assert_eq!(cdlxxxii.to_s(), "18316");
}

#[test]
fn it_multiplies_powers_of_with_carot() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let ten = Digits::new(&base10, "10".to_string());
  let two = ten.propagate("2".to_string());
  assert_eq!((ten ^ two).to_s(), "100");
}

#[test]
fn it_multiplies_powers_of_with_carot_assign() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let mut ten = Digits::new(&base10, "10".to_string());
  let two = ten.propagate("2".to_string());
  ten ^= two;
  assert_eq!(ten.to_s(), "100");
  let two = ten.propagate("2".to_string());
  ten ^= two;
  assert_eq!(ten.to_s(), "10000");
}

#[test]
fn it_can_tell_which_digits_is_larger() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let ten = Digits::new(&base10, "10".to_string());
  let one = ten.one();
  assert_eq!(one < ten, true);
  assert_eq!(one > ten, false);
  assert_eq!(one <= ten, true);
  assert_eq!(one >= ten, false);
}

#[test]
fn it_can_tell_which_digits_is_larger_equal_length() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let big   = Digits::new(&base10,"2".to_string());
  let small = big.propagate("1".to_string());
  assert_eq!(small <  big, true);
  assert_eq!(small >  big, false);
  assert_eq!(small <= big, true);
  assert_eq!(small >= big, false);
}

#[test]
fn it_can_tell_which_digits_is_larger_equal_length_big() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  let big   = Digits::new(&base10,"12349786".to_string());
  let small = big.propagate("11913785".to_string());
  assert_eq!(small <  big, true);
  assert_eq!(small >  big, false);
  assert_eq!(small <= big, true);
  assert_eq!(small >= big, false);
}
