extern crate digits;
use digits::prelude::*;
use std::thread;

fn base10() -> BaseCustom<char> { BaseCustom::<char>::new("0123456789".chars().collect()) }

// Runs `f` on a deliberately small stack so that any per-digit recursion overflows.
fn on_small_stack<F: FnOnce() + Send + 'static>(f: F) {
  thread::Builder::new().stack_size(256 * 1024).spawn(f).unwrap().join().unwrap();
}

#[test]
fn new_handles_very_long_numbers_on_a_small_stack() {
  on_small_stack(|| {
    let value = format!("1{}", "0".repeat(200_000));
    let d = Digits::new(base10(), value);
    // Only non-recursive accessors are used here; the other walkers are made
    // iterative in their own changes.
    assert_eq!(d.pinky(), '0');
    assert!(d.is_valid_adjacent(200_000));
    assert!(!d.is_valid_adjacent(10));
    std::mem::forget(d); // dropping is still recursive until Drop is made iterative
  });
}

#[test]
fn new_builds_the_same_structure_as_before() {
  let d = Digits::new(base10(), "0042".to_string());
  assert_eq!(d.to_s(), "0042");
  assert_eq!(d.length(), 4);
  assert_eq!(d.as_mapping_vec(), vec![0, 0, 4, 2]);
  assert_eq!(Digits::new(base10(), "".to_string()).to_s(), "0");
  assert_eq!(Digits::new(base10(), "".to_string()).length(), 1);
  assert_eq!(Digits::new(base10(), "7".to_string()).length(), 1);
}
