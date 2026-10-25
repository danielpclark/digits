extern crate digits;
use digits::prelude::*;
use std::thread;

fn base10() -> BaseCustom<char> { BaseCustom::<char>::new("0123456789".chars().collect()) }

// Builds a long number on a generous stack (construction is still recursive on this
// branch) and hands it back to the caller.
fn long_number(digits: usize) -> Digits {
  thread::Builder::new().stack_size(1 << 30).spawn(move || {
    Digits::new(base10(), format!("1{}", "0".repeat(digits)))
  }).unwrap().join().unwrap()
}

#[test]
fn cloning_a_long_number_does_not_overflow_a_small_stack() {
  let d = long_number(20_000);
  thread::Builder::new().stack_size(256 * 1024).spawn(move || {
    let c = d.clone();
    assert_eq!(c.pinky(), d.pinky());
    assert!(c.is_valid_adjacent(20_000));
    // Dropping is still recursive on this branch; it is made iterative separately.
    std::mem::forget(c);
    std::mem::forget(d);
  }).unwrap().join().unwrap();
}

#[test]
fn clone_is_a_deep_independent_copy() {
  let original = Digits::new(base10(), "0042".to_string());
  let mut copy = original.clone();
  assert_eq!(copy, original);
  assert_eq!(copy.to_s(), "0042");
  assert_eq!(copy.length(), 4);
  copy.succ();
  assert_eq!(copy.to_s(), "0043");
  assert_eq!(original.to_s(), "0042");
  assert_eq!(Digits::new(base10(), "7".to_string()).clone().length(), 1);
}
