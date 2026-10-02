# Changelog

## 2.0.0

Version 2 rewrites the internals of `Digits`.  A number is now a vector of digits plus
a shared (`Arc`) character mapping instead of a linked list holding a full copy of the
mapping in every digit.  Nothing recurses per digit any more, so numbers are limited by
memory alone, and the common operations are much faster.

### Fixed

* Multiplication was wrong in every base other than base 10 written with `0-9`: hex
  `f * f` gave `225` instead of `e1`, and bases using other characters panicked.
* `mut_mul` (and `*=`) by zero left part of the old value behind: `12 *= 0` gave `10`.
* `pow` (and `^=`) with an exponent of zero did not change the number.
* Converting to a larger base overflowed `u64`: a 65 digit binary number converted to
  decimal gave `0` in release builds and panicked in debug builds.
* Converting between two mappings of the same size (e.g. upper to lower case hex) returned
  the number unchanged in the original mapping.
* `==` treated `"009"` and `"9"` as different while `partial_cmp` said they were equal.
* Adjacency limits above 255 were truncated to a `u8`, so `next_non_adjacent(256)` acted
  like `next_non_adjacent(0)`.
* Numbers of a few thousand digits overflowed the stack when created, compared, printed
  or dropped.

### Performance

Criterion medians on identical inputs, 1.1.1 (from crates.io) against 2.0.0, from
[`benchmarks/`](benchmarks) on an Intel Xeon @ 2.10GHz with rustc 1.97.  Full tables with
confidence intervals and smaller sizes are in [`benchmarks/RESULTS.md`](benchmarks/RESULTS.md).

| Operation | 1.1.1 | 2.0.0 | Speedup |
|---|---:|---:|---:|
| `succ` on 1,000 nines (full carry) | 1.14 s | 700 ns | 1,600,000× |
| 1,000 `succ` calls on an 8-digit counter | 2.73 ms | 2.12 µs | 1,300× |
| `add` of two 1,000-digit numbers | 134 ms | 1.51 µs | 88,000× |
| `mul` of two 20-digit numbers | 298 ms | 417 ns | 715,000× |
| `pow`: 2^100 | 300 ms | 1.23 µs | 243,000× |
| `partial_cmp` of 1,000-digit numbers (differing in the last digit) | 321 ms | 295 ns | 1,090,000× |
| `Digits::new` from 4,000 characters | 7.05 ms | 33.5 µs | 210× |
| 18-digit decimal → hex | 877 µs | 820 ns | 1,070× |
| 1,000 `next_non_adjacent(0)` steps from `00000000` | 7.00 ms | 12.7 µs | 550× |

Sizes 1.1.1 cannot reach (it overflows converting more than 19 decimal digits, and the
operations above grow far faster than linearly), measured on 2.0.0 alone: `succ` on 100,000
nines 42.9 µs, 2,000 × 2,000-digit `mul` 1.5 ms, 2^100000 112 ms, 10,000-digit decimal → hex
25.9 ms, `Digits::new` from 1,000,000 characters 8.0 ms.

### Changed (breaking)

* `BaseCustom` comes from `base_custom` 0.2.
* The crate's own `Into<String>` trait is gone; `new` and `propagate` take the standard
  `Into<String>`, so string slices work: `Digits::new(base, "42")`.
* Methods that change a number in place (`succ`, `pred_till_zero`, `mut_add`, `mut_mul`,
  `pow`, `next_non_adjacent`, `prep_non_adjacent`, `step_non_adjacent`) return `&mut Self`
  instead of a clone.  Chaining such as `n.succ().to_s()` still works; where you kept the
  returned value, add `.clone()`.
* `add`, `mul`, `mut_add`, `mut_mul` and `pow` accept owned or borrowed operands
  (`impl Borrow<Digits>`).
* `Display` writes the number itself (`"0042"`), and `to_string` comes from `Display`.
  `Debug` shows the value, base and mapping.
* Equality and hashing compare numeric values and ignore zero padding, matching
  ordering.  `Digits` now implements `Eq` and `Hash`.
* Comparing numbers with different mappings returns `None` instead of panicking.
* Addition through `mut_add` and `+`/`+=` keeps the width of the wider operand, as `add`
  already did.  Previously they kept only the left operand's width.
* Base conversions (`From`, `gen`, `Radix`) always produce results without zero padding.
* `pow` and `mut_mul` results never carry zero padding.
* Non-adjacent stepping works in every base (it used to panic below base 4), and
  `step_non_adjacent` no longer needs a valid starting value.
* Requires Rust 1.56 or newer (edition 2021).

### Added

* Operators for borrowed values: `&a + &b`, `a * &b`, `c ^= &d` and so on.
* `Digits::mapping` returns the `BaseCustom` a number uses.
* `From<&Digits> for String`.
* Clear panic messages for characters outside a base and for arithmetic across bases of
  different sizes.

### Removed

* The `clippy` cargo feature (a long-obsolete compiler plugin).
* The `array_tool` dependency.
* `replicate` is deprecated; use `clone`.

### Migrating from 1.x

```rust,ignore
// 1.x
let n = Digits::new(base, "42".to_string());
let next = n.clone().succ();        // succ returned a clone
let s = n.to_string();              // inherent method
// 2.0
let n = Digits::new(base, "42");
let next = n.clone().succ().clone(); // or: let mut next = n.clone(); next.succ();
let s = n.to_string();              // from Display, same text
```

A borrow error such as `n.succ().mut_add(n.one())` means a value taken from `n` is used
while `n` is being changed; take it first: `let one = n.one(); n.succ().mut_add(one);`.
