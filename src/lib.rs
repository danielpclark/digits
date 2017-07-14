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
use base_custom::BaseCustom;
use std::fmt;

/// # struct Digits
///
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
  /// `new` creates a new Digits instance with the provided character set and value.
  ///
  /// The first parameter must be a BaseCustom object which defines and maps all values.
  /// The second parameter is a string value with all valid characters from the BaseCustom set.
  ///
  /// You can view the numbers current value at any time with `to_s()` or `to_string()`.
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

  /// `replicate` — alias for clone (useful for unboxing)
  pub fn replicate(self) -> Self { self.clone() }

  /// `propagate` creates a new number from the same underlying numeric base
  pub fn propagate<S>(&self, number: S) -> Self
  where S: Into<String> {
    Digits::new(self.mapping, number)
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

  // the way to recurse and process Digits
  fn head_tail(self) -> (u64, Option<Box<Self>>) {
    match self.left {
      Some(bx) => (self.digit, Some(bx)),
      None => (self.digit, None),
    }
  }

  // logic for setting left linked list continuation
  fn set_left(&mut self, d: Digits<'a>) {
    if d.is_end() {
      self.left = None;
    } else {
      self.left = Some(Box::new(d));
    }
  }

  /// `length` returns a `usize` of the total linked list length.
  pub fn length(&self) -> usize {
    match &self.left {
      &None => 1,
      &Some(ref l) => { l.length() + 1 }
    }
  }

  /// `zero` returns a Digits instance with value of zero and the current character mapping.
  pub fn zero(&self) -> Self {
    Digits::new_zero(self.mapping)
  }

  /// `new_zero` returns a Digits instance with value of zero and the current character mapping.
  pub fn new_zero(mapping: &'a BaseCustom<char>) -> Self {
    Digits { mapping: mapping, digit: 0, left: None }
  }

  /// `is_zero` returns bool value of if the number is zero
  pub fn is_zero(&self) -> bool {
    if self.digit != 0 { return false }
    match &self.left {
      &None => { true },
      &Some(ref bx) => { bx.is_zero() },
    }
  }

  /// `pinky` is the smallest digit.
  /// a.k.a. current digit in the linked list.
  /// a.k.a. the right most digit.
  /// This will be a char value for that digit.
  pub fn pinky(&self) -> char {
    self.mapping.char(self.digit as usize).unwrap()
  }

  /// `one` returns a Digits instance with value of one and the current character mapping.
  pub fn one(&self) -> Self {
    Digits::new_one(self.mapping)
  }

  /// `new_one` returns a Digits instance with value of one and the current character mapping.
  pub fn new_one(mapping: &'a BaseCustom<char>) -> Self {
    Digits { mapping: mapping, digit: 1, left: None }
  }

  // A non-consuming quick end check.
  // More efficient than calling `is_zero` when this applies.
  fn is_end(&self) -> bool {
    self.digit == 0 && match self.left { None => true, _ => false }
  }

  /// Add two Digits instances together.  The one the `add` method is called on
  /// must be mutable and modifies itself.  The other is consumed.
  ///
  /// Returns a clone of the updated `Self` as well.
  pub fn add(&mut self, other: Self) -> Self {
    assert!(self.mapping == other.mapping);
    if other.is_end() { return self.clone(); };
    let (last, rest) = other.head_tail();

    // sums current single digit
    let added = self.propagate(self.mapping.gen(last + self.digit));
    let (l, r) = added.head_tail();
    self.digit = l;

    // sums for left
    let mut intermediate = Digits::new_zero(self.mapping);
    if let Some(dg) = r { intermediate.add(dg.replicate()); }
    if let Some(dg) = rest { intermediate.add(dg.replicate()); }

    let current_left = match self.left.clone() {
      None => { Box::new(self.zero()) },
      Some(bx) => { bx },
    }.as_ref().clone();

    self.set_left( intermediate.add(current_left).clone() );
    self.clone()
  }

  /// Multiply two Digits instances together.  The one the `mul` method is called on
  /// must be mutable and modifies itself.  The other is consumed.
  ///
  /// Returns a clone of the updated `Self` as well.
  pub fn mul(&mut self, other: Self) -> Self {
    self.multiply(other, 0)
  }

  fn multiply(&mut self, other: Self, power_of_ten: u32) -> Self {
    println!("STILL DEBUGGING THIS!");
    let mut additives: Vec<Digits> = vec![];
    let mut position: u32 = power_of_ten;
    let mut o = Some(Box::new(other));
    loop {
      match o.clone() {
        Some(thing) => {
          let (dgt, tail) = thing.head_tail();
          o = tail;

          println!("{:?} * {:?} = {:?}", self.digit, dgt, self.digit * dgt);
          let mltply = self.propagate((self.digit * dgt * 10_u64.pow(position)).to_string());
          println!("{:?} {}", position, mltply.to_s());

          let mapping = self.mapping.clone();

          if let Some(ref mut bx) = self.left {
            additives.push(
              bx.clone().multiply(
                Digits::new(
                  mapping,
                  dgt.to_string()
                ),
                position + 1
              )
            );
          };

          if !mltply.is_zero() { additives.push( mltply ); }
        },
        None => break,
      }

      position += 1;
    }

    loop {
      match additives.pop() {
        Some(dg) => {
          self.add(dg);
        },
        None => break,
      }
    }

    self.clone()
  }
}

/// NODOC
pub trait Into<String> {
  /// NODOC
  fn into(self) -> String;
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
