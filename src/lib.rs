// Copyright 2017 Daniel P. Clark & other digits Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// #![deny(missing_docs,trivial_casts,trivial_numeric_casts,
//         missing_debug_implementations, missing_copy_implementations,
//         unsafe_code,unstable_features,unused_import_braces,unused_qualifications)
// ]
extern crate base_custom;
use base_custom::BaseCustom;

#[derive(Debug,Clone)]
pub struct Digits<'a> {
  mapping: &'a BaseCustom<char>,
  digit: u64,
  left: Option<Box<Digits<'a>>>,
}

impl<'a> Digits<'a> {
  pub fn new<S>(mapping: &'a BaseCustom<char>, number: S) -> Digits<'a>
  where S: Into<String> {
    let number = number.into();
    if number.is_empty() { return Digits { mapping: mapping, digit: 0, left: None }; };
    let (last, rest) = Digits::last_rest(number);
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

  pub fn to_s(&self) -> String {
    let num = self.mapping.gen(self.digit);
    match &self.left {
      &None => format!("{}", num),
      &Some(ref bx) => format!("{}{}", bx.to_s(), num),
    }
  }

  pub fn to_string(&self) -> String {
    self.to_s()
  }

  fn last_rest(s: String) -> (char, String) {
    let mut n = s.chars().rev();
    let last = n.next().unwrap();
    let rest = n.rev().collect::<String>();
    (last, rest)
  }

  fn set_left(&mut self, d: Digits<'a>) {
    if d.is_end() {
      self.left = None;
    } else {
      self.left = Some(Box::new(d));
    }
  }

  pub fn zero(&self) -> Self {
    Digits { mapping: self.mapping, digit: 0, left: None }
  }

  fn is_end(&self) -> bool {
    self.digit == 0 && match self.left { None => true, _ => false }
  }

  pub fn add(&mut self, other: Self) -> Self {
    if other.is_end() { return self.clone(); };
    let (last, rest) = Digits::last_rest(other.to_s());

    // sums current single digit
    let added = self.mapping.gen(self.mapping.decimal(last.to_string()) + self.digit);
    let (l, r) = Digits::last_rest(added);
    self.digit = self.mapping.decimal(l.to_string());

    // sums for left
    let mut intermediate = Digits::new(self.mapping, r);
    intermediate.add(Digits::new(self.mapping, rest));

    let current_left = match self.left.clone() {
      None => { Box::new(self.zero()) },
      Some(bx) => { bx },
    }.as_ref().clone();

    self.set_left( intermediate.add(current_left).clone() );
    self.clone()
  }
}

pub trait Into<String> {
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

