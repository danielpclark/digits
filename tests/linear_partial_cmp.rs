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
use std::cmp::Ordering;

fn on_big_stack_value<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(f: F) -> T {
  thread::Builder::new().stack_size(1 << 30).spawn(f).unwrap().join().unwrap()
}

#[test]
fn comparison_of_long_numbers_runs_on_a_small_stack() {
  let a = on_big_stack_value(|| Digits::new(base10(), format!("1{}", "0".repeat(30_000))));
  let b = on_big_stack_value(|| Digits::new(base10(), format!("1{}1", "0".repeat(29_999))));
  on_small_stack(move || {
    assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
    assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
    assert!(a < b);
    std::mem::forget(a); std::mem::forget(b); // dropping is still recursive on this branch
  });
}

#[test]
fn comparison_results_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  assert!(n("10") > n("9"));
  assert!(n("9") < n("10"));
  assert!(n("0010") > n("9"));
  assert!(n("9") < n("0010"));
  assert_eq!(n("42").partial_cmp(&n("042")), Some(Ordering::Equal));
  assert_eq!(n("042").partial_cmp(&n("42")), Some(Ordering::Equal));
  assert_eq!(n("123").partial_cmp(&n("123")), Some(Ordering::Equal));
  assert!(n("200") > n("199"));
}
