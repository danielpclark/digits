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
fn as_mapping_vec_handles_a_long_number_on_a_small_stack() {
  let d = long_number(format!("1{}", "0".repeat(30_000)));
  on_small_stack(move || {
    let v = d.as_mapping_vec();
    assert_eq!(v.len(), 30_001);
    assert_eq!(v[0], 1);
    assert_eq!(v[30_000], 0);
    std::mem::forget(d); // dropping is still recursive on this branch
  });
}

#[test]
fn as_mapping_vec_is_unchanged() {
  let base = BaseCustom::<char>::new("abcdefghij".chars().collect());
  assert_eq!(Digits::new(base10(), "0042".to_string()).as_mapping_vec(), vec![0, 0, 4, 2]);
  assert_eq!(Digits::new(base10(), "".to_string()).as_mapping_vec(), vec![0]);
  assert_eq!(Digits::new(base, "jab".to_string()).as_mapping_vec(), vec![9, 0, 1]);
}
