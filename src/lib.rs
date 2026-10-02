// Copyright 2017 Daniel P. Clark & other digits Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
#![forbid(unsafe_code)]
#![deny(
  missing_docs,
  trivial_casts,
  trivial_numeric_casts,
  missing_debug_implementations,
  missing_copy_implementations,
  unused_import_braces,
  unused_qualifications
)]
//! # digits
//!
//! `Digits` is an unbounded number written in a numeric base made of any
//! characters you choose.  Think of it as a score card flipper for a custom
//! character set: you can step through every sequence of characters (for
//! example to brute force a character space) far beyond what `u64` can count,
//! and do basic math along the way.
//!
//! ```
//! use digits::prelude::*;
//!
//! let base = BaseCustom::<char>::new("abc".chars().collect());
//! let mut counter = Digits::new(base, "aa");
//!
//! assert_eq!(counter.succ().to_s(), "ab");
//! assert_eq!(counter.succ().to_s(), "ac");
//! assert_eq!(counter.succ().to_s(), "ba");
//! ```
//!
//! ## Zero padding
//!
//! Leading "zero" characters (the first character of the base) are kept as
//! part of a number's width, which is what makes `Digits` useful as a fixed
//! width counter:
//!
//! * `succ`, `pred_till_zero` and addition keep the width of the widest
//!   operand and only grow when a carry needs another digit.
//! * Multiplication, `pow` and base conversion produce unpadded results.
//! * `zero_fill` and `zero_trim` change the padding explicitly.
//! * Equality, hashing and ordering compare numeric values and ignore padding.
//! * The adjacency methods count padding zeros as digits.
//!
//! ## Threads
//!
//! `Digits` is `Send + Sync`.  The character mapping is shared through an
//! `Arc`, so cloning a `Digits` or creating new ones from it with `propagate`,
//! `zero`, `one` or `new_mapped` never copies the mapping.
#[doc(no_inline)]
pub use base_custom::BaseCustom;

// Compile and run the README examples as doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

mod arith;
mod ops;

/// The characters of a numeric base and a lookup from character to digit value.
struct Mapping {
  base_custom: BaseCustom<char>,
  chars: Vec<char>,
  lookup: HashMap<char, u8>,
}

impl Mapping {
  fn new(base_custom: BaseCustom<char>) -> Arc<Mapping> {
    let chars: Vec<char> = (0..base_custom.base as usize)
      .map(|i| {
        *base_custom
          .nth(i)
          .expect("BaseCustom character out of range")
      })
      .collect();
    let lookup = chars
      .iter()
      .enumerate()
      .map(|(i, &c)| (c, i as u8))
      .collect();
    Arc::new(Mapping {
      base_custom,
      chars,
      lookup,
    })
  }

  fn base(&self) -> u32 {
    self.chars.len() as u32
  }

  fn digit(&self, c: char) -> u8 {
    match self.lookup.get(&c) {
      Some(&digit) => digit,
      None => panic!("{:?} is not a character of this numeric base", c),
    }
  }
}

/// An unbounded number in a custom character base provided by a `BaseCustom`.
///
/// Internally a `Digits` is a vector of digit values (least significant first)
/// plus a shared reference to its character mapping, so every operation works
/// iteratively and the only limit on size is memory.
#[derive(Clone)]
pub struct Digits {
  mapping: Arc<Mapping>,
  // Least significant digit first; never empty.
  digits: Vec<u8>,
}

impl Digits {
  /// Add two Digits instances together, returning a new instance.
  ///
  /// The result keeps the width (zero padding) of the wider operand.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let eleven = Digits::new(base10.clone(), "11");
  /// let two = Digits::new(base10, "2");
  ///
  /// assert_eq!(eleven.add(&two).to_s(), "13");
  /// ```
  ///
  /// _This will panic if the numeric bases are not the same size._
  #[must_use]
  pub fn add(&self, other: impl Borrow<Digits>) -> Self {
    let mut result = self.clone();
    result.mut_add(other);
    result
  }

  /// Returns a vector of each character's position in the mapping, most
  /// significant first.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  /// let num = Digits::new(base16, "0fa");
  ///
  /// assert_eq!(num.as_mapping_vec(), vec![0, 15, 10]);
  /// ```
  pub fn as_mapping_vec(&self) -> Vec<u64> {
    self.digits.iter().rev().map(|&d| u64::from(d)).collect()
  }

  /// The size of the numeric base.
  pub fn base(&self) -> usize {
    self.mapping.chars.len()
  }

  /// Generate a Digits with this instance's character mapping from a `u64`,
  /// or from another `Digits` even if it uses a different numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let two = Digits::new(base10, "2");
  /// let three = two.gen(3_u64);
  ///
  /// assert_eq!(three.to_s(), "3");
  /// ```
  pub fn gen<T>(&self, other: T) -> Self
  where
    Self: From<(BaseCustom<char>, T)>,
  {
    Digits::from((self.mapping.base_custom.clone(), other))
  }

  /// Returns whether no character repeats more than `adjacent` times next to
  /// itself.  Stops at the first violation.
  ///
  /// Same as a more efficient `self.max_adjacent() <= adjacent`.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let num = Digits::new(base10, "1223");
  ///
  /// assert!(num.is_valid_adjacent(1));
  /// assert!(!num.is_valid_adjacent(0));
  /// ```
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
  /// let two = Digits::new(base10.clone(), "2");
  /// let three = Digits::new(base10, "3");
  ///
  /// assert!(two.is_compat(&three));
  /// ```
  pub fn is_compat(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.mapping, &other.mapping) || self.mapping.chars == other.mapping.chars
  }

  /// Returns whether the value is one, ignoring zero padding.
  pub fn is_one(&self) -> bool {
    self.significant() == [1]
  }

  /// Returns whether the value is zero, ignoring zero padding.
  pub fn is_zero(&self) -> bool {
    arith::is_zero(&self.digits)
  }

  /// The number of digits, including zero padding.
  pub fn length(&self) -> usize {
    self.digits.len()
  }

  /// The character mapping this number is written in.
  pub fn mapping(&self) -> &BaseCustom<char> {
    &self.mapping.base_custom
  }

  /// The largest count of identical characters adjacent to a character of the
  /// same kind.
  ///
  /// Adjacent is a non-inclusive count: a run of 7 identical characters is one
  /// character adjacent to 6 others, so it counts as 6.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let num = Digits::new(base10, "557771");
  ///
  /// assert_eq!(num.max_adjacent(), 2);
  /// ```
  ///
  /// The above example demonstrates that there are 2 adjacent 7s next to a 7
  /// and that is the biggest adjacent set of numbers.
  pub fn max_adjacent(&self) -> usize {
    arith::longest_run(&self.digits) - 1
  }

  /// Multiply two Digits instances together, returning a new instance.
  ///
  /// The result has no zero padding.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let eleven = Digits::new(base10.clone(), "11");
  /// let two = Digits::new(base10, "2");
  ///
  /// assert_eq!(eleven.mul(&two).to_s(), "22");
  /// ```
  ///
  /// _This will panic if the numeric bases are not the same size._
  #[must_use]
  pub fn mul(&self, other: impl Borrow<Digits>) -> Self {
    let other = other.borrow();
    self.assert_same_base(other, "multiply");
    Digits {
      mapping: Arc::clone(&self.mapping),
      digits: arith::mul(&self.digits, &other.digits, self.mapping.base()),
    }
  }

  /// Add another Digits to this one in place.
  ///
  /// The result keeps the width (zero padding) of the wider operand.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(base10.clone(), "11");
  /// let two = Digits::new(base10, "2");
  ///
  /// assert_eq!(eleven.mut_add(two).to_s(), "13");
  /// ```
  ///
  /// _This will panic if the numeric bases are not the same size._
  pub fn mut_add(&mut self, other: impl Borrow<Digits>) -> &mut Self {
    let other = other.borrow();
    self.assert_same_base(other, "add");
    arith::add_assign(&mut self.digits, &other.digits, self.mapping.base());
    self
  }

  /// Multiply this Digits by another in place.
  ///
  /// The result has no zero padding.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(base10.clone(), "11");
  /// let two = Digits::new(base10, "2");
  ///
  /// assert_eq!(eleven.mut_mul(two).to_s(), "22");
  /// ```
  ///
  /// _This will panic if the numeric bases are not the same size._
  pub fn mut_mul(&mut self, other: impl Borrow<Digits>) -> &mut Self {
    let other = other.borrow();
    self.assert_same_base(other, "multiply");
    self.digits = arith::mul(&self.digits, &other.digits, self.mapping.base());
    self
  }

  /// Creates a new Digits instance with the provided character set and value.
  ///
  /// The first parameter is a `BaseCustom` which defines and maps all values.
  /// The second is the number written with characters from that set.  An
  /// empty string is zero.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9");
  ///
  /// assert_eq!(nine.to_s(), "9");
  /// ```
  ///
  /// _This will panic if the number contains a character outside the base._
  pub fn new<S>(mapping: BaseCustom<char>, number: S) -> Digits
  where
    S: Into<String>,
  {
    Digits::parse(Mapping::new(mapping), &number.into())
  }

  fn parse(mapping: Arc<Mapping>, number: &str) -> Digits {
    let mut digits: Vec<u8> = number.chars().rev().map(|c| mapping.digit(c)).collect();
    if digits.is_empty() {
      digits.push(0);
    }
    Digits { mapping, digits }
  }

  /// Create a Digits with this instance's character mapping from a slice of
  /// positional values, most significant first.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  /// let builder = Digits::new(base16, "");
  /// let num = builder.new_mapped(&[1, 0, 2, 1]).unwrap();
  ///
  /// assert_eq!(num.to_s(), "1021");
  /// ```
  ///
  /// If zero had been Z in the example above the same slice `[1,0,2,1]` would
  /// have produced "1Z21".  Each value is the index of a character in the
  /// mapping, regardless of numeric base.
  ///
  /// If a value is not below the numeric base size this returns
  /// `Err("Character mapping out of range!")`.  An empty slice is zero.
  pub fn new_mapped(&self, places: &[u64]) -> Result<Self, &'static str> {
    if places.iter().any(|&x| x >= u64::from(self.mapping.base())) {
      return Err("Character mapping out of range!");
    }
    let mut digits: Vec<u8> = places.iter().rev().map(|&x| x as u8).collect();
    if digits.is_empty() {
      digits.push(0);
    }
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
    Digits {
      mapping: Mapping::new(mapping),
      digits: vec![1],
    }
  }

  /// Creates a new Digits instance with value of zero and the provided character mapping.
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
    Digits {
      mapping: Mapping::new(mapping),
      digits: vec![0],
    }
  }

  /// Steps to the next larger value in which no character is adjacent to more
  /// than `adjacent` identical characters.
  ///
  /// Zero padding counts as characters and the width is kept unless the value
  /// needs another digit.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "98");
  ///
  /// assert_eq!(num.next_non_adjacent(0).to_s(), "101");
  /// ```
  pub fn next_non_adjacent(&mut self, adjacent: usize) -> &mut Self {
    self.succ();
    self.raise_to_non_adjacent(adjacent);
    self
  }

  /// Creates a new Digits instance with value of one and this instance's character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9");
  /// let one = nine.one();
  ///
  /// assert_eq!(one.to_s(), "1");
  /// ```
  pub fn one(&self) -> Self {
    self.with_digits(vec![1])
  }

  /// The “pinky” is the smallest digit, a.k.a. the right most character.
  pub fn pinky(&self) -> char {
    self.mapping.chars[usize::from(self.digits[0])]
  }

  /// Raises this Digits to the power of the given Digits in place.
  ///
  /// The exponent may use any numeric base.  The result has no zero padding.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(base10.clone(), "11");
  /// let two = Digits::new(base10, "2");
  ///
  /// assert_eq!(eleven.pow(two).to_s(), "121");
  /// ```
  pub fn pow(&mut self, pwr: impl Borrow<Digits>) -> &mut Self {
    let pwr = pwr.borrow();
    self.digits = arith::pow(
      &self.digits,
      &pwr.digits,
      self.mapping.base(),
      pwr.mapping.base(),
    );
    self
  }

  /// Subtracts one unless the value is zero, in which case it stays zero.
  /// The width is kept, so "10" becomes "09".
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut ten = Digits::new(base10, "10");
  ///
  /// assert_eq!(ten.pred_till_zero().to_s(), "09");
  /// ```
  pub fn pred_till_zero(&mut self) -> &mut Self {
    if !self.is_zero() {
      arith::decrement(&mut self.digits, self.mapping.base());
    }
    self
  }

  /// Moves an invalid starting value to one below the next value that
  /// satisfies the adjacency limit, so that `step_non_adjacent` lands on it.
  /// A value that already satisfies the limit is left unchanged.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "0003");
  ///
  /// assert_eq!(num.prep_non_adjacent(1).to_s(), "0009");
  /// ```
  ///
  /// In the example above the prep moves to the valid state of "0010" and then
  /// subtracts one to "0009" so that `step_non_adjacent` will add 1 and return
  /// to "0010".
  ///
  /// `next_non_adjacent` and `step_non_adjacent` handle invalid starting
  /// values themselves, so this is only needed to inspect that state.
  pub fn prep_non_adjacent(&mut self, adjacent: usize) -> &mut Self {
    if !self.is_valid_adjacent(adjacent) {
      self.raise_to_non_adjacent(adjacent);
      self.pred_till_zero();
    }
    self
  }

  /// Creates a new Digits instance with this instance's character mapping and the given value.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9");
  /// let forty_two = nine.propagate("42");
  ///
  /// assert_eq!(forty_two.to_s(), "42");
  /// ```
  ///
  /// _This will panic if the number contains a character outside the base._
  pub fn propagate<S>(&self, number: S) -> Self
  where
    S: Into<String>,
  {
    Digits::parse(Arc::clone(&self.mapping), &number.into())
  }

  /// Counts how many characters from the right match the mapping index given.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("ABC3456789".chars().collect());
  /// let num = Digits::new(base10, "34BBB");
  ///
  /// assert_eq!(num.rcount(1), 3);
  /// ```
  pub fn rcount(&self, character_index: u8) -> usize {
    self
      .digits
      .iter()
      .take_while(|&&d| d == character_index)
      .count()
  }

  /// An alias for `clone`.
  #[deprecated(since = "2.0.0", note = "use `clone` instead")]
  pub fn replicate(self) -> Self {
    self
  }

  /// Same as `next_non_adjacent`.
  ///
  /// Before 2.0 this required a valid starting value (see
  /// `prep_non_adjacent`).  It now handles any starting value.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "98");
  ///
  /// assert_eq!(num.step_non_adjacent(0).to_s(), "101");
  /// ```
  pub fn step_non_adjacent(&mut self, adjacent: usize) -> &mut Self {
    self.next_non_adjacent(adjacent)
  }

  /// Adds one in place, keeping the width unless the value needs another digit.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut num = Digits::new(base10, "0099");
  ///
  /// assert_eq!(num.succ().to_s(), "0100");
  /// ```
  pub fn succ(&mut self) -> &mut Self {
    arith::increment(&mut self.digits, self.mapping.base());
    self
  }

  /// The full value of all digits as a String, including zero padding.
  pub fn to_s(&self) -> String {
    self
      .digits
      .iter()
      .rev()
      .map(|&d| self.mapping.chars[usize::from(d)])
      .collect()
  }

  /// Creates a new Digits instance with value of zero and this instance's character mapping.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let nine = Digits::new(base10, "9");
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
  /// let mut nine = Digits::new(base10, "9");
  /// nine.zero_fill(4);
  ///
  /// assert_eq!(nine.to_s(), "0009");
  /// ```
  pub fn zero_fill(&mut self, length: usize) {
    if self.digits.len() < length {
      self.digits.resize(length, 0);
    }
  }

  /// Removes zero padding from the left of the current number.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::prelude::*;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// let mut nine = Digits::new(base10, "0009");
  /// nine.zero_trim();
  ///
  /// assert_eq!(nine.to_s(), "9");
  /// ```
  pub fn zero_trim(&mut self) {
    arith::trim(&mut self.digits);
  }

  fn assert_same_base(&self, other: &Digits, operation: &str) {
    assert!(
      self.base() == other.base(),
      "cannot {} Digits of base {} with Digits of base {}",
      operation,
      self.base(),
      other.base()
    );
  }

  /// This value, without zero padding, in another mapping (sharing that mapping's `Arc`).
  fn converted_to(&self, mapping: &Arc<Mapping>) -> Digits {
    let digits = if self.mapping.chars == mapping.chars {
      self.significant().to_vec()
    } else {
      arith::convert(&self.digits, self.mapping.base(), mapping.base())
    };
    Digits {
      mapping: Arc::clone(mapping),
      digits,
    }
  }

  fn raise_to_non_adjacent(&mut self, adjacent: usize) {
    arith::raise_to_valid_runs(
      &mut self.digits,
      adjacent.saturating_add(1),
      self.mapping.base(),
    );
  }

  fn significant(&self) -> &[u8] {
    &self.digits[..arith::significant_len(&self.digits)]
  }

  fn with_digits(&self, digits: Vec<u8>) -> Digits {
    Digits {
      mapping: Arc::clone(&self.mapping),
      digits,
    }
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
  /// let mut nine = Digits::new(base10, "0009");
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

impl From<(BaseCustom<char>, u64)> for Digits {
  fn from(d: (BaseCustom<char>, u64)) -> Digits {
    let mapping = Mapping::new(d.0);
    let digits = arith::from_u64(d.1, mapping.base());
    Digits { mapping, digits }
  }
}

/// Converts the value into the given mapping.  The result has no zero padding.
impl From<(BaseCustom<char>, Digits)> for Digits {
  fn from(d: (BaseCustom<char>, Digits)) -> Digits {
    d.1.converted_to(&Mapping::new(d.0))
  }
}

/// Converts the second value into the mapping of the first.  The result has no
/// zero padding.
impl From<(Digits, Digits)> for Digits {
  fn from(d: (Digits, Digits)) -> Digits {
    d.1.converted_to(&d.0.mapping)
  }
}

impl From<Digits> for String {
  fn from(d: Digits) -> String {
    d.to_s()
  }
}

impl<'a> From<&'a Digits> for String {
  fn from(d: &'a Digits) -> String {
    d.to_s()
  }
}

/// Writes the number, including zero padding.  Width, fill and alignment
/// flags are honoured.
impl fmt::Display for Digits {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.pad(&self.to_s())
  }
}

impl fmt::Debug for Digits {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Digits")
      .field("value", &self.to_s())
      .field("base", &self.base())
      .field("mapping", &self.mapping.chars.iter().collect::<String>())
      .finish()
  }
}

/// Values are equal when they share a character mapping and have the same
/// numeric value; zero padding is ignored, so "009" equals "9".
impl PartialEq for Digits {
  fn eq(&self, other: &Digits) -> bool {
    self.is_compat(other) && self.significant() == other.significant()
  }
}

impl Eq for Digits {}

impl Hash for Digits {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.significant().hash(state);
  }
}

/// Compares numeric values, ignoring zero padding.  Numbers with different
/// character mappings are not comparable and return `None`.
impl PartialOrd for Digits {
  fn partial_cmp(&self, other: &Digits) -> Option<Ordering> {
    if self.is_compat(other) {
      Some(arith::cmp(&self.digits, &other.digits))
    } else {
      None
    }
  }
}

#[allow(missing_docs)]
pub mod prelude {
  #[doc(inline)]
  pub use super::Digits;
  #[doc(inline)]
  pub use base_custom::BaseCustom;
}

/// A decimal zero.
impl Default for Digits {
  fn default() -> Digits {
    Digits::new_zero(radices::decimal_base())
  }
}

/// A default Radix modules including most common numeric bases.
pub mod radices {
  use super::BaseCustom;
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
  fn binary(&self) -> Self;
  /// Convert current `Digits` to octal
  fn octal(&self) -> Self;
  /// Convert current `Digits` to decimal
  fn decimal(&self) -> Self;
  /// Convert current `Digits` to hexadecimal
  fn hex(&self) -> Self;
  /// Convert current `Digits` to lowercase hexadecimal
  fn hexl(&self) -> Self;
}

impl Radix for Digits {
  fn binary(&self) -> Digits {
    self.converted_to(&Mapping::new(radices::binary_base()))
  }

  fn octal(&self) -> Digits {
    self.converted_to(&Mapping::new(radices::octal_base()))
  }

  fn decimal(&self) -> Digits {
    self.converted_to(&Mapping::new(radices::decimal_base()))
  }

  fn hex(&self) -> Digits {
    self.converted_to(&Mapping::new(radices::hex_base()))
  }

  fn hexl(&self) -> Digits {
    self.converted_to(&Mapping::new(radices::hexl_base()))
  }
}
