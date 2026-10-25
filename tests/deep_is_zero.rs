extern crate digits;
use digits::prelude::*;
use std::thread;

fn base10() -> BaseCustom<char> { BaseCustom::<char>::new("0123456789".chars().collect()) }

// Builds a long number on a generous stack (construction is still recursive on this
// branch) and hands it back to the caller.
fn long_number(value: String) -> Digits {
  thread::Builder::new().stack_size(1 << 30).spawn(move || Digits::new(base10(), value)).unwrap().join().unwrap()
}

// Runs `f` on a deliberately small stack so that any per-digit recursion overflows.
fn on_small_stack<F: FnOnce() + Send + 'static>(f: F) {
  thread::Builder::new().stack_size(256 * 1024).spawn(f).unwrap().join().unwrap();
}

#[test]
fn is_zero_handles_long_numbers_on_a_small_stack() {
  let zeros = long_number("0".repeat(30_000));
  let one = long_number(format!("1{}", "0".repeat(30_000)));
  on_small_stack(move || {
    assert!(zeros.is_zero());
    assert!(!one.is_zero());
    assert!(!zeros.is_one());
    std::mem::forget(zeros); std::mem::forget(one); // dropping is still recursive on this branch
  });
}

#[test]
fn is_zero_and_is_one_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  assert!(n("").is_zero());
  assert!(n("0").is_zero());
  assert!(n("000").is_zero());
  assert!(!n("010").is_zero());
  assert!(n("001").is_one());
  assert!(!n("011").is_one());
}
