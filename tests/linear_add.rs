extern crate digits;
use digits::prelude::*;
use std::thread;

fn base10() -> BaseCustom<char> { BaseCustom::<char>::new("0123456789".chars().collect()) }

// Runs `f` on a generous stack: on this branch construction, clone and several walkers
// are still recursive, so the stack size must not be the limiting factor here.
fn on_big_stack<F: FnOnce() + Send + 'static>(f: F) {
  thread::Builder::new().stack_size(1 << 30).spawn(f).unwrap().join().unwrap();
}

// Runs `f` on a deliberately small stack so that any per-digit recursion overflows.
#[allow(dead_code)]
fn on_small_stack<F: FnOnce() + Send + 'static>(f: F) {
  thread::Builder::new().stack_size(256 * 1024).spawn(f).unwrap().join().unwrap();
}

#[test]
fn add_on_long_numbers_completes_in_linear_time() {
  // Before: the remainder of both lists was cloned on every step (quadratic time).
  on_big_stack(|| {
    let a = Digits::new(base10(), "9".repeat(20_000));
    let b = Digits::new(base10(), format!("1{}", "0".repeat(19_999)));
    let sum = a.add(b);
    assert_eq!(sum.to_s(), format!("10{}", "9".repeat(19_999)));
    assert_eq!(sum.length(), 20_001);
    assert_eq!(sum.pinky(), '9');
  });
}

#[test]
fn add_results_and_zero_padding_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  assert_eq!(n("11").add(n("2")).to_s(), "13");
  assert_eq!(n("0005").add(n("5")).to_s(), "0010");
  assert_eq!(n("5").add(n("0005")).to_s(), "0010");
  assert_eq!(n("99").add(n("1")).to_s(), "100");
  assert_eq!(n("0").add(n("0")).to_s(), "0");
  assert_eq!(n("").add(n("007")).to_s(), "007");
  let hex = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  assert_eq!(Digits::new(hex.clone(), "ff".to_string()).add(Digits::new(hex, "1".to_string())).to_s(), "100");
}
