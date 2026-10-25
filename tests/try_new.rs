extern crate digits;
use digits::prelude::*;

fn base10() -> BaseCustom<char> { BaseCustom::<char>::new("0123456789".chars().collect()) }

#[test]
fn try_new_builds_valid_numbers() {
  assert_eq!(Digits::try_new(base10(), "0042".to_string()).unwrap().to_s(), "0042");
  assert_eq!(Digits::try_new(base10(), "".to_string()).unwrap().to_s(), "0");
  let greek = BaseCustom::<char>::new("αβγ".chars().collect());
  assert_eq!(Digits::try_new(greek, "γβα".to_string()).unwrap().as_mapping_vec(), vec![2, 1, 0]);
}

#[test]
fn try_new_rejects_unknown_characters() {
  assert_eq!(Digits::try_new(base10(), "12a".to_string()), Err("Character not in numeric base mapping!"));
  assert!(Digits::try_new(base10(), " 1".to_string()).is_err());
}

#[test]
fn try_new_matches_new_for_valid_input() {
  let a = Digits::try_new(base10(), "90210".to_string()).unwrap();
  let b = Digits::new(base10(), "90210".to_string());
  assert_eq!(a, b);
}

#[test]
#[should_panic(expected = "Character 'a' is not part of the numeric base \"0123456789\"")]
fn new_panics_with_a_descriptive_message() {
  Digits::new(base10(), "12a".to_string());
}

#[test]
#[should_panic(expected = "is not part of the numeric base")]
fn propagate_panics_with_a_descriptive_message() {
  let nine = Digits::new(base10(), "9".to_string());
  nine.propagate("x".to_string());
}
