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
fn succ_on_a_long_number_completes_in_linear_memory() {
  // Before: every recursion level cloned the rest of the list, so this needed tens of GB.
  on_big_stack(|| {
    let mut n = Digits::new(base10(), "9".repeat(30_000));
    n.succ();
    assert_eq!(n.to_s(), format!("1{}", "0".repeat(30_000)));
    let mut m = Digits::new(base10(), format!("5{}", "0".repeat(30_000)));
    m.mut_add(Digits::new(base10(), "12345".to_string()));
    assert_eq!(m.to_s(), format!("5{}12345", "0".repeat(29_995)));
  });
}

#[test]
fn addition_results_and_zero_padding_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  let mut a = n("0005"); a.mut_add(n("5"));      assert_eq!(a.to_s(), "0010");
  let mut b = n("5");    b.mut_add(n("0005"));   assert_eq!(b.to_s(), "10");
  let mut c = n("099");  c.mut_add(n("1"));      assert_eq!(c.to_s(), "100");
  let mut d = n("0099"); d.mut_add(n("901"));    assert_eq!(d.to_s(), "1000");
  let mut e = n("05");   e.mut_add(n("00"));     assert_eq!(e.to_s(), "05");
  let mut f = n("0");    f.mut_add(n("0"));      assert_eq!(f.to_s(), "0");
  let mut g = n("0003"); g.succ();               assert_eq!(g.to_s(), "0004");
  assert_eq!((n("0005") + n("5")).to_s(), "0010");
  // the trimming variant is reached through multiplication
  assert_eq!(n("05").mul(n("1")).to_s(), "5");
  assert_eq!(n("0").mul(n("0")).to_s(), "0");
  assert_eq!(n("1000").mul(n("1000")).to_s(), "1000000");
  let hex = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  let mut h = Digits::new(hex.clone(), "0fff".to_string());
  h.mut_add(Digits::new(hex, "1".to_string()));
  assert_eq!(h.to_s(), "1000");
}
