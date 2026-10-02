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

Measured in release builds, 1.1.1 → 2.0.0:

| Operation | 1.1.1 | 2.0.0 |
|---|---|---|
| `succ` on 4,000 nines (full carry) | 16 s | 2.5 µs |
| `succ` without carry on 4,000 digits | 4.3 ms | 6 ns |
| 40 × 40 digit `mul` | 11 s | 6 µs |
| `2^200` with `pow` | 12 s | 7 µs |
| `2^100000` with `pow` | (did not finish) | 0.19 s |
| 10,000 digit decimal → hex | (overflowed) | 56 ms |
| 100,000 `next_non_adjacent(0)` steps on 7 digits | — | 2.2 ms |

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
