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
fn zero_fill_on_a_long_number_completes_in_linear_memory() {
  // Before: every level cloned the rest of the list, so this needed tens of GB.
  on_big_stack(|| {
    let mut n = Digits::new(base10(), "9".repeat(30_000));
    n.zero_fill(30_005);
    assert_eq!(n.to_s(), format!("00000{}", "9".repeat(30_000)));
    let mut m = Digits::new(base10(), "7".to_string());
    m.zero_fill(100_000);
    assert_eq!(m.length(), 100_000);
  });
}

#[test]
fn zero_fill_results_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  let mut a = n("9");    a.zero_fill(4); assert_eq!(a.to_s(), "0009");
  let mut b = n("0009"); b.zero_fill(2); assert_eq!(b.to_s(), "0009");
  let mut c = n("123");  c.zero_fill(3); assert_eq!(c.to_s(), "123");
  let mut d = n("123");  d.zero_fill(0); assert_eq!(d.to_s(), "123");
  let mut e = n("");     e.zero_fill(3); assert_eq!(e.to_s(), "000");
  let greek = BaseCustom::<char>::new("αβγ".chars().collect());
  let mut g = Digits::new(greek, "γ".to_string());
  g.zero_fill(3);
  assert_eq!(g.to_s(), "ααγ");
}
