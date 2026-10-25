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
fn equality_handles_long_numbers_on_a_small_stack() {
  let a = long_number(format!("1{}", "0".repeat(30_000)));
  let b = long_number(format!("1{}", "0".repeat(30_000)));
  let c = long_number(format!("1{}1", "0".repeat(29_999)));
  on_small_stack(move || {
    assert!(a == b);
    assert!(a != c);
    std::mem::forget(a); std::mem::forget(b); std::mem::forget(c); // dropping is still recursive on this branch
  });
}

#[test]
fn equality_is_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  assert!(n("42") == n("42"));
  assert!(n("042") != n("42"));   // zero padding is part of the value's structure
  assert!(n("42") != n("43"));
  assert!(n("4") != n("42"));
  assert!(n("") == n("0"));
  let other = Digits::new(BaseCustom::<char>::new("0123456789abcdef".chars().collect()), "42".to_string());
  assert!(n("42") != other);       // different mapping
}
