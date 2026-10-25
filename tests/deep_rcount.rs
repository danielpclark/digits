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
fn rcount_handles_a_long_number_on_a_small_stack() {
  let d = long_number(format!("1{}", "0".repeat(30_000)));
  on_small_stack(move || {
    assert_eq!(d.rcount(0), 30_000);
    assert_eq!(d.rcount(1), 0);
    std::mem::forget(d); // dropping is still recursive on this branch
  });
}

#[test]
fn rcount_is_unchanged() {
  let base = BaseCustom::<char>::new("ABC3456789".chars().collect());
  let num = Digits::new(base.clone(), "34BBB".to_string());
  assert_eq!(num.rcount(1), 3);
  assert_eq!(num.rcount(0), 0);
  assert_eq!(Digits::new(base.clone(), "B".to_string()).rcount(1), 1);
  assert_eq!(Digits::new(base.clone(), "BBBB".to_string()).rcount(1), 4);
  assert_eq!(Digits::new(base, "AB".to_string()).rcount(1), 1);
}
