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
fn length_handles_a_long_number_on_a_small_stack() {
  let d = long_number(format!("1{}", "0".repeat(30_000)));
  on_small_stack(move || {
    assert_eq!(d.length(), 30_001);
    std::mem::forget(d); // dropping is still recursive on this branch
  });
}

#[test]
fn length_is_unchanged() {
  assert_eq!(Digits::new(base10(), "".to_string()).length(), 1);
  assert_eq!(Digits::new(base10(), "7".to_string()).length(), 1);
  assert_eq!(Digits::new(base10(), "0042".to_string()).length(), 4);
}
