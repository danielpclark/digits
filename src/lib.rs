// Copyright 2017 Daniel P. Clark & other digits Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
#![deny(missing_docs,trivial_casts,trivial_numeric_casts,
        missing_debug_implementations, missing_copy_implementations,
        unsafe_code,unstable_features,unused_import_braces,unused_qualifications)
]
//! # digits
//!
//! The digits crate is a linked list implementation of a score card flipper.  But
//! in this case it's with any characters you want and you can enumerate through
//! possibilities beyond the numeric limits intrinsic in basic numerc types like `u64`.
//!
//! Primary use case would be brute forcing character sequences.
extern crate base_custom;
#[doc(no_inline)]
pub use base_custom::BaseCustom;
use std::fmt;
use std::ops::{
  Add,AddAssign,Mul,MulAssign,BitXor,BitXorAssign
};
use std::cmp::{PartialOrd,Ordering};

/// This struct acts similar to a full number with a custom numeric character base.
/// But the underlying implementation is a linked list where all the methods recurse
/// as far as need to to implement the operations.
#[derive(Clone)]
pub struct Digits<'a> {
  mapping: &'a BaseCustom<char>,
  digit: u64,
  left: Option<Box<Digits<'a>>>,
}

impl<'a> Digits<'a> {
  /// Add two Digits instances together. This will panic if the two Digits don't share the same
  /// numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(&base10, "11".to_string());
  /// let two = Digits::new(&base10, "2".to_string());
  ///
  /// assert_eq!(eleven.add(two).to_s(), "13");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "13"
  /// ```
  pub fn add(&self, other: Self) -> Self {
    self.clone().mut_add_internal(other, false)
  }

  /// Allows you to generate/encode a Digits from a `u64` or other `Digits` even if they are of a
  /// different numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let two = Digits::new(&base10, "2".to_string());
  /// let three = two.gen(3_u64);
  ///
  /// assert_eq!(three.to_s(), "3");
  /// ```
  pub fn gen<T>(&self, other: T) -> Self
  where Self: From<(&'a BaseCustom<char>, T)> {
    Digits::from((self.mapping, other))
  }

  // the way to recurse and process Digits
  fn head_tail(self) -> (u64, Option<Box<Self>>) {
    match self.left {
      Some(bx) => (self.digit, Some(bx)),
      None => (self.digit, None),
    }
  }

  /// Determine if two Digits are compatible for addition or multiplication.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let two = Digits::new(&base10, "2".to_string());
  /// let three = Digits::new(&base10, "3".to_string());
  ///
  /// assert!(two.is_compat(&three));
  /// ```
  pub fn is_compat(&self, other: &Self) -> bool {
    self.mapping == other.mapping
  }

  // A non-consuming quick end check.
  // More efficient than calling `is_zero` when this applies.
  fn is_end(&self) -> bool {
    self.digit == 0 && match self.left { None => true, _ => false }
  }

  /// Returns bool value of if the number is one.
  pub fn is_one(&self) -> bool {
    if self.digit != 1 { return false }
    match &self.left {
      &None => { true },
      &Some(ref bx) => { bx.is_zero() },
    }
  }

  /// Returns bool value of if the number is zero.
  pub fn is_zero(&self) -> bool {
    if self.digit != 0 { return false }
    match &self.left {
      &None => { true },
      &Some(ref bx) => { bx.is_zero() },
    }
  }

  /// Returns a `usize` of the total linked list length.
  pub fn length(&self) -> usize {
    match &self.left {
      &None => 1,
      &Some(ref l) => { l.length() + 1 }
    }
  }

  /// Multiply two Digits instances together.  This will panic if the two Digits don't share the same
  /// numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(&base10, "11".to_string());
  /// let two = Digits::new(&base10, "2".to_string());
  ///
  /// assert_eq!(eleven.mul(two).to_s(), "22");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "22"
  /// ```
  pub fn mul(&self, other: Self) -> Self {
    self.multiply(other, 0)
  }

  // Internal implementation for multiply. Needs the recursive
  // value of powers of ten for addition.
  fn multiply(&self, other: Self, power_of_ten: usize) -> Self {
    assert!(self.mapping == other.mapping);
    let mut position: usize = power_of_ten;
    let mut o = Some(Box::new(other));
    let mut result = self.zero();

    loop {
      match o.clone() {
        Some(thing) => {
          let (dgt, tail) = thing.head_tail();
          o = tail;

          let mltply = self.propagate((self.digit * dgt).to_string()).pow_ten(position);

          let current_digit = self.propagate(self.mapping.gen(dgt).to_string());

          if let Some(ref bx) = self.left {
            result.mut_add_internal(bx.clone().multiply(current_digit, position + 1), true);
          };

          result.mut_add_internal( mltply, true );
        },
        None => break,
      }

      position += 1;
    }

    result
  }

  /// Add two Digits instances together.  This will panic if the two Digits don't share the same
  /// numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(&base10, "11".to_string());
  /// let two = Digits::new(&base10, "2".to_string());
  ///
  /// assert_eq!(eleven.mut_add(two).to_s(), "13");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "13"
  /// ```
  pub fn mut_add(&mut self, other: Self) -> Self {
    self.mut_add_internal(other, false)
  }
  fn mut_add_internal(&mut self, other: Self, trim: bool) -> Self {
    assert!(self.mapping == other.mapping);
    if other.is_end() { return self.clone(); };
    let (last, rest) = other.head_tail();

    // sums current single digit
    let added = self.propagate(self.mapping.gen(last + self.digit));
    let (l, r) = added.head_tail();
    self.digit = l;

    // sums for left
    let mut intermediate = Digits::new_zero(self.mapping);
    if let Some(dg) = r { intermediate.mut_add_internal(dg.replicate(), trim); }
    if let Some(dg) = rest { intermediate.mut_add_internal(dg.replicate(), trim); }

    match self.left.clone() {
      Some(bx) => {
        self.set_left( bx.replicate().mut_add_internal(intermediate, trim).clone(), trim );
      },
      None => {
        if !intermediate.is_zero() {
          self.set_left( intermediate, trim );
        }
      }
    };

    self.clone()
  }

  /// Multiply two Digits instances together.  This will panic if the two Digits don't share the same
  /// numeric base.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(&base10, "11".to_string());
  /// let two = Digits::new(&base10, "2".to_string());
  ///
  /// assert_eq!(eleven.mut_mul(two).to_s(), "22");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "22"
  /// ```
  pub fn mut_mul(&mut self, other: Self) -> Self {
    let (d, r) = self.multiply(other, 0).head_tail();
    self.digit = d;
    if let Some(rest) = r { self.set_left(rest.replicate(), true); }
    self.clone()
  }

  /// Creates a new Digits instance with the provided character set and value.
  ///
  /// The first parameter must be a BaseCustom object which defines and maps all values.
  /// The second parameter is a string value with all valid characters from the BaseCustom set.
  pub fn new<S>(mapping: &'a BaseCustom<char>, number: S) -> Digits<'a>
  where S: Into<String> {
    let number = number.into();
    if number.is_empty() { return Digits { mapping: mapping, digit: 0, left: None }; };
    let (last, rest) = {
      let mut n = number.chars().rev();
      (n.next().unwrap(), n.rev().collect::<String>())
    };

    let continuation = {
      if rest.is_empty() {
        None
      } else {
        Some(Box::new(Digits::new(&mapping, rest)))
      }
    };
    Digits {
      mapping: mapping,
      digit: mapping.decimal(last.to_string()),
      left: continuation,
    }
  }

  /// Creates a new Digits instance with value of one and the provided character mapping.
  pub fn new_one(mapping: &'a BaseCustom<char>) -> Self {
    Digits { mapping: mapping, digit: 1, left: None }
  }

  /// Creates a new Digits instance with value of zero and uses the provided character mapping.
  pub fn new_zero(mapping: &'a BaseCustom<char>) -> Self {
    Digits { mapping: mapping, digit: 0, left: None }
  }

  /// Creates a new Digits instance with value of one and uses the current character mapping.
  pub fn one(&self) -> Self {
    Digits::new_one(self.mapping)
  }

  /// The “pinky” is the smallest digit
  /// a.k.a. current digit in the linked list
  /// a.k.a. the right most digit.
  /// This will be a `char` value for that digit.
  pub fn pinky(&self) -> char {
    self.mapping.char(self.digit as usize).unwrap()
  }

  /// Multiplies self times the power-of given Digits parameter.
  ///
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  ///
  /// let mut eleven = Digits::new(&base10, "11".to_string());
  /// let two = Digits::new(&base10, "2".to_string());
  ///
  /// assert_eq!(eleven.pow(two).to_s(), "121");
  /// ```
  ///
  /// # Output
  ///
  /// ```text
  /// "121"
  /// ```
  pub fn pow(&mut self, mut pwr: Self) -> Self {
    if pwr.is_zero() { return self.one(); }
    let copy = self.clone();
    loop {
      match pwr.is_one() {
        true => break,
        false => {
          self.mut_mul(copy.clone());
        }
      }
      pwr.pred_till_zero();
    }
    self.clone()
  }

  // multiply self by 10ⁿ without using typical multiplication
  fn pow_ten(&self, positions: usize) -> Self {
    let mut result: Digits = self.clone();
    for _ in 0..positions {
      let original = result;
      result = Digits::new_zero(self.mapping);
      result.set_left(original, true);
    }
    result
  }

  /// Minuses one unless it's zero, then it just returns a Digits instance of zero.
  pub fn pred_till_zero(&mut self) -> Self {
    if self.is_zero() { return self.clone(); }
    if self.digit == 0 {
      self.digit = self.mapping.base - 1;
      match self.left.clone() {
        Some(ref mut bx) => self.set_left(bx.pred_till_zero(), false),
        None => self.left = None
      }
    } else {
      self.digit -= 1;
    }
    self.clone()
  }

  /// Creates a new Digits instance with the internal character set and given value.
  ///
  /// The parameter is a string value with all valid characters from the BaseCustom set.
  pub fn propagate<S>(&self, number: S) -> Self
  where S: Into<String> {
    Digits::new(self.mapping, number)
  }

  /// An alias for `clone`. _Useful for unboxing._
  pub fn replicate(self) -> Self { self.clone() }

  // logic for setting left linked list continuation
  fn set_left(&mut self, d: Digits<'a>, trim: bool) {
    if trim && d.is_end() {
      self.left = None;
    } else {
      self.left = Some(Box::new(d));
    }
  }

  /// Plus one.
  pub fn succ(&mut self) -> Self {
    let one = self.one();
    self.mut_add_internal(one, false)
  }

  /// Gives the full value of all digits within the linked list as a String.
  pub fn to_s(&self) -> String {
    let num = self.mapping.gen(self.digit);
    match &self.left {
      &None => format!("{}", num),
      &Some(ref bx) => format!("{}{}", bx.to_s(), num),
    }
  }

  /// Gives the full value of all digits within the linked list as a String.
  pub fn to_string(&self) -> String {
    self.to_s()
  }

  /// Creates a new Digits instance with value of zero and the current character mapping.
  pub fn zero(&self) -> Self {
    Digits::new_zero(self.mapping)
  }
}

#[allow(missing_docs)]
pub trait Into<String> {
  fn into(self) -> String;
}

impl<'a> From<(&'a BaseCustom<char>, u64)> for Digits<'a> {
  fn from(d: (&'a BaseCustom<char>, u64)) -> Digits<'a> {
    let mapping = d.0;
    let value = d.1;
    Digits::new(mapping, mapping.gen(value))
  }
}

impl<'a,'b> From<(&'a BaseCustom<char>, Digits<'b>)> for Digits<'a> {
  fn from(d: (&'a BaseCustom<char>, Digits<'b>)) -> Digits<'a> {
    let mapping = d.0;
    let source = d.1;
    let from_base = source.mapping.base;
    let mut result = Digits::new_zero(mapping);
    let mut pointer: Option<Box<Digits<'b>>> = Some(Box::new(source));
    let mut position = 0;
    loop {
      match pointer {
        Some(bx) => {
          let (h, t) = bx.head_tail();
          if h != 0 { // speed optimization
            result.mut_add_internal(
              Digits::new(mapping, mapping.gen(h)).mul(
                Digits::new(mapping, mapping.gen(from_base)).
                  pow(Digits::new(mapping, mapping.gen(position)))
              ),
              true
            );
          }
          position += 1;
          pointer = t;
        },
        None => break,
      }
    }
    result
  }
}

impl<'a,'b> From<(Digits<'a>, Digits<'b>)> for Digits<'a> {
  fn from(d: (Digits<'a>, Digits<'b>)) -> Digits<'a> {
    Digits::from((d.0.mapping, d.1))
  }
}

impl<'a> From<Digits<'a>> for String {
  fn from(d: Digits) -> String {
    d.to_s()
  }
}

impl<'a> Into<String> for Digits<'a> {
  fn into(self) -> String {
    self.to_s()
  }
}

impl Into<String> for String {
  fn into(self) -> String {
    self.clone()
  }
}

impl<'a> fmt::Display for Digits<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
   write!(f, "Digits<'a> — (Character: '{}', Decimal Value: {}{})",
     self.mapping.gen(self.digit), self.digit, {
       match self.left {
         None => "".to_string(),
         Some(ref l) => format!(", With Preceeding: '{}'", l.to_s()),
       }
     }
     )
  }
}

impl<'a> fmt::Debug for Digits<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} with base: {:?}", self, self.mapping)
  }
}

impl<'a> PartialEq for Digits<'a> {
  fn eq(&self, other: &Digits<'a>) -> bool {
    self.mapping == other.mapping &&
      self.digit == other.digit &&
      self.left == other.left
  }
}

impl<'a> Add for Digits<'a> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    self.clone().mut_add(other)
  }
}

impl<'a> AddAssign for Digits<'a> {
  fn add_assign(&mut self, other: Self) {
    self.mut_add(other);
  }
}

impl<'a> Mul for Digits<'a> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    self.multiply(other, 0)
  }
}

impl<'a> MulAssign for Digits<'a> {
  fn mul_assign(&mut self, other: Self) {
    self.mut_mul(other);
  }
}

impl<'a> BitXor for Digits<'a> {
  type Output = Self;
  fn bitxor(self, other: Self) -> Self {
    self.clone().pow(other)
  }
}

impl<'a> BitXorAssign for Digits<'a> {
  fn bitxor_assign(&mut self, other: Self) {
    self.pow(other);
  }
}

impl<'a> PartialOrd for Digits<'a> {
  fn partial_cmp(&self, other: &Digits<'a>) -> Option<Ordering> {
    assert!(self.mapping == other.mapping);
    if self.length() != other.length() {
      return self.length().partial_cmp(&other.length());
    }
    let mut result: Option<Ordering>;
    let mut a: Self = self.clone();
    let mut b: Self = other.clone();
    result = a.digit.partial_cmp(&b.digit);
    loop {
      match (a.left, b.left) {
        (Some(x),Some(y)) => {
          a = x.replicate();
          b = y.replicate();
          match a.digit.partial_cmp(&b.digit) {
            Some(Ordering::Equal) => (),
            Some(change) => { result = Some(change); },
            None => (),
          }
        }
        _ => { break }
      }
    }
    result
  }
}
