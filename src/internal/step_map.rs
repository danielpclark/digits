#[allow(unused_imports)]
use ::{BaseCustom,Digits};

#[allow(missing_docs)]
#[derive(Debug)]
pub(crate) struct StepMap {
  digits: Digits,
  base_map: Vec<u64>,
  limit: u8,
}

impl StepMap {
  #[allow(missing_docs)]
  pub fn new(d: Digits, n: u8) -> Self {
    assert!(d.mapping.base > 3, "\n\n  WARNING!\n\n  \"You may not use non-adjacent stepping with numeric bases of less than 4!\"\n\n");
    StepMap {
      digits: d,
      base_map: vec![],
      limit: n,
    }
  }
}

use array_tool::vec::Shift;

impl Iterator for StepMap {
  type Item = Digits;

  #[inline]
  fn next(&mut self) -> Option<Digits> {
    let mut next_map: Vec<u64>;
    match self.base_map.len() {
      0 => next_map = vec![1],
      1 => {
        match self.base_map[0] {
          1 => next_map = vec![2],
          2 => next_map = vec![3],
          3 => next_map = vec![1,1],
          _ => unreachable!(),
        }
      },
      2 => {
        match (
            self.base_map[0],
            self.base_map[1]
          ) {
          (1,1) => next_map = vec![2,1],
          (2,1) => {
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

          let end_zero_qty = |v: &Vec<u64>| {
            let mut count = 0;
            let mut i = v.iter().rev();
            while let Some(&0) = i.next() {
              count += 1; 
            }
            count
          };

          match (
              self.base_map[self.base_map.len()-2],
              self.base_map[self.base_map.len()-1]
            ) {
            (0,1) => {
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
            (0,3) => {
              next_map.pop();
              next_map.push(2);
              next_map.push(1);
            },
            (2,1) => {
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

#[test]
fn it_produces_correct_steps_for_zero_neighbors_in_base4(){
  let base4 = BaseCustom::<char>::new("0123".chars().collect());
  let num = Digits::new(base4, "".to_string());
  let mut step_iter = StepMap::new(num, 0);
  assert_eq!(step_iter.next().unwrap().to_s(), "1".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "3".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "11".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "21".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "103".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "102021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "102020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1020202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2020202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "102020202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "202020202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1020202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2020202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10202020202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20202020202021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "102020202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "202020202020203".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1020202020202021".to_string());
}

#[test]
fn it_produces_correct_steps_for_one_neighbor_in_base4(){
  let base4 = BaseCustom::<char>::new("0123".chars().collect());
  let num = Digits::new(base4, "".to_string());
  let mut step_iter = StepMap::new(num, 1);
  assert_eq!(step_iter.next().unwrap().to_s(), "1".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "3".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "11".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "21".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "101".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "100201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "200201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1002003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2002003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10020021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20020021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "100200201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "200200201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1002002003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2002002003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10020020021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20020020021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "100200200201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "200200200201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1002002002003".to_string());
}

#[test]
fn it_produces_correct_steps_for_two_neighbors_in_base4(){
  let base4 = BaseCustom::<char>::new("0123".chars().collect());
  let num = Digits::new(base4, "".to_string());
  let mut step_iter = StepMap::new(num, 2);
  assert_eq!(step_iter.next().unwrap().to_s(), "1".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "3".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "11".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "21".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "101".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1001".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2001".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "100021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "200021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1000201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2000201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10002001".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20002001".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "100020003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "200020003".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "1000200021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "2000200021".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "10002000201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "20002000201".to_string());
  assert_eq!(step_iter.next().unwrap().to_s(), "100020002001".to_string());
}
