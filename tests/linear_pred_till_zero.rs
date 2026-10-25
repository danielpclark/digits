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
fn pred_till_zero_on_a_long_number_completes_in_linear_memory() {
  // Before: every borrow cloned the rest of the list, so this needed tens of GB.
  on_big_stack(|| {
    let mut n = Digits::new(base10(), format!("1{}", "0".repeat(30_000)));
    n.pred_till_zero();
    assert_eq!(n.to_s(), format!("0{}", "9".repeat(30_000)));
  });
}

#[test]
fn pred_till_zero_results_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  let mut a = n("10");   assert_eq!(a.pred_till_zero().to_s(), "09");
  let mut b = n("0100"); assert_eq!(b.pred_till_zero().to_s(), "0099");
  let mut c = n("1");    assert_eq!(c.pred_till_zero().to_s(), "0");
  let mut d = n("000");  assert_eq!(d.pred_till_zero().to_s(), "000");
  let mut e = n("42");   assert_eq!(e.pred_till_zero().to_s(), "41");
  let hex = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  let mut h = Digits::new(hex, "100".to_string());
  assert_eq!(h.pred_till_zero().to_s(), "0ff");
}
