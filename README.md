arrange
======
Convenience functions for populating an array with values from a range.

[![Build status](https://github.com/CryptoPatrick/arrange/workflows/ci/badge.svg)](https://github.com/CryptoPatrick/arrange/actions)
[![](http://meritbadge.herokuapp.com/suffix)](https://crates.io/crates/suffix)

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).


### Documentation
https://docs.rs/arrange


### Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/arrange).  
The package will be regularly updated.
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
arrange = "0.1.2"
```

### Examples

Usage is simple:

```rust
extern crate arrange;

use arrange::IntRange;

fn main() {
    assert_eq!(IntRange::new(1, 10, 1).range(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
```

### Status of implementation
Some may want to negative indexing but we won't implement that for the time
being since they can be prone to bugs.

Still a lot of work to do.

Make sure to use version 0.1.1 or above.