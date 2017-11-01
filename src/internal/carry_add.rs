#[allow(dead_code)]
#[derive(Clone,Copy)]
pub(crate) enum Sign {
  Plus,
  Minus,
}

#[derive(Clone,Copy)]
pub(crate) struct SignNum<T> {
  pub num: T,
  pub sign: Sign,
}

impl<T> SignNum<T>
  where T: Copy {
  pub fn new(n: T) -> Self {
    SignNum {
      num: n,
      sign: Sign::Plus
    }
  }
}

#[allow(dead_code)]
pub(crate) struct CarryResult<T> where T: Sized + PartialEq {
  pub sign_num: SignNum<T>,
  pub carry: Option<SignNum<T>>,
}

pub(crate) trait CappedAdd<T> where T: Sized + PartialEq {
  fn capped_add(&self, other: Self, cap: (T, T)) -> CarryResult<T>;
}

impl CappedAdd<u64> for SignNum<u64> {
  #[allow(unused_comparisons)]
  fn capped_add(&self, other: Self, cap: (u64, u64)) -> CarryResult<u64> {
    if !(cap.1 == 0 && cap.0 < 0) { // If not a nega-base (not yet implemented)
      assert!(cap.0 == 0 && cap.1 > 0);
    }
    match (self.sign, other.sign) {
      (Sign::Plus, Sign::Plus) => {
        let added = self.num + other.num;
        let carry = SignNum::new(added/(cap.1));
        let num = SignNum::new(added%(cap.1));
        CarryResult {
          sign_num: num,
          carry: {
            if carry.num == 0 { None } else { Some(carry) }
          }
        }
      },
      _ => unimplemented!(),
    }
  }
}

impl CappedAdd<u64> for u64 {
  fn capped_add(&self, other: Self, cap: (u64, u64)) -> CarryResult<u64> {
    SignNum::new(self.clone()).capped_add(SignNum::new(other), cap)
  }
}

#[test]
fn it_works() {
  let nine: SignNum<u64> = SignNum {
    num: 9,
    sign: Sign::Plus,
  };
  let bnine = nine.clone();
  let result = nine.capped_add(bnine, (0,10));
  assert_eq!(result.sign_num.num, 8);
  assert_eq!(result.carry.unwrap().num, 1);
}
