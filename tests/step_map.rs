extern crate digits;
use digits::{BaseCustom,Digits,StepMap};

#[test]
fn it_produces_correct_steps_for_zero_neighbors_in_base4(){
  let base4 = BaseCustom::<char>::new("0123".chars().collect());
  let num = Digits::new(&base4, "".to_string());
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
  let num = Digits::new(&base4, "".to_string());
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
  let num = Digits::new(&base4, "".to_string());
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
