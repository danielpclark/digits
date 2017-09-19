# digits
[![Build Status](https://travis-ci.org/danielpclark/digits.svg?branch=master)](https://travis-ci.org/danielpclark/digits)
[![crates.io version](https://img.shields.io/crates/v/digits.svg)](https://crates.io/crates/digits)
[![Documentation](https://img.shields.io/badge/docs-%F0%9F%91%8D-brightgreen.svg)](http://danielpclark.github.io/digits/index.html)

Digits is a custom character base numeric sequencer.  This crate is designed for infinite character progressions.  It will contain additive methods such as `add` and `mul`.

This is an extension on top of [base_custom](https://github.com/danielpclark/base_custom).

The largest unsigned digit type in Rust is **u64**.  Consider this an upgrade to **u∞**
The limits that this can calculate to are unknown and may only be limited to your systems RAM
should you try to reach infinity ;-).

This package lets you invent your own numeric systems and perform basic math on them including:

* addition
* multiplication
* multiply by powers of
* and simple +1/-1 steps with `succ` and `pred_till_zero`
* as of version 0.3 Digits preserves zero padding for addition methods

You may consider this a highly advanced score card flipper (character sequences) with basic
math methods added to help progress through sequences as you would like.

### Installation

Add the following to your Cargo.toml file
```toml
[dependencies]
digits = "^0.3"
```

To include it for usage add

```rust
extern crate digits;
use digits::{BaseCustom,Digits};
```

to your file.

### Usage

There are several ways to create a new instance of **Digits**, but before any of that you need
to define you own numeric base from the `base_custom` package.

```rust
// Define your own numeric base to use using a set of any characters
// We'll use the string representations for base 10 so you can see this
// work with something familiar.

let base10 = BaseCustom::<char>::new("0123456789".chars().collect());

// Once you have a custom numeric base defined you can create instances of Digits in many ways.

let hundred = Digits::new(&base10, "100".to_string());

// If you don't want to have to pass the base value in each time you create a new number
// you can propagate a new one out with the `propagate` method.

let two = hundred.propagate("2".to_string()); // re-uses internal base10 mappings

// Now we have two instances of Digits created: one for 100 and one for 2

// The mathematical methods mutate the Digits instance they're called from
// so you need to either use `let mut` or call `clone` to use them.

hundred.clone().add(two).to_s() // outputs: "102"
hundred.clone().mul(two).to_s() // outputs: "200"
hundred.clone().pow(two).to_s() // outputs: "10000"

// There are several ways to create and check one or zero.

let one = Digits::new_one(&base10); // A Digits instance with the value of 1
one.is_one() // true
one.is_zero() // false

let zero = Digits::new_zero(&base10); // A Digits instance with the value of 0
zero.is_one() // false
zero.is_zero() // true

// And you can create a one or zero off of an existing Digits instance with `one` or `zero`
hundred.one() // A Digits instance with the value of 1
hundred.zero() // A Digits instance with the value of 0

// Count down or up with `pred_till_zero` and `succ`
let mut ten = Digits::new(&base10, "10".to_string());
assert_eq!(ten.pred_till_zero().to_s(), "09");
assert_eq!(ten.pred_till_zero().to_s(), "08");
assert_eq!(ten.pred_till_zero().to_s(), "07");

let mut nine = Digits::new(&base10, "9".to_string());
assert_eq!(nine.succ().to_s(), "10");
assert_eq!(nine.succ().to_s(), "11");
assert_eq!(nine.succ().to_s(), "12");
```

And this is just with normal 0 thourgh 9 values.  Imagine if you invent your own
numeric bases and character sets.  It can be used for quite a lot!

## Goals / Roadmap

1) The first goal of this library is to be thread safe and function well for sequencing characters.

2) The secondary goal, which may improve with time, is performance.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
