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
use digits::Reverse;

#[test]
fn reverse_handles_a_long_number_on_a_small_stack() {
  let mut d = on_big_stack_value(|| Digits::new(base10(), format!("1{}", "0".repeat(30_000))));
  on_small_stack(move || {
    d.reverse();
    assert_eq!(d.pinky(), '1');
    assert!(d.is_valid_adjacent(30_000));
    std::mem::forget(d); // dropping is still recursive on this branch
  });
}

fn on_big_stack_value<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(f: F) -> T {
  thread::Builder::new().stack_size(1 << 30).spawn(f).unwrap().join().unwrap()
}

#[test]
fn reverse_results_are_unchanged() {
  let n = |v: &str| Digits::new(base10(), v.to_string());
  let mut a = n("0009"); a.reverse(); assert_eq!(a.to_s(), "9000");
  let mut b = n("123");  b.reverse(); assert_eq!(b.to_s(), "321"); assert_eq!(b.length(), 3);
  let mut c = n("7");    c.reverse(); assert_eq!(c.to_s(), "7");
  let mut d = n("");     d.reverse(); assert_eq!(d.to_s(), "0");
  let mut e = n("1200"); e.reverse(); e.reverse(); assert_eq!(e, n("1200"));
}
