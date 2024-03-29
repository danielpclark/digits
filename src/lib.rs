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
//! The digits crate is a linked list implementation of a score card flipper.  But
//! in this case it's with any characters you want and you can enumerate through
//! possibilities beyond the numeric limits intrinsic in basic numerc types like `u64`.
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

extern crate array_tool;
mod internal;
use internal::step_map::StepMap;
use internal::carry_add::{CappedAdd,SignNum,Sign};

/// This struct acts similar to a full number with a custom numeric character base
/// which is provided and mapped via a `BaseCustom` instance.
///
/// The underlying implementation for Digits is a linked list where all the methods recurse
/// as far as need to to implement the operations.
#[derive(Clone)]
pub struct Digits {
  mapping: BaseCustom<char>,
  digit: u64,
  left: Option<Box<Digits>>,
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
    assert!(self.base() == other.base());
    let mut result: Vec<u64> = vec![];

    let mut carry: u64 = 0;

    let mut c_self: Option<Box<Digits>> = Some(Box::new(self.clone()));
    let mut o_self: Option<Box<Digits>> = Some(Box::new(other));
    
    let mut remainder: Option<SignNum<u64>>;
    loop {
      if carry == 0 && c_self == None && o_self == None { break }
      let cs = c_self.clone();
      let os = o_self.clone();
      result.push(
        {
          let cr = carry.capped_add({
            cs.unwrap_or_else(|| Box::new(self.zero())).digit +
            os.unwrap_or_else(|| Box::new(self.zero())).digit},
            (0, self.base() as u64)
          );
          remainder = cr.carry;
          cr.sign_num.num
        }
      );
      let guard = remainder.unwrap_or_else(|| SignNum::new(0));
      match guard.sign {
        Sign::Plus => {
          carry = guard.num;
        },
        Sign::Minus => unimplemented!()
      }
      c_self = c_self.unwrap_or_else(|| Box::new(self.zero())).left;
      o_self = o_self.unwrap_or_else(|| Box::new(self.zero())).left;
    }
    result.reverse();
    self.new_mapped(&result).unwrap()
  }

  /// Returns a vector of each characters position mapping
  pub fn as_mapping_vec(&self) -> Vec<u64> {
    match self.left {
      Some(ref l) => {
        let mut result = l.as_mapping_vec();
        result.extend(vec![self.digit]);
        result
      },
      None => vec![self.digit],
    }
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
    Digits::from((self.mapping.clone(), other))
  }

  // the way to recurse and process Digits
  fn head_tail(self) -> (u64, Option<Box<Self>>) {
    match self.left {
      Some(bx) => (self.digit, Some(bx)),
      None => (self.digit, None),
    }
  }

  /// Returns true of false based on whether the limit of allowed adjacents is not exceeded.
  /// Early termination result when false.
  ///
  /// Same as being a more efficient `self.max_adjacent <= allowed_adjacent`.
  pub fn is_valid_adjacent(&self, adjacent: usize) -> bool {
    let mut ptr = self;
    let mut last_num = self.digit;
    let mut last_num_count = 0;
    while let Some(ref item) = ptr.left {
      if item.digit == last_num {
        last_num_count += 1;
      } else {
        last_num_count = 0;
      }

      if last_num_count > adjacent { return false; }
      last_num = item.digit;
      ptr = item;
    }
    true
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
    match self.left {
      None => { true },
      Some(ref bx) => { bx.is_zero() },
    }
  }

  /// Returns bool value of if the number is zero.
  pub fn is_zero(&self) -> bool {
    if self.digit != 0 { return false }
    match self.left {
      None => { true },
      Some(ref bx) => { bx.is_zero() },
    }
  }

  /// Returns a `usize` of the total linked list length.
  pub fn length(&self) -> usize {
    match self.left {
      None => 1,
      Some(ref l) => { l.length() + 1 }
    }
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
    self.max_adj(self.digit, 0, 1) - 1
  }

  fn max_adj(&self, last_num: u64, last_num_count: usize, max_count: usize) -> usize {
    let mut lnc = last_num_count;
    if self.digit == last_num { lnc += 1; } else { lnc = 1; }
    let max_count = std::cmp::max(lnc, max_count);
    match self.left {
      None => max_count,
      Some(ref l) => { l.max_adj(self.digit, lnc, max_count) },
    }
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
    self.multiply(other, 0)
  }

  // Internal implementation for multiply. Needs the recursive
  // value of powers of ten for addition.
  fn multiply(&self, other: Digits, power_of_ten: usize) -> Self {
    assert!(self.base() == other.base());

    let mut position: usize = power_of_ten;
    let mut o = Some(Box::new(other));
    let mut result = self.zero();

    while let Some(thing) = o.clone() {
      let (dgt, tail) = thing.head_tail();
      o = tail;

      let mltply = self.propagate((self.digit * dgt).to_string()).pow_ten(position);

      let current_digit = self.propagate(self.mapping.gen(dgt).to_string());

      if let Some(ref bx) = self.left {
        result.mut_add_internal(bx.clone().multiply(current_digit, position + 1), true);
      };

      result.mut_add_internal( mltply, true );
      position += 1;
    }

    result
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
    self.mut_add_internal(other, false)
  }
  fn mut_add_internal(&mut self, other: Digits, trim: bool) -> Self {
    assert!(self.base() == other.base());

    if other.is_end() { return self.clone(); };
    let (last, rest) = other.head_tail();

    // sums current single digit
    let added = self.propagate(self.mapping.gen(last + self.digit));
    let (l, r) = added.head_tail();
    self.digit = l;

    // sums for left
    let mut intermediate = Digits::new_zero(self.mapping.clone());
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
    let (d, r) = self.multiply(other, 0).head_tail();
    self.digit = d;
    if let Some(rest) = r { self.set_left(rest.replicate(), true); }
    self.clone()
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
        Some(Box::new(Digits::new(mapping.clone(), rest)))
      }
    };
    Digits {
      mapping: mapping.clone(),
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
    let num = places.iter().fold("".to_string(), |mut acc, &x| {
        acc.push(*self.mapping.nth(x as usize).unwrap());
        acc
      }
    );
    Ok(Digits::new(self.mapping.clone(), num))
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
    Digits { mapping: mapping, digit: 1, left: None }
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
    Digits { mapping: mapping, digit: 0, left: None }
  }

  /// Returns the next Digits in incrementing that only allows the given number of
  /// adjacent number duplicates.
  ///
  /// _This will panic! if numeric base is less than 4._
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
    self.prep_non_adjacent(adjacent);
    self.step_non_adjacent(adjacent)
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
    Digits::new_one(self.mapping.clone())
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
  pub fn pow(&mut self, mut pwr: Self) -> Self {
    if pwr.is_zero() { return self.one(); }
    let copy = self.clone();
    loop {
      if pwr.is_one() {
        break
      } else {
        self.mut_mul(copy.clone());
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
      result = Digits::new_zero(self.mapping.clone());
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

  /// Sometimes given starting Digits have more adjacent characters than is desired
  /// when proceeding with non-adjacent steps.  This method provides a valid initial
  /// state for `step_non_adjacent`'s algorithm to not miss any initial steps.
  /// 
  /// _This method is used internally for `next_non_adjacent`.
  ///
  /// _This will panic! if numeric base is less than 4._
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
    assert!(self.mapping.base > 3, "\n\n  WARNING!\n\n  \"You may not use non-adjacent stepping with numeric bases of less than 4!\"\n\n");

    if self.is_valid_adjacent(adjacent) {
      return self.clone();
    }

    let mut v = self.as_mapping_vec();
    'outer: loop {
      let mut last_num: Option<u64> = None;
      let mut last_num_count = 0;
      let w = v.clone();
      let itr = w.iter().enumerate();

      for (i, item) in itr {
        if last_num == None {
          last_num = Some(*item);
          continue;
        }

        if let Some(val) = last_num {
          if item == &val {
            last_num_count += 1;
          } else {
            last_num_count = 0;
          }

          if last_num_count > adjacent {
            let i = i + 1;
            let mut d = self.new_mapped(&v[0..i].to_vec()).ok().unwrap();
            d.succ();
            let mut new_v = d.as_mapping_vec();

            for _ in v[i..v.len()].iter() {
              new_v.push(0)
            }

            v = new_v;
            continue 'outer;
          }
        }
        
        last_num = Some(*item);
      }
      break;
    }
    let result = self.new_mapped(&v).ok().unwrap().pred_till_zero();
    self.digit = result.digit;
    self.left = result.left.clone();
    result
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
    Digits::new(self.mapping.clone(), number)
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
    if let Some(ref d) = self.left {
      if self.digit == u64::from(character_index) {
        return d.rcount(character_index) + 1;
      }
    } else if self.digit == u64::from(character_index) {
      return 1;
    }
    0
  }

  /// An alias for `clone`. _Useful for unboxing._
  pub fn replicate(self) -> Self { self.clone() }

  // logic for setting left linked list continuation
  fn set_left(&mut self, d: Digits, trim: bool) {
    if trim && d.is_end() {
      self.left = None;
    } else {
      self.left = Some(Box::new(d));
    }
  }

  /// Returns the next Digits in incrementing that only allows the given number of
  /// adjacent number duplicates.
  ///
  /// _This will panic! if numeric base is less than 4._
  ///
  /// **NOTE:** _This assumes the starting state is valid for given non adjacent characters.
  /// If you want to ensure this please use prep_adjacent before this, or just use
  /// `next_non_adjacent` to handle them both._
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
    let mut step_map = StepMap::new(self.zero(), adjacent as u8);
    let mut v: Self;
    loop {
      let mut builder = self.clone();
      v = builder.mut_add(step_map.next().unwrap());
      if v.is_valid_adjacent(adjacent) {
        break;
      }
    }
    self.digit = v.digit;
    self.left = v.left;
    self.clone()
  }

  /// Plus one.
  pub fn succ(&mut self) -> Self {
    let one = self.one();
    self.mut_add_internal(one, false)
  }

  /// Gives the full value of all digits within the linked list as a String.
  pub fn to_s(&self) -> String {
    let num = self.mapping.gen(self.digit);
    match self.left {
      None => num.to_owned(),
      Some(ref bx) => format!("{}{}", bx.to_s(), num),
    }
  }

  /// Gives the full value of all digits within the linked list as a String.
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
    Digits::new_zero(self.mapping.clone())
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
    let mut lnum: String = "".to_string();
    if let Some(ref v) = self.left {
      lnum = v.to_s();
    }
    lnum = lnum.trim_start_matches(*self.mapping.zero()).to_string();
    let lval = self.propagate(lnum);
    self.set_left(lval, true);
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
    let mut curr_node: Option<Digits> = Some(self.clone());
    let mut prev_node: Option<Digits> = None;

    while curr_node != None {
      let cn = curr_node.unwrap();
      let next_node = if let Some(n) = cn.clone().left {
        Some(n.replicate())
      } else { None };
      let mut c = cn;
      if let Some(prev) = prev_node {
        c.set_left(prev, false);
      } else {
        c.left = None;
      }
      prev_node = Some(c);
      curr_node = next_node;
    }

    let p = prev_node.unwrap();
    self.digit = p.digit;
    self.left = p.left;
  }
}

#[allow(missing_docs)]
pub trait Into<String> {
  fn into(self) -> String;
}

impl From<(BaseCustom<char>, u64)> for Digits {
  fn from(d: (BaseCustom<char>, u64)) -> Digits {
    let mapping = d.0;
    let value = d.1;
    Digits::new(mapping.clone(), mapping.gen(value))
  }
}

impl From<(BaseCustom<char>, Digits)> for Digits {
  fn from(d: (BaseCustom<char>, Digits)) -> Digits {
    let mapping = d.0;
    let source = d.1;
    let from_base = source.mapping.base;
    let mut result = Digits::new_zero(mapping.clone());
    let mut pointer: Option<Box<Digits>> = Some(Box::new(source.clone()));
    let mut position = 0;
    // Down-Casting
    if from_base >= mapping.base {
      while let Some(bx) = pointer {
        let (h, t) = bx.head_tail();
        if h != 0 { // speed optimization
          result.mut_add_internal(
            Digits::new(mapping.clone(), mapping.gen(h)).mul(
              Digits::new(mapping.clone(), mapping.gen(from_base)).
                pow(source.gen(position))
            ),
            true
          );
        }
        position += 1;
        pointer = t;
      }
    } else { // Up-Casting
      while let Some(bx) = pointer {
        let (h, t) = bx.head_tail();
        if h != 0 { // speed optimization
          result.mut_add_internal(
            // This implementation is limited by the max of usize
            Digits::new(mapping.clone(), mapping.gen(h * from_base.pow(position as u32))),
            true
          );
        }
        position += 1;
        pointer = t;
      }
    }
    result
  }
}

impl From<(Digits, Digits)> for Digits {
  fn from(d: (Digits, Digits)) -> Digits {
    if d.0.base() == d.1.base() { return d.1; }
    Digits::from((d.0.mapping, d.1))
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
     self.mapping.gen(self.digit), self.digit, {
       match self.left {
         None => "".to_string(),
         Some(ref l) => format!(", With Preceeding: '{}'", l.to_s()),
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
    self.mapping == other.mapping &&
      self.digit == other.digit &&
      self.left == other.left
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
    self.mut_add(other);
  }
}

impl Mul for Digits {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    self.multiply(other, 0)
  }
}

impl MulAssign for Digits {
  fn mul_assign(&mut self, other: Self) {
    self.mut_mul(other);
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
    self.pow(other);
  }
}

impl PartialOrd for Digits {
  fn partial_cmp(&self, other: &Digits) -> Option<Ordering> {
    assert!(self.mapping == other.mapping);
    let mut result: Option<Ordering>;
    let mut a: Self = self.clone();
    let mut b: Self = other.clone();
    result = a.digit.partial_cmp(&b.digit);
    while let (Some(x),Some(y)) = (a.left.clone(), b.left.clone()) {
      a = x.replicate();
      b = y.replicate();
      match a.digit.partial_cmp(&b.digit) {
        Some(Ordering::Equal) | None => (),
        Some(change) => { result = Some(change); },
      }
    }
    if a.left.is_some() && !b.left.is_some() && !a.left.clone().unwrap().is_zero() {
      result = Some(Ordering::Greater);
    }
    if !a.left.is_some() && b.left.is_some() && !b.left.unwrap().is_zero() {
      result = Some(Ordering::Less);
    }
    result
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
