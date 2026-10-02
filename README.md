# digits
[![CI](https://github.com/danielpclark/digits/actions/workflows/ci.yml/badge.svg)](https://github.com/danielpclark/digits/actions/workflows/ci.yml)
[![crates.io version](https://img.shields.io/crates/v/digits.svg)](https://crates.io/crates/digits)
[![Documentation](https://docs.rs/digits/badge.svg)](https://docs.rs/digits)

Digits is a custom character base numeric sequencer.  This crate is designed for infinite character progressions.
It contains additive methods such as `add` and `mul`.

This is an extension on top of [base_custom](https://github.com/danielpclark/base_custom).

The largest unsigned digit type in Rust is **u128**.  Consider this an upgrade to **u∞**.
Numbers are stored as plain vectors of digits and every operation is iterative, so the only
limit on their size is your system's RAM should you try to reach infinity ;-).

This package lets you invent your own numeric systems and perform basic math on them including:

* addition
* multiplication
* raising to a power
* simple +1/-1 steps with `succ` and `pred_till_zero`
* stepping through sequences that limit how often a character may repeat next to itself
* conversion between any two numeric bases

You may consider this a highly advanced score card flipper (character sequences) with basic
math methods added to help progress through sequences as you would like.

### Installation

Add the following to your Cargo.toml file
```toml
[dependencies]
digits = "2.0"
```

and bring it into scope with

```rust
use digits::prelude::*;
```

Upgrading from 1.x?  See the [changelog](CHANGELOG.md) for what changed and how to migrate.

### Usage

Before creating a number you define your own numeric base with `BaseCustom` from the
`base_custom` package (re-exported here).

```rust
use digits::prelude::*;

// Define your own numeric base using a set of any characters.  We'll use the
// characters for base 10 so you can see this work with something familiar.
let base10 = BaseCustom::<char>::new("0123456789".chars().collect());

// Once you have a custom numeric base defined you can create instances of Digits.
let hundred = Digits::new(base10.clone(), "100");

// `propagate` creates another number that shares the same character mapping.
let two = hundred.propagate("2");

// `add`, `mul` and the operators leave their operands untouched.
assert_eq!(hundred.add(&two).to_s(), "102");
assert_eq!(hundred.mul(&two).to_s(), "200");
assert_eq!((&hundred + &two).to_s(), "102");
assert_eq!((&hundred ^ &two).to_s(), "10000");

// The `mut_` methods, `pow`, `succ` and friends change the number in place
// and return it again so calls can be chained.
let mut counter = hundred.clone();
counter.mut_add(&two).pow(&two);
assert_eq!(counter.to_s(), "10404");

// There are several ways to create and check one or zero.
let one = Digits::new_one(base10.clone());
assert!(one.is_one());
assert!(!one.is_zero());

let zero = Digits::new_zero(base10.clone());
assert!(!zero.is_one());
assert!(zero.is_zero());

// And you can create a one or zero from an existing Digits instance.
assert_eq!(hundred.one().to_s(), "1");
assert_eq!(hundred.zero().to_s(), "0");

// Count down or up with `pred_till_zero` and `succ`.  Zero padding is kept.
let mut ten = Digits::new(base10.clone(), "10");
assert_eq!(ten.pred_till_zero().to_s(), "09");
assert_eq!(ten.pred_till_zero().to_s(), "08");
assert_eq!(ten.pred_till_zero().to_s(), "07");

let mut nine = Digits::new(base10, "9");
assert_eq!(nine.succ().to_s(), "10");
assert_eq!(nine.succ().to_s(), "11");
assert_eq!(nine.succ().to_s(), "12");
```

And this is just with normal 0 through 9 values.  Imagine if you invent your own
numeric bases and character sets.  It can be used for quite a lot!

```rust
use digits::prelude::*;
use digits::Radix;

// Brute force every lowercase three letter combination, starting at "aaa".
let letters = BaseCustom::<char>::new(('a'..='z').collect());
let mut word = Digits::new(letters, "aaa");
let mut count = 1;
while word.length() == 3 {
    count += 1;
    word.succ();
}
assert_eq!(count, 26 * 26 * 26 + 1);
assert_eq!(word.to_s(), "baaa");

// Skip sequences where a character repeats next to itself.
let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
let mut pin = Digits::new(base10, "0000");
assert_eq!(pin.next_non_adjacent(0).to_s(), "0101");
assert_eq!(pin.next_non_adjacent(0).to_s(), "0102");

// Convert between numeric bases, with no u64 limit.
let big = Digits::new(digits::radices::binary_base(), format!("1{}", "0".repeat(100)));
assert_eq!(big.decimal().to_s(), "1267650600228229401496703205376");
```

### Zero padding

Leading "zero" characters (the first character of a base) are part of a number's width:

* `succ`, `pred_till_zero` and addition keep the width of the widest operand and only
  grow when a carry needs another digit.
* Multiplication, `pow` and base conversion produce results without padding.
* `zero_fill` and `zero_trim` change the padding explicitly.
* Equality, hashing and ordering compare values and ignore padding, so `"009" == "9"`.
* The adjacency methods count padding zeros as characters.

### Threads

`Digits` is `Send + Sync`.  A number's character mapping is shared through an `Arc`, so cloning
numbers, or deriving new ones with `propagate`, `zero`, `one` or `new_mapped`, never copies the mapping.

## Goals / Roadmap

1) The first goal of this library is to be thread safe and function well for sequencing characters.

2) The secondary goal, which may improve with time, is performance.

3) The third goal is to have fun re-inventing mathematics and experiments.

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([MIT-LICENSE](MIT-LICENSE) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
