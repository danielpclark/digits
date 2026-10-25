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
fn to_s_handles_a_long_number_on_a_small_stack() {
  let d = long_number(format!("1{}", "0".repeat(30_000)));
  on_small_stack(move || {
    let s = d.to_s();
    assert_eq!(s.len(), 30_001);
    assert!(s.starts_with("10"));
    assert_eq!(format!("{}", d).len() > 30_000, true);
    std::mem::forget(d); // dropping is still recursive on this branch
  });
}

#[test]
fn to_s_output_is_unchanged() {
  let base = BaseCustom::<char>::new("abcdefghij".chars().collect());
  assert_eq!(Digits::new(base10(), "0042".to_string()).to_s(), "0042");
  assert_eq!(Digits::new(base10(), "".to_string()).to_s(), "0");
  assert_eq!(Digits::new(base, "aaj".to_string()).to_s(), "aaj");
  let n = Digits::new(base10(), "90210".to_string());
  assert_eq!(n.to_string(), "90210");
  assert_eq!(format!("{}", n), "Digits — (Character: '0', Decimal Value: 0, With Preceeding: '9021')");
}
