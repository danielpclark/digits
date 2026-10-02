// Copyright 2017 Daniel P. Clark & other digits Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Operator overloads.  Each operator accepts owned and borrowed operands, so
//! `&a + &b` works without cloning either side.

use crate::Digits;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign};

macro_rules! digits_operator {
  ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident, $in_place:ident) => {
    impl $Op<Digits> for Digits {
      type Output = Digits;
      fn $op(mut self, other: Digits) -> Digits {
        self.$in_place(other);
        self
      }
    }

    impl<'a> $Op<&'a Digits> for Digits {
      type Output = Digits;
      fn $op(mut self, other: &'a Digits) -> Digits {
        self.$in_place(other);
        self
      }
    }

    impl<'a> $Op<Digits> for &'a Digits {
      type Output = Digits;
      fn $op(self, other: Digits) -> Digits {
        let mut result = self.clone();
        result.$in_place(other);
        result
      }
    }

    impl<'a, 'b> $Op<&'b Digits> for &'a Digits {
      type Output = Digits;
      fn $op(self, other: &'b Digits) -> Digits {
        let mut result = self.clone();
        result.$in_place(other);
        result
      }
    }

    impl $OpAssign<Digits> for Digits {
      fn $op_assign(&mut self, other: Digits) {
        self.$in_place(other);
      }
    }

    impl<'a> $OpAssign<&'a Digits> for Digits {
      fn $op_assign(&mut self, other: &'a Digits) {
        self.$in_place(other);
      }
    }
  };
}

digits_operator!(Add, add, AddAssign, add_assign, mut_add);
digits_operator!(Mul, mul, MulAssign, mul_assign, mut_mul);
// `^` raises to a power, mirroring the `pow` method.
digits_operator!(BitXor, bitxor, BitXorAssign, bitxor_assign, pow);
