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
fn dropping_a_long_number_does_not_overflow_a_small_stack() {
  let d = long_number(30_000);
  thread::Builder::new().stack_size(256 * 1024).spawn(move || {
    drop(d);
  }).unwrap().join().unwrap();
}

#[test]
fn methods_that_take_the_list_apart_still_work() {
  use digits::Reverse;
  let mut n = Digits::new(base10(), "0123".to_string());
  n.reverse();
  assert_eq!(n.to_s(), "3210");
  let mut m = Digits::new(base10(), "98".to_string());
  assert_eq!(m.step_non_adjacent(0).to_s(), "101");
  let mut k = Digits::new(base10(), "11".to_string());
  assert_eq!(k.mut_mul(Digits::new(base10(), "2".to_string())).to_s(), "22");
}
