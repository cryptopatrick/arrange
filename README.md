arrange
======
Convenience functions for populating an array with values from a range.

[![Build status](https://github.com/CryptoPatrick/arrange/workflows/ci/badge.svg)](https://github.com/CryptoPatrick/arrange/actions)
[![](http://meritbadge.herokuapp.com/suffix)](https://crates.io/crates/suffix)

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).


### Documentation

https://docs.rs/arrange

If you just want the details on how construction algorithm used, see the
documentation for the `SuffixTable` type. This is where you'll find info on
exactly how much overhead is required.


### Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/arrange).  
The package will be regularly updated.
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
arrange = "0.1.0"
```


### Examples

Usage is simple. Just create a ranged array:

```rust
use arrange;::SuffixTable;

fn main() {
  let st = SuffixTable::new("the quick brown fox was quick.");
  assert_eq!(st.positions("quick"), &[4, 24]);
}
```




### Status of implementation

The big thing missing at the moment is a generalized suffix array. I started
out with the intention to build them into the construction algorithm, but this
has proved more difficult than I thought.

A kind-of-sort-of compromise is to append your distinct texts together, and
separate them with a character that doesn't appear in your document. (This is
technically incorrect, but maybe your documents don't contain any `NUL`
characters.) During construction of this one giant string, you should record
the offsets of where each document starts and stops. Then build a `SuffixTable`
with your giant string. After searching with the `SuffixTable`, you can find
the original document by doing a binary search on your list of documents.

I'm currently experimenting with different techniques to do this.


