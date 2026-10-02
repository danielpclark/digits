// Copyright 2017 Daniel P. Clark & other digits Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Arithmetic on little-endian digit vectors in an arbitrary base.
//!
//! Every slice here stores the least significant digit first and every digit
//! is strictly less than `base`.  `BaseCustom` limits a base to 255 symbols,
//! so a digit always fits in a `u8` and intermediate values fit comfortably
//! in `u32` / `u64`.  Nothing in this module recurses, so the size of a
//! number is limited only by memory.

use std::cmp::Ordering;

/// Number of digits once most significant zeros are ignored (never less than 1).
pub(crate) fn significant_len(d: &[u8]) -> usize {
  d.iter().rposition(|&x| x != 0).map_or(1, |i| i + 1)
}

/// Remove most significant zeros, keeping at least one digit.
pub(crate) fn trim(d: &mut Vec<u8>) {
  let len = significant_len(d);
  d.truncate(len);
}

pub(crate) fn is_zero(d: &[u8]) -> bool {
  d.iter().all(|&x| x == 0)
}

/// Compare numeric values, ignoring zero padding.
pub(crate) fn cmp(a: &[u8], b: &[u8]) -> Ordering {
  let a = &a[..significant_len(a)];
  let b = &b[..significant_len(b)];
  a.len()
    .cmp(&b.len())
    .then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

/// `a += b`.  The width of `a` grows to the wider operand, plus one digit on a final carry.
pub(crate) fn add_assign(a: &mut Vec<u8>, b: &[u8], base: u32) {
  if a.len() < b.len() {
    a.resize(b.len(), 0);
  }
  let mut carry = 0;
  for (i, digit) in a.iter_mut().enumerate() {
    let other = match b.get(i) {
      Some(&x) => u32::from(x),
      None if carry == 0 => return,
      None => 0,
    };
    let sum = u32::from(*digit) + other + carry;
    if sum >= base {
      *digit = (sum - base) as u8;
      carry = 1;
    } else {
      *digit = sum as u8;
      carry = 0;
    }
  }
  if carry > 0 {
    a.push(1);
  }
}

/// `a += 1`, keeping the width of `a` unless the carry runs off the end.
pub(crate) fn increment(a: &mut Vec<u8>, base: u32) {
  increment_from(a, 0, base);
}

/// Add one at position `start`, carrying upward.
fn increment_from(a: &mut Vec<u8>, start: usize, base: u32) {
  for digit in a.iter_mut().skip(start) {
    if u32::from(*digit) + 1 < base {
      *digit += 1;
      return;
    }
    *digit = 0;
  }
  a.push(1);
}

/// `a -= 1`, keeping the width of `a`.  `a` must not be zero.
pub(crate) fn decrement(a: &mut [u8], base: u32) {
  for digit in a.iter_mut() {
    if *digit > 0 {
      *digit -= 1;
      return;
    }
    *digit = (base - 1) as u8;
  }
  debug_assert!(false, "decrement called on zero");
}

/// Schoolbook multiplication.  The result carries no zero padding.
pub(crate) fn mul(a: &[u8], b: &[u8], base: u32) -> Vec<u8> {
  let a = &a[..significant_len(a)];
  let b = &b[..significant_len(b)];
  if is_zero(a) || is_zero(b) {
    return vec![0];
  }

  // Each column sums at most min(len) products of at most 254 * 254, so a
  // u64 column cannot overflow before the operands reach ~2.8e14 digits.
  let mut columns = vec![0u64; a.len() + b.len()];
  for (i, &x) in a.iter().enumerate() {
    if x == 0 {
      continue;
    }
    for (column, &y) in columns[i..].iter_mut().zip(b) {
      *column += u64::from(x) * u64::from(y);
    }
  }

  let base = u64::from(base);
  let mut carry = 0;
  let mut out: Vec<u8> = columns
    .into_iter()
    .map(|column| {
      let total = column + carry;
      carry = total / base;
      (total % base) as u8
    })
    .collect();
  while carry > 0 {
    out.push((carry % base) as u8);
    carry /= base;
  }
  trim(&mut out);
  out
}

/// `a = a * factor + addend` for `factor` and `addend` up to `CHUNK_LIMIT`.
fn mul_small_add(a: &mut Vec<u8>, factor: u64, addend: u64, base: u32) {
  // carry stays below 255 * factor, so `total` stays far below u64::MAX.
  let base = u64::from(base);
  let mut carry = addend;
  for digit in a.iter_mut() {
    let total = u64::from(*digit) * factor + carry;
    *digit = (total % base) as u8;
    carry = total / base;
  }
  while carry > 0 {
    a.push((carry % base) as u8);
    carry /= base;
  }
}

/// Largest multiplier `convert` feeds to `mul_small_add` in one pass.
const CHUNK_LIMIT: u64 = 1 << 32;

/// `a /= divisor` for a small divisor, returning the remainder.
pub(crate) fn div_small(a: &mut [u8], divisor: u32, base: u32) -> u32 {
  let mut remainder = 0;
  for digit in a.iter_mut().rev() {
    let current = remainder * base + u32::from(*digit);
    *digit = (current / divisor) as u8;
    remainder = current % divisor;
  }
  remainder
}

/// `base_value ^ exponent`, where the exponent is written in `exponent_base`.
pub(crate) fn pow(base_value: &[u8], exponent: &[u8], base: u32, exponent_base: u32) -> Vec<u8> {
  let mut exponent = exponent.to_vec();
  trim(&mut exponent);
  let mut square = base_value.to_vec();
  trim(&mut square);
  let mut result = vec![1];

  // Right-to-left binary exponentiation: peel off one bit of the exponent at a time.
  while !is_zero(&exponent) {
    let bit = div_small(&mut exponent, 2, exponent_base);
    trim(&mut exponent);
    if bit == 1 {
      result = mul(&result, &square, base);
    }
    if !is_zero(&exponent) {
      square = mul(&square, &square, base);
    }
  }
  result
}

/// Digits of `value` in `base`.
pub(crate) fn from_u64(mut value: u64, base: u32) -> Vec<u8> {
  if value == 0 {
    return vec![0];
  }
  let base = u64::from(base);
  let mut out = Vec::new();
  while value > 0 {
    out.push((value % base) as u8);
    value /= base;
  }
  out
}

/// Re-express a number written in `from` base as digits of the `to` base.
pub(crate) fn convert(digits: &[u8], from: u32, to: u32) -> Vec<u8> {
  let digits = &digits[..significant_len(digits)];
  let from = u64::from(from);
  let mut out = vec![0];
  // Horner's method, most significant digit first: out = out * from + digit.
  // Source digits are gathered into chunks (out = out * from^k + chunk) so
  // each pass over `out` consumes several of them.
  let (mut chunk, mut scale) = (0, 1);
  for &digit in digits.iter().rev() {
    chunk = chunk * from + u64::from(digit);
    scale *= from;
    if scale * from > CHUNK_LIMIT {
      mul_small_add(&mut out, scale, chunk, to);
      chunk = 0;
      scale = 1;
    }
  }
  if scale > 1 {
    mul_small_add(&mut out, scale, chunk, to);
  }
  trim(&mut out);
  out
}

/// Length of the longest run of identical neighbouring digits (at least 1).
pub(crate) fn longest_run(d: &[u8]) -> usize {
  let mut longest = 1;
  let mut run = 1;
  for pair in d.windows(2) {
    if pair[0] == pair[1] {
      run += 1;
      longest = longest.max(run);
    } else {
      run = 1;
    }
  }
  longest
}

/// Whether no run of identical neighbouring digits is longer than `max_run`.
pub(crate) fn runs_within(d: &[u8], max_run: usize) -> bool {
  let mut run = 1;
  for pair in d.windows(2) {
    if pair[0] == pair[1] {
      run += 1;
      if run > max_run {
        return false;
      }
    } else {
      run = 1;
    }
  }
  true
}

/// Index of the most significant digit that ends a run longer than `max_run`.
fn first_run_violation(d: &[u8], max_run: usize) -> Option<usize> {
  let mut run = 0;
  for i in (0..d.len()).rev() {
    if i + 1 < d.len() && d[i] == d[i + 1] {
      run += 1;
    } else {
      run = 1;
    }
    if run > max_run {
      return Some(i);
    }
  }
  None
}

/// Raise `d` to the smallest value at least as large whose runs of identical
/// neighbouring digits are no longer than `max_run`.  The width of `d` is kept
/// (zero padding counts as digits) unless the value needs another digit.
///
/// If the most significant violation ends at position `i`, every valid number
/// at or above `d` must have a larger prefix `d[i..]`, so the prefix is
/// incremented and the rest is filled with the smallest valid suffix: zeros,
/// with a one wherever another zero would make a run too long.  Incrementing
/// can create a new violation in the prefix, so this repeats until valid.
pub(crate) fn raise_to_valid_runs(d: &mut Vec<u8>, max_run: usize, base: u32) {
  debug_assert!(max_run >= 1 && base >= 2);
  while let Some(i) = first_run_violation(d, max_run) {
    increment_from(d, i, base);

    let mut previous = d[i];
    let mut run = d[i..].iter().take_while(|&&x| x == previous).count();
    for position in (0..i).rev() {
      let digit = if previous == 0 && run >= max_run {
        1
      } else {
        0
      };
      if digit == previous {
        run += 1;
      } else {
        previous = digit;
        run = 1;
      }
      d[position] = digit;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn value(d: &[u8], base: u32) -> u128 {
    d.iter()
      .rev()
      .fold(0, |acc, &x| acc * u128::from(base) + u128::from(x))
  }

  #[test]
  fn significant_len_ignores_padding() {
    assert_eq!(significant_len(&[0]), 1);
    assert_eq!(significant_len(&[0, 0, 0]), 1);
    assert_eq!(significant_len(&[5, 0, 0]), 1);
    assert_eq!(significant_len(&[5, 0, 3, 0]), 3);
  }

  #[test]
  fn add_assign_carries_and_widens() {
    let mut a = vec![9, 9, 9];
    add_assign(&mut a, &[9, 9, 9], 10);
    assert_eq!(a, vec![8, 9, 9, 1]);
    let mut a = vec![1];
    add_assign(&mut a, &[1, 1, 0, 0], 10);
    assert_eq!(a, vec![2, 1, 0, 0]);
    let mut a = vec![254];
    add_assign(&mut a, &[254], 255);
    assert_eq!(a, vec![253, 1]);
  }

  #[test]
  fn mul_matches_integer_math() {
    for &base in &[2u32, 3, 10, 16, 255] {
      for x in [0u64, 1, 7, 254, 1000, 65_535, 123_456_789].iter() {
        for y in [0u64, 1, 3, 255, 99_999, 4_294_967_295].iter() {
          let product = mul(&from_u64(*x, base), &from_u64(*y, base), base);
          assert_eq!(value(&product, base), u128::from(*x) * u128::from(*y));
        }
      }
    }
  }

  #[test]
  fn convert_has_no_width_limit() {
    // 2^64 written in binary is a 65 digit number.
    let mut binary = vec![0u8; 64];
    binary.push(1);
    let decimal = convert(&binary, 2, 10);
    let text: String = decimal.iter().rev().map(|d| char::from(b'0' + d)).collect();
    assert_eq!(text, "18446744073709551616");
  }

  #[test]
  fn convert_matches_from_u64() {
    for &from in &[2u32, 3, 10, 16, 200, 255] {
      for &to in &[2u32, 7, 10, 36, 255] {
        for &v in &[0u64, 1, 254, 255, 256, 65_535, 1 << 32, u64::MAX] {
          assert_eq!(convert(&from_u64(v, from), from, to), from_u64(v, to));
        }
      }
    }
  }

  #[test]
  fn pow_by_squaring() {
    assert_eq!(value(&pow(&[2], &from_u64(10, 10), 10, 10), 10), 1024);
    assert_eq!(value(&pow(&[7], &[0], 10, 10), 10), 1);
    assert_eq!(value(&pow(&[0], &[0], 10, 10), 10), 1);
    assert_eq!(value(&pow(&[0], &[3], 10, 10), 10), 0);
    // exponent written in a different base (binary 101 = 5)
    assert_eq!(value(&pow(&[3], &[1, 0, 1], 10, 2), 10), 243);
  }

  #[test]
  fn runs() {
    assert_eq!(longest_run(&[1]), 1);
    assert_eq!(longest_run(&[1, 1, 2, 2, 2, 1]), 3);
    assert!(runs_within(&[1, 1, 2], 2));
    assert!(!runs_within(&[1, 1, 1], 2));
  }
}
