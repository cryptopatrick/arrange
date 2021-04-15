arrange
======
Convenience functions for populating an array with values from a range.

[![Build status](https://github.com/CryptoPatrick/arrange/workflows/ci/badge.svg)](https://github.com/CryptoPatrick/arrange/actions)
[![](http://meritbadge.herokuapp.com/suffix)](https://crates.io/crates/suffix)

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).


### Documentation
https://docs.rs/arrange


Why

    a..b in V can only be in increasing order and not in negative order.
    Lacks inbuilt step which most people need or want.
    No support for float type.
    Solution for vlang/v#5944.

Features

    Make range arrays easily
    Make ranges for int and f32
    Positive as well as Negative Support!
    No need to write the whole for loop! (this maybe slower than the normal one)
    Use range for functional programming
    Full Python's range() functionality







### Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/arrange).  
The package will be regularly updated.
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
arrange = "0.1.0"
```

### Usage


    range.int(start:0, stop:value, step:1) makes a range of int with the following parameters:
        start: start value of the range by default it's 0
        stop: stop value of the range
        step: step value of the range by default it's 1

    range.float(start:0.0, stop:value, step:1.0) makes a range of f32 with the following parameters:
        start: start value of the range by default it's 0.0
        stop: stop value of the range
        step: step value of the range by default it's 1.0

Note: If range.int(step:0) or range.float(step:0) then an error will be raised because step cannot be zero.

### Examples

Usage is simple:

```rust
use arrange::Range;

fn main() {
    let a: []int = Range<int>::new(start:1, stop:10, step:2);
    println("{:?}", a);
}
```


```rust 
extern crate arrange;
use arrange::IntRange;

fn main() {
    /*     let mir = IntRange::new(1, 10, 2);
    let b = mir.range();
    println!("first {:?}", mir);
    println!("{:?}", b);
    //
    println!("second {:?}", IntRange::new(1, 10, 2).range()); */
    for i in IntRange::new(1, 10, 2).range() {
        print!("{} ", i); // prints out: 13579
    }
}

    /* range.int(start:1, stop:10) {
    range.int(start:1, stop:10, step:2) {
    range.int(start:10, stop:1, step:-1) {
    range.int(start:-5, stop:-1, step: 1) {
    // using range.float
    range.float(stop:10) {
    range.float(stop:10, step:0.2) {
    range.float(start:0.1, stop:10) {
    range.float(start:10, stop:1.0, step:-0.2) {
    range.float(start:-10, stop:-1.0, step:0.2) { */

```






### Status of implementation
Some may want to negative indexing but we won't implement that for the time since
it can be prone to bugs.