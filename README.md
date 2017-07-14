# digits
[![Build Status](https://travis-ci.org/danielpclark/digits.svg)](https://travis-ci.org/danielpclark/digits)
[![crates.io version](https://img.shields.io/crates/v/digits.svg)](https://crates.io/crates/digits)
[![License](https://img.shields.io/crates/l/digits.svg)]()

Digits is a custom character base numeric sequencer.  This crate is designed for infinite character progressions.  It will contain additive methods such as `add` and `mul`.  _Documentation is coming very soon!_

This is an extension on top of [base_custom](https://github.com/danielpclark/base_custom).  That has the following benefits.

Use any characters as your own numeric base and convert to and from decimal.  This can be taken advantage of in various ways:

* Mathematics: number conversion
* Brute force sequencing
* Rolling ciphers
* Moderate information concealment
* Other potential uses such as deriving music or art from numbers

### Installation

Add the following to your Cargo.toml file
```toml
[dependencies]
digits = "^0.0.3"
```

To include it for usage add

```rust
extern crate digits;
use digits::Digits;
```

to your file.

### Usage

**WIP!**

```rust
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
