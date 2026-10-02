// Copyright 2017 Daniel P. Clark & other digits Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
#![deny(missing_docs,trivial_casts,trivial_numeric_casts,
        missing_debug_implementations, missing_copy_implementations,
        unsafe_code,unused_import_braces,unused_qualifications)
]
//! # digits
//!
//! The digits crate is a score card flipper.  But in this case it's with any
//! characters you want and you can enumerate through possibilities beyond the
//! numeric limits intrinsic in basic numerc types like `u64`.
//!
//! Primary use case would be brute forcing character sequences.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate base_custom;
#[doc(no_inline)]
pub use base_custom::BaseCustom;
use std::fmt;
use std::ops::{
  Add,AddAssign,Mul,MulAssign,BitXor,BitXorAssign
};
use std::cmp::{PartialOrd,Ordering};
use std::sync::Arc;

mod internal;
use internal::arith;

/// This struct acts similar to a full number with a custom numeric character base
/// which is provided and mapped via a `BaseCustom` instance.
///
/// The digits are stored in a vector, least significant first, along with a
/// shared `BaseCustom` mapping.  Every operation works iteratively, so the size
/// of a number is limited only by memory.
#[derive(Clone)]
pub struct Digits {
  mapping: Arc<BaseCustom<char>>,
  // Least significant digit first; never empty.
  digits: Vec<u8>,
}

impl Digits {
  /// Add two Digits instances together.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let eleven = Digits::new(base10.clone(), "11".to_string());
  /// let two = Digits::new(base10, "2".to_string());
  ///
  /// assert_eq!(eleven.add(two).to_s(), "13");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "13"
  /// ```
  ///
  /// _This will panic if numeric bases are not the same._
  pub fn add(&self, other: Self) -> Self {
    self.assert_same_base(&other);
    let mut result = self.clone();
    let radix = result.radix();
    arith::add_assign(&mut result.digits, &other.digits, radix);
    result
  }

  /// Returns a vector of each characters position mapping
  pub fn as_mapping_vec(&self) -> Vec<u64> {
    self.digits.iter().rev().map(|&d| u64::from(d)).collect()
  }

  /// Make numeric base size publicly available on Digits
  pub fn base(&self) -> usize {
    self.mapping.base as usize
  }

  /// Allows you to generate/encode a Digits from a `u64` or other `Digits` even if they are of a
  /// different numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let two = Digits::new(base10, "2".to_string());
  /// let three = two.gen(3_u64);
  ///
  /// assert_eq!(three.to_s(), "3");
  /// ```
  pub fn gen<T>(&self, other: T) -> Self
  where Self: From<(BaseCustom<char>, T)> {
    Digits::from((BaseCustom::clone(&self.mapping), other))
  }

  /// Returns true of false based on whether the limit of allowed adjacents is not exceeded.
  /// Early termination result when false.
  ///
  /// Same as being a more efficient `self.max_adjacent <= allowed_adjacent`.
  pub fn is_valid_adjacent(&self, adjacent: usize) -> bool {
    arith::runs_within(&self.digits, adjacent.saturating_add(1))
  }

  /// Returns whether the two Digits instances have the same numeric base and
  /// character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let two = Digits::new(base10.clone(), "2".to_string());
  /// let three = Digits::new(base10, "3".to_string());
  ///
  /// assert!(two.is_compat(&three));
  /// ```
  pub fn is_compat(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.mapping, &other.mapping) || self.mapping == other.mapping
  }

  /// Returns bool value of if the number is one.
  pub fn is_one(&self) -> bool {
    self.significant() == [1]
  }

  /// Returns bool value of if the number is zero.
  pub fn is_zero(&self) -> bool {
    arith::is_zero(&self.digits)
  }

  /// Returns a `usize` of the total number of digits.
  pub fn length(&self) -> usize {
    self.digits.len()
  }

  /// Give the count for the maximum of the same adjacent characters for this digit.
  ///
  /// Note that adjacent is a non-inclusive count.  So for 7 numbers it's 1 adjacent
  /// to 6 which will return 6.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let num = Digits::new(base10, "557771".to_string());
  ///
  /// assert_eq!(num.max_adjacent(), 2);
  /// ```
  ///
  /// The above example demonstrates that there are 2 adjacent 7s next to a 7
  /// and that is the biggest adjacent set of numbers.
  pub fn max_adjacent(&self) -> usize {
    arith::longest_run(&self.digits) - 1
  }

  /// Multiply two Digits instances together.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let eleven = Digits::new(base10.clone(), "11".to_string());
  /// let two = Digits::new(base10, "2".to_string());
  ///
  /// assert_eq!(eleven.mul(two).to_s(), "22");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "22"
  /// ```
  ///
  /// _This will panic if numeric bases are not the same._
  pub fn mul(&self, other: Self) -> Self {
    self.multiply(&other)
  }

  // An unpadded product.
  fn multiply(&self, other: &Digits) -> Self {
    self.assert_same_base(other);
    self.with_digits(arith::mul(&self.digits, &other.digits, self.radix()))
  }

  /// Add two Digits instances together.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(base10.clone(), "11".to_string());
  /// let two = Digits::new(base10, "2".to_string());
  ///
  /// assert_eq!(eleven.mut_add(two).to_s(), "13");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "13"
  /// ```
  ///
  /// _This will panic if numeric bases are not the same._
  pub fn mut_add(&mut self, other: Self) -> Self {
    self.mut_add_internal(&other);
    self.clone()
  }

  // Keeps this number's width (zero padding), growing only on a carry.
  fn mut_add_internal(&mut self, other: &Digits) {
    self.assert_same_base(other);
    let width = self.digits.len();
    let radix = self.radix();
    arith::add_assign(&mut self.digits, &other.digits, radix);
    let keep = width.max(arith::significant_len(&self.digits));
    self.digits.truncate(keep);
  }

  /// Multiply two Digits instances together.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(base10.clone(), "11".to_string());
  /// let two = Digits::new(base10, "2".to_string());
  ///
  /// assert_eq!(eleven.mut_mul(two).to_s(), "22");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "22"
  /// ```
  ///
  /// _This will panic if numeric bases are not the same._
  pub fn mut_mul(&mut self, other: Self) -> Self {
    self.mut_mul_internal(&other);
    self.clone()
  }

  fn mut_mul_internal(&mut self, other: &Digits) {
    self.assert_same_base(other);
    let product = arith::mul(&self.digits, &other.digits, self.radix());
    self.set_product(product);
  }

  // Products have no zero padding, except that a single digit product keeps
  // the padding of a number whose other digits are all zero ("02" * 4 is "08").
  fn set_product(&mut self, product: Vec<u8>) {
    let width = self.digits.len();
    let padded = product.len() == 1 && arith::is_zero(&self.digits[1..]);
    self.digits = product;
    if padded {
      self.digits.resize(width, 0);
    }
  }

  /// Creates a new Digits instance with the provided character set and value.
  ///
  /// The first parameter must be a BaseCustom object which defines and maps all values.
  /// The second parameter is a string value with all valid characters from the BaseCustom set.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9".to_string());
  ///
  /// assert_eq!(nine.to_s(), "9");
  /// ```
  pub fn new<S>(mapping: BaseCustom<char>, number: S) -> Digits
  where S: Into<String> {
    Digits::parse(Arc::new(mapping), &number.into())
  }

  fn parse(mapping: Arc<BaseCustom<char>>, number: &str) -> Digits {
    let mut digits: Vec<u8> = number.chars().rev().map(|c| {
      match mapping.position(c) {
        // A base has at most 255 characters, so a position fits in a u8.
        Some(position) => position as u8,
        None => panic!("{:?} is not a character of this numeric base", c),
      }
    }).collect();
    if digits.is_empty() { digits.push(0); }
    Digits { mapping: mapping, digits: digits }
  }

  /// Create a Digits from a Vector of from zero positional mappings for custom Digits numeric
  /// base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  /// let builder = Digits::new(base16, "".to_string());
  /// let num = builder.new_mapped(&vec![1,0,2,1]).ok().unwrap();
  ///
  /// assert_eq!(num.to_s(), "1021");
  /// ```
  ///
  /// If zero had been Z in the example above the same vector `vec![1,0,2,1]` would have
  /// produced a Digits instance of a Hex value of "1Z21".  The vector is the litteral positional
  /// map of the character(s) via an index from zero regardless of numeric base.
  ///
  /// If a number provided within the vector is higher than the numeric base size then the method
  /// will return an `Err(&'static str)` Result.
  pub fn new_mapped(&self, places: &[u64]) -> Result<Self, &'static str> {
    if places.iter().any(|&x| x >= self.mapping.base) {
      return Err("Character mapping out of range!");
    }
    let mut digits: Vec<u8> = places.iter().rev().map(|&x| x as u8).collect();
    if digits.is_empty() { digits.push(0); }
    Ok(self.with_digits(digits))
  }

  /// Creates a new Digits instance with value of one and the provided character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let one = Digits::new_one(base10);
  ///
  /// assert_eq!(one.to_s(), "1");
  /// ```
  pub fn new_one(mapping: BaseCustom<char>) -> Self {
    Digits { mapping: Arc::new(mapping), digits: vec![1] }
  }

  /// Creates a new Digits instance with value of zero and uses the provided character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let zero = Digits::new_zero(base10);
  ///
  /// assert_eq!(zero.to_s(), "0");
  /// ```
  pub fn new_zero(mapping: BaseCustom<char>) -> Self {
    Digits { mapping: Arc::new(mapping), digits: vec![0] }
  }

  /// Returns the next Digits in incrementing that only allows the given number of
  /// adjacent number duplicates.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "98".to_string());
  ///
  /// assert_eq!(num.next_non_adjacent(0).to_s(), "101");
  /// ```
  pub fn next_non_adjacent(&mut self, adjacent: usize) -> Self {
    self.increment();
    self.raise_to_non_adjacent(adjacent);
    self.clone()
  }

  /// Creates a new Digits instance with value of one and uses the current character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9".to_string());
  /// let one = nine.one();
  ///
  /// assert_eq!(one.to_s(), "1");
  /// ```
  pub fn one(&self) -> Self {
    self.with_digits(vec![1])
  }

  /// The “pinky” is the smallest digit
  /// a.k.a. the right most digit.
  /// This will be a `char` value for that digit.
  pub fn pinky(&self) -> char {
    self.character(self.digits[0])
  }

  /// Multiplies self times the power-of given Digits parameter.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(base10.clone(), "11".to_string());
  /// let two = Digits::new(base10, "2".to_string());
  ///
  /// assert_eq!(eleven.pow(two).to_s(), "121");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "121"
  /// ```
  pub fn pow(&mut self, pwr: Self) -> Self {
    self.pow_internal(&pwr);
    self.clone()
  }

  fn pow_internal(&mut self, pwr: &Digits) {
    if pwr.is_zero() {
      self.require_two_characters("hold one");
      self.digits = vec![1];
    } else if !pwr.is_one() {
      let power = arith::pow(&self.digits, &pwr.digits, self.radix(), pwr.radix());
      self.set_product(power);
    }
  }

  /// Minuses one unless it's zero, then it just returns a Digits instance of zero.
  pub fn pred_till_zero(&mut self) -> Self {
    if !self.is_zero() {
      let radix = self.radix();
      arith::decrement(&mut self.digits, radix);
    }
    self.clone()
  }

  /// Sometimes given starting Digits have more adjacent characters than is desired
  /// when proceeding with non-adjacent steps.  This method provides a valid initial
  /// state for `step_non_adjacent`'s algorithm to not miss any initial steps.
  ///
  /// _This method is used internally for `next_non_adjacent`.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "0003".to_string());
  ///
  /// assert_eq!(num.prep_non_adjacent(1).to_s(), "0009");
  /// ```
  ///
  /// In the example above the prep moves to a valid state of "0010" and then
  /// minuses one to "0009" so that `step_non_adjacent` will add 1 and return to the
  /// valid state of "0010" for this one-adjacent scenario.
  ///
  /// For performance in your own applications use this method once and continue iterating
  /// with `step_non_adjacent`.
  ///
  /// For convenience you may just use `next_non_adjacent` instead of prep and step.
  pub fn prep_non_adjacent(&mut self, adjacent: usize) -> Self {
    if !self.is_valid_adjacent(adjacent) {
      self.raise_to_non_adjacent(adjacent);
      self.pred_till_zero();
    }
    self.clone()
  }

  /// Creates a new Digits instance with the internal character set and given value.
  ///
  /// The parameter is a string value with all valid characters from the BaseCustom set.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9".to_string());
  /// let forty_two = nine.propagate("42".to_string());
  ///
  /// assert_eq!(forty_two.to_s(), "42");
  /// ```
  pub fn propagate<S>(&self, number: S) -> Self
  where S: Into<String> {
    Digits::parse(Arc::clone(&self.mapping), &number.into())
  }

  /// Right count of digits character index.
  ///
  /// Returns a `usize` of how many Digits values from the right
  /// match the BaseCustom index given for number.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("ABC3456789".chars().collect());
  /// let num = Digits::new(base10, "34BBB".to_string());
  ///
  /// assert_eq!(num.rcount(1), 3);
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// 3
  /// ```
  pub fn rcount(&self, character_index: u8) -> usize {
    self.digits.iter().take_while(|&&d| d == character_index).count()
  }

  /// An alias for `clone`. _Useful for unboxing._
  pub fn replicate(self) -> Self { self.clone() }

  /// Returns the next Digits in incrementing that only allows the given number of
  /// adjacent number duplicates.
  ///
  /// The starting value does not need to be valid for the adjacency limit
  /// already; this gives the same result as `next_non_adjacent`.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "98".to_string());
  ///
  /// assert_eq!(num.step_non_adjacent(0).to_s(), "101");
  /// ```
  pub fn step_non_adjacent(&mut self, adjacent: usize) -> Self {
    self.next_non_adjacent(adjacent)
  }

  /// Plus one.
  pub fn succ(&mut self) -> Self {
    self.increment();
    self.clone()
  }

  fn increment(&mut self) {
    self.require_two_characters("count up");
    let radix = self.radix();
    arith::increment(&mut self.digits, radix);
  }

  /// Gives the full value of all digits as a String.
  pub fn to_s(&self) -> String {
    self.digits.iter().rev().map(|&d| self.character(d)).collect()
  }

  /// Gives the full value of all digits as a String.
  pub fn to_string(&self) -> String {
    self.to_s()
  }

  /// Creates a new Digits instance with value of zero and the current character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9".to_string());
  /// let zero = nine.zero();
  ///
  /// assert_eq!(zero.to_s(), "0");
  /// ```
  pub fn zero(&self) -> Self {
    self.with_digits(vec![0])
  }

  /// Zero fills the left of the current number up to a total character length.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut nine = Digits::new(base10, "9".to_string());
  /// nine.zero_fill(4);
  ///
  /// assert_eq!(nine.to_s(), "0009");
  /// ```
  pub fn zero_fill(&mut self, length: usize) {
    if self.digits.len() < length {
      self.digits.resize(length, 0);
    }
  }

  /// Zero trims the left of the current number.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut nine = Digits::new(base10, "0009".to_string());
  /// nine.zero_trim();
  ///
  /// assert_eq!(nine.to_s(), "9");
  /// ```
  pub fn zero_trim(&mut self) {
    arith::trim(&mut self.digits);
  }

  fn assert_same_base(&self, other: &Digits) {
    assert!(self.base() == other.base());
  }

  fn character(&self, digit: u8) -> char {
    *self.mapping.nth(usize::from(digit)).expect("digit outside the numeric base")
  }

  // This value, without zero padding, in another mapping.
  fn converted_to(&self, mapping: Arc<BaseCustom<char>>) -> Digits {
    if mapping.base < 2 && !self.is_zero() {
      panic!("a numeric base of a single character can only hold zero");
    }
    let digits = arith::convert(&self.digits, self.radix(), mapping.base as u32);
    Digits { mapping: mapping, digits: digits }
  }

  fn radix(&self) -> u32 {
    self.mapping.base as u32
  }

  fn raise_to_non_adjacent(&mut self, adjacent: usize) {
    self.require_two_characters("step");
    let radix = self.radix();
    arith::raise_to_valid_runs(&mut self.digits, adjacent.saturating_add(1), radix);
  }

  // A `BaseCustom` built from repeats of one character (`['a', 'a']`) has a
  // single unit, so it can hold zero but cannot count.
  fn require_two_characters(&self, operation: &str) {
    if self.mapping.base < 2 {
      panic!("cannot {} in a numeric base of a single character, which can only hold zero", operation);
    }
  }

  fn significant(&self) -> &[u8] {
    &self.digits[..arith::significant_len(&self.digits)]
  }

  fn with_digits(&self, digits: Vec<u8>) -> Digits {
    Digits { mapping: Arc::clone(&self.mapping), digits: digits }
  }
}

/// Reverse mutates self into a reversed self.
pub trait Reverse {
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits,Reverse};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut nine = Digits::new(base10, "0009".to_string());
  ///
  /// nine.reverse();
  ///
  /// assert_eq!(nine.to_s(), "9000");
  /// ```
  fn reverse(&mut self);
}

impl Reverse for Digits {
  fn reverse(&mut self) {
    self.digits.reverse();
  }
}

#[allow(missing_docs)]
pub trait Into<String> {
  fn into(self) -> String;
}

impl From<(BaseCustom<char>, u64)> for Digits {
  fn from(d: (BaseCustom<char>, u64)) -> Digits {
    let text = d.0.gen(d.1);
    Digits::parse(Arc::new(d.0), &text)
  }
}

impl From<(BaseCustom<char>, Digits)> for Digits {
  fn from(d: (BaseCustom<char>, Digits)) -> Digits {
    d.1.converted_to(Arc::new(d.0))
  }
}

impl From<(Digits, Digits)> for Digits {
  fn from(d: (Digits, Digits)) -> Digits {
    if d.0.is_compat(&d.1) { return d.1; }
    d.1.converted_to(d.0.mapping)
  }
}

impl From<Digits> for String {
  fn from(d: Digits) -> String {
    d.to_s()
  }
}

impl Into<String> for Digits {
  fn into(self) -> String {
    self.to_s()
  }
}

impl Into<String> for String {
  fn into(self) -> String {
    self.clone()
  }
}

impl fmt::Display for Digits {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Digits — (Character: '{}', Decimal Value: {}{})",
      self.pinky(), self.digits[0], {
        if self.digits.len() > 1 {
          let preceding: String = self.digits[1..].iter().rev().map(|&d| self.character(d)).collect();
          format!(", With Preceeding: '{}'", preceding)
        } else {
          "".to_string()
        }
      }
    )
  }
}

impl fmt::Debug for Digits {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} with base: {:?}", self, self.mapping)
  }
}

impl PartialEq for Digits {
  fn eq(&self, other: &Digits) -> bool {
    self.is_compat(other) && self.digits == other.digits
  }
}

impl Add for Digits {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    self.clone().mut_add(other)
  }
}

impl AddAssign for Digits {
  fn add_assign(&mut self, other: Self) {
    self.mut_add_internal(&other);
  }
}

impl Mul for Digits {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    self.multiply(&other)
  }
}

impl MulAssign for Digits {
  fn mul_assign(&mut self, other: Self) {
    self.mut_mul_internal(&other);
  }
}

impl BitXor for Digits {
  type Output = Self;
  fn bitxor(self, other: Self) -> Self {
    self.clone().pow(other)
  }
}

impl BitXorAssign for Digits {
  fn bitxor_assign(&mut self, other: Self) {
    self.pow_internal(&other);
  }
}

impl PartialOrd for Digits {
  fn partial_cmp(&self, other: &Digits) -> Option<Ordering> {
    assert!(self.is_compat(other));
    Some(arith::cmp(&self.digits, &other.digits))
  }
}

#[allow(missing_docs)]
pub mod prelude {
  #[doc(inline)]
  pub use super::Digits;
  #[doc(inline)]
  pub use base_custom::BaseCustom;
}

impl Default for Digits {
  fn default() -> Digits {
    let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
    Digits::new_zero(base10)
  }
}

/// A default Radix modules including most common numeric bases.
pub mod radices {
  use super::*;
  /// Binary implementation of `BaseCustom`
  pub fn binary_base() -> BaseCustom<char> {
    BaseCustom::<char>::new("01".chars().collect())
  }

  /// Octal implementation of `BaseCustom`
  pub fn octal_base() -> BaseCustom<char> {
    BaseCustom::<char>::new("01234567".chars().collect())
  }

  /// Decimal implementation of `BaseCustom`
  pub fn decimal_base() -> BaseCustom<char> {
    BaseCustom::<char>::new("0123456789".chars().collect())
  }

  /// Hexadecimal implementation of `BaseCustom`
  pub fn hex_base() -> BaseCustom<char> {
    BaseCustom::<char>::new("0123456789ABCDEF".chars().collect())
  }

  /// Lowercase hexadecimal implementation of `BaseCustom`
  pub fn hexl_base() -> BaseCustom<char> {
    BaseCustom::<char>::new("0123456789abcdef".chars().collect())
  }
}

/// Default Radix type conversion for `Digits`
pub trait Radix {
  /// Convert current `Digits` to binary
  fn binary(&self)  -> Self;
  /// Convert current `Digits` to octal
  fn octal(&self)   -> Self;
  /// Convert current `Digits` to decimal
  fn decimal(&self) -> Self;
  /// Convert current `Digits` to hexadecimal
  fn hex(&self)     -> Self;
  /// Convert current `Digits` to lowercase hexadecimal
  fn hexl(&self)    -> Self;
}

impl Radix for Digits {
  fn binary(&self) -> Digits {
    Digits::from((radices::binary_base(), self.clone()))
  }

  fn octal(&self) -> Digits {
    Digits::from((radices::octal_base(), self.clone()))
  }

  fn decimal(&self) -> Digits {
    Digits::from((radices::decimal_base(), self.clone()))
  }

  fn hex(&self) -> Digits {
    Digits::from((radices::hex_base(), self.clone()))
  }

  fn hexl(&self) -> Digits {
    Digits::from((radices::hexl_base(), self.clone()))
  }
}
