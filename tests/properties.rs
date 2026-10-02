extern crate digits;

// Randomised checks of Digits against native integer arithmetic and a brute
// force model of non-adjacent stepping, across several numeric bases.
use digits::prelude::*;
use digits::Radix;

const ALPHABETS: &[&str] = &[
  "01",
  "012",
  "0123",
  "abcde",
  "01234567",
  "0123456789",
  "0123456789abcdef",
  "ZYXWVUTSRQPONMLKJIHGFEDCBA",
];

/// Small deterministic xorshift generator so the tests need no dependencies.
struct Rng(u64);

impl Rng {
  fn below(&mut self, n: u64) -> u64 {
    self.0 ^= self.0 << 13;
    self.0 ^= self.0 >> 7;
    self.0 ^= self.0 << 17;
    self.0 % n
  }
}

fn base_of(alphabet: &str) -> BaseCustom<char> {
  BaseCustom::<char>::new(alphabet.chars().collect())
}

fn value(text: &str, alphabet: &str) -> u128 {
  let base = alphabet.chars().count() as u128;
  text.chars().fold(0, |acc, c| {
    acc * base + alphabet.chars().position(|x| x == c).unwrap() as u128
  })
}

fn render(mut v: u128, alphabet: &str, width: usize) -> String {
  let chars: Vec<char> = alphabet.chars().collect();
  let base = chars.len() as u128;
  let mut out = Vec::new();
  while v > 0 {
    out.push(chars[(v % base) as usize]);
    v /= base;
  }
  while out.len() < width.max(1) {
    out.push(chars[0]);
  }
  out.iter().rev().collect()
}

fn random_number(rng: &mut Rng, alphabet: &str, max_len: u64) -> String {
  let chars: Vec<char> = alphabet.chars().collect();
  let len = 1 + rng.below(max_len);
  (0..len)
    .map(|_| chars[rng.below(chars.len() as u64) as usize])
    .collect()
}

fn runs_ok(text: &str, adjacent: usize) -> bool {
  let chars: Vec<char> = text.chars().collect();
  let mut run = 1;
  for pair in chars.windows(2) {
    run = if pair[0] == pair[1] { run + 1 } else { 1 };
    if run > adjacent + 1 {
      return false;
    }
  }
  true
}

#[test]
fn arithmetic_matches_integers() {
  let mut rng = Rng(0x2545_F491_4F6C_DD1D);
  for round in 0..3000 {
    let alphabet = ALPHABETS[round % ALPHABETS.len()];
    let (a, b) = (
      random_number(&mut rng, alphabet, 12),
      random_number(&mut rng, alphabet, 12),
    );
    let (va, vb) = (value(&a, alphabet), value(&b, alphabet));
    let x = Digits::new(base_of(alphabet), a.clone());
    let y = x.propagate(b.clone());
    let width = a.len().max(b.len());

    assert_eq!(x.to_s(), a);
    assert_eq!(
      x.add(y.clone()).to_s(),
      render(va + vb, alphabet, width),
      "{} + {}",
      a,
      b
    );
    assert_eq!(
      x.mul(y.clone()).to_s(),
      render(va * vb, alphabet, 1),
      "{} * {}",
      a,
      b
    );
    assert_eq!(x.partial_cmp(&y), Some(va.cmp(&vb)), "{} <=> {}", a, b);
    // `==` compares digits exactly, padding included
    assert_eq!(x == y, a == b);

    let mut up = x.clone();
    up.succ();
    assert_eq!(up.to_s(), render(va + 1, alphabet, a.len()));
    let mut down = x.clone();
    down.pred_till_zero();
    assert_eq!(down.to_s(), render(va.saturating_sub(1), alphabet, a.len()));

    let exponent = rng.below(4) as u32;
    if let Some(expected) = va.checked_pow(exponent) {
      let mut p = x.clone();
      // the exponent may be written in any base
      p.pow(Digits::from((radices_for(round), u64::from(exponent))));
      // Values only: padding of powers follows 1.1.1 (see tests/regressions.rs)
      assert_eq!(value(&p.to_s(), alphabet), expected, "{} ^ {}", a, exponent);
    }

    assert_eq!(x.decimal().to_s(), va.to_string());
    assert_eq!(x.hexl().to_s(), format!("{:x}", va));
    assert_eq!(x.binary().to_s(), format!("{:b}", va));
    let back = Digits::from((x.clone(), x.hex()));
    assert_eq!(back.partial_cmp(&x), Some(std::cmp::Ordering::Equal));
  }
}

fn radices_for(round: usize) -> BaseCustom<char> {
  match round % 3 {
    0 => digits::radices::binary_base(),
    1 => digits::radices::decimal_base(),
    _ => digits::radices::hex_base(),
  }
}

#[test]
fn non_adjacent_stepping_matches_brute_force() {
  let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
  for round in 0..800 {
    let alphabet = ALPHABETS[round % ALPHABETS.len()];
    // Short numbers keep the brute force model fast; carries, padding and
    // growth into an extra digit all still occur at this width.
    let start = random_number(&mut rng, alphabet, 4);
    let adjacent = rng.below(3) as usize;

    let mut stepped = Digits::new(base_of(alphabet), start.clone());
    let mut expected = start.clone();
    for _ in 0..12 {
      let mut candidate = value(&expected, alphabet) + 1;
      expected = loop {
        let text = render(candidate, alphabet, expected.len());
        if runs_ok(&text, adjacent) {
          break text;
        }
        candidate += 1;
      };
      stepped.next_non_adjacent(adjacent);
      assert_eq!(
        stepped.to_s(),
        expected,
        "from {} with adjacent {}",
        start,
        adjacent
      );
      assert!(stepped.is_valid_adjacent(adjacent));
    }

    let mut prepped = Digits::new(base_of(alphabet), start.clone());
    prepped.prep_non_adjacent(adjacent);
    if runs_ok(&start, adjacent) {
      assert_eq!(prepped.to_s(), start);
    } else {
      prepped.succ();
      assert!(prepped.is_valid_adjacent(adjacent));
      let mut from_start = Digits::new(base_of(alphabet), start.clone());
      assert_eq!(
        from_start.next_non_adjacent(adjacent).to_s(),
        prepped.to_s()
      );
    }
  }
}
