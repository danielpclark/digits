//! digits 1.1.1 (crates.io) against this checkout, on identical workloads.
//!
//! Each group benchmarks one operation at several sizes.  Sizes are capped
//! where 1.1.1 takes about a second per iteration or stops producing correct
//! results (base conversion overflows past 19 decimal digits).  The `large`
//! group runs 2.0.0 alone at sizes 1.1.1 cannot reach in reasonable time.
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;
use std::time::Duration;

const V1: &str = "1.1.1";
const V2: &str = "2.0.0";

fn base10_v1() -> digits_v1::BaseCustom<char> {
  digits_v1::BaseCustom::<char>::new("0123456789".chars().collect())
}

fn base10_v2() -> digits::BaseCustom<char> {
  digits::BaseCustom::<char>::new("0123456789".chars().collect())
}

fn v1(text: &str) -> digits_v1::Digits {
  digits_v1::Digits::new(base10_v1(), text.to_string())
}

fn v2(text: &str) -> digits::Digits {
  digits::Digits::new(base10_v2(), text)
}

/// A number of `n` digits that varies but never starts with zero.
fn number(n: usize) -> String {
  (0..n)
    .map(|i| char::from(b'1' + (i * 7 % 9) as u8))
    .collect()
}

fn construct(c: &mut Criterion) {
  let mut group = c.benchmark_group("construct");
  for &n in &[100usize, 1_000, 4_000] {
    let text = number(n);
    group.bench_with_input(BenchmarkId::new(V1, n), &text, |b, text| {
      b.iter_batched(
        || (base10_v1(), text.clone()),
        |(base, text)| digits_v1::Digits::new(base, text),
        BatchSize::SmallInput,
      )
    });
    group.bench_with_input(BenchmarkId::new(V2, n), &text, |b, text| {
      b.iter_batched(
        || (base10_v2(), text.clone()),
        |(base, text)| digits::Digits::new(base, text),
        BatchSize::SmallInput,
      )
    });
  }
  group.finish();
}

fn to_s(c: &mut Criterion) {
  let mut group = c.benchmark_group("to_s");
  for &n in &[100usize, 1_000, 4_000] {
    let (a, b2) = (v1(&number(n)), v2(&number(n)));
    group.bench_function(BenchmarkId::new(V1, n), |b| b.iter(|| black_box(&a).to_s()));
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter(|| black_box(&b2).to_s())
    });
  }
  group.finish();
}

/// 1,000 consecutive `succ` calls from an all-zero counter of the given width.
fn count_1000(c: &mut Criterion) {
  let mut group = c.benchmark_group("count_1000");
  for &n in &[8usize, 64, 1_000] {
    let zeros = "0".repeat(n);
    let (start1, start2) = (v1(&zeros), v2(&zeros));
    group.bench_function(BenchmarkId::new(V1, n), |b| {
      b.iter_batched(
        || start1.clone(),
        |mut x| {
          for _ in 0..1_000 {
            black_box(x.succ());
          }
          x
        },
        BatchSize::SmallInput,
      )
    });
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter_batched(
        || start2.clone(),
        |mut x| {
          for _ in 0..1_000 {
            black_box(x.succ());
          }
          x
        },
        BatchSize::SmallInput,
      )
    });
  }
  group.finish();
}

/// One `succ` on a number of all nines, so the carry runs the full width.
fn succ_full_carry(c: &mut Criterion) {
  let mut group = c.benchmark_group("succ_full_carry");
  group.sample_size(10);
  for &n in &[10usize, 100, 1_000] {
    let nines = "9".repeat(n);
    let (start1, start2) = (v1(&nines), v2(&nines));
    group.bench_function(BenchmarkId::new(V1, n), |b| {
      b.iter_batched(
        || start1.clone(),
        |mut x| {
          x.succ();
          x
        },
        BatchSize::SmallInput,
      )
    });
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter_batched(
        || start2.clone(),
        |mut x| {
          x.succ();
          x
        },
        BatchSize::SmallInput,
      )
    });
  }
  group.finish();
}

fn add(c: &mut Criterion) {
  let mut group = c.benchmark_group("add");
  for &n in &[10usize, 100, 1_000] {
    let (a1, b1) = (v1(&number(n)), v1(&"9".repeat(n)));
    let (a2, b2) = (v2(&number(n)), v2(&"9".repeat(n)));
    group.bench_function(BenchmarkId::new(V1, n), |b| {
      b.iter_batched(|| b1.clone(), |other| a1.add(other), BatchSize::SmallInput)
    });
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter(|| a2.add(black_box(&b2)))
    });
  }
  group.finish();
}

fn mul(c: &mut Criterion) {
  let mut group = c.benchmark_group("mul");
  group.sample_size(10);
  for &n in &[5usize, 10, 20] {
    let (a1, b1) = (v1(&number(n)), v1(&"7".repeat(n)));
    let (a2, b2) = (v2(&number(n)), v2(&"7".repeat(n)));
    group.bench_function(BenchmarkId::new(V1, n), |b| {
      b.iter_batched(|| b1.clone(), |other| a1.mul(other), BatchSize::SmallInput)
    });
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter(|| a2.mul(black_box(&b2)))
    });
  }
  group.finish();
}

/// 2 raised to the given exponent.
fn pow(c: &mut Criterion) {
  let mut group = c.benchmark_group("pow_2_to_the");
  group.sample_size(10);
  for &e in &[25u64, 50, 100] {
    let (two1, exp1) = (v1("2"), v1(&e.to_string()));
    let (two2, exp2) = (v2("2"), v2(&e.to_string()));
    group.bench_function(BenchmarkId::new(V1, e), |b| {
      b.iter_batched(
        || (two1.clone(), exp1.clone()),
        |(mut x, exponent)| x.pow(exponent),
        BatchSize::SmallInput,
      )
    });
    group.bench_function(BenchmarkId::new(V2, e), |b| {
      b.iter_batched(
        || two2.clone(),
        |mut x| {
          x.pow(&exp2);
          x
        },
        BatchSize::SmallInput,
      )
    });
  }
  group.finish();
}

fn compare(c: &mut Criterion) {
  let mut group = c.benchmark_group("partial_cmp");
  for &n in &[10usize, 100, 1_000] {
    // Equal except in the least significant digit: the worst case for 2.0.0,
    // which compares from the most significant end.
    let head = number(n - 1);
    let (a1, b1) = (v1(&format!("{}1", head)), v1(&format!("{}2", head)));
    let (a2, b2) = (v2(&format!("{}1", head)), v2(&format!("{}2", head)));
    group.bench_function(BenchmarkId::new(V1, n), |b| {
      b.iter(|| black_box(&a1).partial_cmp(&b1))
    });
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter(|| black_box(&a2).partial_cmp(&b2))
    });
  }
  group.finish();
}

/// Decimal to hex.  1.1.1 overflows above 19 decimal digits, so sizes stop at 18.
fn decimal_to_hex(c: &mut Criterion) {
  use digits::Radix as _;
  use digits_v1::Radix as _;
  let mut group = c.benchmark_group("decimal_to_hex");
  for &n in &[6usize, 12, 18] {
    let (a1, a2) = (v1(&number(n)), v2(&number(n)));
    assert_eq!(
      a1.hex().to_s(),
      a2.hex().to_s(),
      "versions disagree at {} digits",
      n
    );
    group.bench_function(BenchmarkId::new(V1, n), |b| b.iter(|| black_box(&a1).hex()));
    group.bench_function(BenchmarkId::new(V2, n), |b| b.iter(|| black_box(&a2).hex()));
  }
  group.finish();
}

/// Hex to decimal (1.1.1 takes its slower "down-casting" path here).
fn hex_to_decimal(c: &mut Criterion) {
  use digits::Radix as _;
  use digits_v1::Radix as _;
  let mut group = c.benchmark_group("hex_to_decimal");
  group.sample_size(10);
  for &n in &[4usize, 8, 16] {
    let text = number(n);
    let a1 = digits_v1::Digits::new(digits_v1::radices::hex_base(), text.clone());
    let a2 = digits::Digits::new(digits::radices::hex_base(), text.as_str());
    assert_eq!(a1.decimal().to_s(), a2.decimal().to_s());
    group.bench_function(BenchmarkId::new(V1, n), |b| {
      b.iter(|| black_box(&a1).decimal())
    });
    group.bench_function(BenchmarkId::new(V2, n), |b| {
      b.iter(|| black_box(&a2).decimal())
    });
  }
  group.finish();
}

/// 1,000 consecutive `next_non_adjacent` steps from "00000000".
fn non_adjacent_1000(c: &mut Criterion) {
  let mut group = c.benchmark_group("next_non_adjacent_1000");
  group.sample_size(10);
  for &adjacent in &[0usize, 1] {
    let (start1, start2) = (v1("00000000"), v2("00000000"));
    let (mut end1, mut end2) = (start1.clone(), start2.clone());
    for _ in 0..1_000 {
      end1.next_non_adjacent(adjacent);
      end2.next_non_adjacent(adjacent);
    }
    assert_eq!(end1.to_s(), end2.to_s());
    group.bench_function(BenchmarkId::new(V1, adjacent), |b| {
      b.iter_batched(
        || start1.clone(),
        |mut x| {
          for _ in 0..1_000 {
            black_box(x.next_non_adjacent(adjacent));
          }
          x
        },
        BatchSize::SmallInput,
      )
    });
    group.bench_function(BenchmarkId::new(V2, adjacent), |b| {
      b.iter_batched(
        || start2.clone(),
        |mut x| {
          for _ in 0..1_000 {
            black_box(x.next_non_adjacent(adjacent));
          }
          x
        },
        BatchSize::SmallInput,
      )
    });
  }
  group.finish();
}

/// 2.0.0 alone, at sizes the 1.1.1 implementation cannot reach in reasonable time.
fn large(c: &mut Criterion) {
  use digits::Radix as _;
  let mut group = c.benchmark_group("large");
  group.sample_size(10);

  let nines = v2(&"9".repeat(100_000));
  group.bench_function(BenchmarkId::new(V2, "succ_full_carry/100000"), |b| {
    b.iter_batched(
      || nines.clone(),
      |mut x| {
        x.succ();
        x
      },
      BatchSize::LargeInput,
    )
  });

  let (a, m) = (v2(&number(2_000)), v2(&"7".repeat(2_000)));
  group.bench_function(BenchmarkId::new(V2, "mul/2000"), |b| b.iter(|| a.mul(&m)));

  let exponent = v2("100000");
  group.bench_function(BenchmarkId::new(V2, "pow_2_to_the/100000"), |b| {
    b.iter_batched(
      || v2("2"),
      |mut x| {
        x.pow(&exponent);
        x
      },
      BatchSize::SmallInput,
    )
  });

  let wide = v2(&number(10_000));
  group.bench_function(BenchmarkId::new(V2, "decimal_to_hex/10000"), |b| {
    b.iter(|| wide.hex())
  });

  let text = number(1_000_000);
  group.bench_function(BenchmarkId::new(V2, "construct/1000000"), |b| {
    b.iter(|| v2(black_box(&text)))
  });
  group.finish();
}

fn config() -> Criterion {
  Criterion::default()
    .warm_up_time(Duration::from_secs(1))
    .measurement_time(Duration::from_secs(3))
}

criterion_group! {
  name = benches;
  config = config();
  targets = construct, to_s, count_1000, succ_full_carry, add, mul, pow, compare,
    decimal_to_hex, hex_to_decimal, non_adjacent_1000, large
}
criterion_main!(benches);
