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
#![feature(slice_patterns)]
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
  /// Add two Digits instances together.
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
    let other = self.into_base(other);
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

  fn into_base(&self, other: Digits<'a>) -> Self {
    if self.mapping == other.mapping {
      other
    } else {
      self.gen(other)
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

  /// Multiply two Digits instances together.
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
  fn multiply(&self, other: Digits<'a>, power_of_ten: usize) -> Self {
    let other = self.into_base(other);

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

  /// Add two Digits instances together.
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
  fn mut_add_internal(&mut self, other: Digits<'a>, trim: bool) -> Self {
    let other = self.into_base(other);

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

  /// Multiply two Digits instances together.
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

  /// Create a Digits from a Vector of from zero positional mappings for custom Digits numeric
  /// base.
  /// 
  /// # Example
  ///
  /// ```
  /// use digits::{BaseCustom,Digits};
  ///
  /// let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  /// let builder = Digits::new(&base16, "".to_string());
  /// let num = builder.new_mapped(vec![1,0,2,1]).ok().unwrap();
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
  pub fn new_mapped(&self, places: Vec<usize>) -> Result<Self, &'static str> {
    if places.iter().any(|&x| x >= self.mapping.base as usize) {
      return Err("Character mapping out of range!");
    }
    let num = places.iter().fold("".to_string(), |mut acc, &x| {
        acc.push(self.mapping.nth(x).unwrap().clone());
        acc
      }
    );
    Ok(Digits::new(self.mapping, num))
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

  /// Right count of digits character index.
  ///
  /// Returns a `usize` of how many Digits values from the right
  /// match the BaseCustom index given for number.
  pub fn rcount(&self, character_index: u8) -> usize {
    if let Some(ref d) = self.left {
      if self.digit == character_index as u64 {
        return d.rcount(character_index) + 1;
      }
    } else{
      if self.digit == character_index as u64 {
        return 1;
      }
    }
    0
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

  /// Zero fills the left of the current number up to a total character length.
  pub fn zero_fill(&mut self, length: usize) {
    if self.length() >= length { return; }
    if length == 0 { return; }
    match self.left.clone() {
      None => {
        let mut l = self.zero();
        l.zero_fill(length - 1);
        self.left = Some(Box::new(l));
      }
      Some(v) => {
        self.set_left(
          {
            let mut l = v.replicate();
            l.zero_fill(length -1 );
            l
          },
          false
        )
      }
    }
  }

  /// Zero fills the left of the current number up to a total character length.
  pub fn zero_trim(&mut self) {
    let mut lnum: String = "".to_string();
    if let Some(ref v) = self.left {
      lnum = v.to_s();
    }
    lnum = lnum.trim_left_matches(self.mapping.zero().clone()).to_string();
    let lval = self.propagate(lnum);
    self.set_left(lval, true);
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

#[allow(missing_docs)]
#[derive(Debug)]
pub struct StepMap<'a> {
  digits: Digits<'a>,
  base_map: Vec<usize>,
  limit: u8,
}

impl<'a> StepMap<'a> {
  #[allow(missing_docs)]
  pub fn new(d: Digits<'a>, n: u8) -> Self {
    StepMap {
      digits: d,
      base_map: vec![],
      limit: n,
    }
  }
}

extern crate array_tool;
use array_tool::vec::Shift;

impl<'a> Iterator for StepMap<'a> {
  type Item = Digits<'a>;

  #[inline]
  fn next(&mut self) -> Option<Digits<'a>> {
    let mut next_map: Vec<usize>;
    match self.base_map.len() {
      0 => next_map = vec![1],
      1 => {
        match self.base_map.as_slice() {
          &[1] => next_map = vec![2],
          &[2] => next_map = vec![3],
          &[3] => next_map = vec![1,1],
          _ => unreachable!(),
        }
      },
      2 => {
        match self.base_map.as_slice() {
          &[1,1] => next_map = vec![2,1],
          &[2,1] => {
            if self.limit == 0 {
              next_map = vec![1,0,3]
            } else {
              next_map = vec![1,0,1]
            }
          },
          _ => unreachable!(),
        }
      },
      _ => {
        // if one then two
        if self.base_map[0] == 1 { 
          next_map = self.base_map[1..self.base_map.len()].to_vec();
          next_map.unshift(2);
        } else { // if two then tail and one
          next_map = self.base_map.clone();

          let end_zero_qty = |v: &Vec<usize>| {
            let mut count = 0;
            let mut i = v.iter().rev();
            loop {
              if i.next() == Some(&0) {
                count += 1                
              } else {
                break
              }
            }
            count
          };

          match self.base_map[self.base_map.len()-2..self.base_map.len()].to_vec().as_slice() {
            &[0,1] => {
              next_map.pop();
              if end_zero_qty(&next_map) < self.limit + 1 {
                next_map.push(0);
              }
              if end_zero_qty(&next_map) < self.limit + 1 {
                next_map.push(1);
              } else {
                next_map.push(3);
              }
            },
            &[0,3] => {
              next_map.pop();
              next_map.push(2);
              next_map.push(1);
            },
            &[2,1] => {
              // build tower of multiples of 20
              // but first max zeros before appending 20
              // then if zeros are max use 3; else 1 on end
              next_map.pop();
              next_map.pop();
              if end_zero_qty(&next_map) == self.limit + 1 {
                next_map.push(2);
                next_map.push(0);
              } else {
                next_map.push(0);
              }
              if self.limit == 0 {
                next_map.push(3);
              } else {
                next_map.push(1);
              }
            },
            _ => unreachable!(),
          }
          next_map.shift();
          next_map.unshift(1);
        }
      },
    }
    self.base_map = next_map;
    Some(self.digits.new_mapped(self.base_map.clone()).ok().unwrap())
  }
}
