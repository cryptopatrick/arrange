extern crate arrange;

use arrange::Range;

fn ar() -> Option<[]T, Error> {
    Range::new(range_struct)
}

// These tests only cover arrays populated with integers.
// More tests, covering other types need to be added.
//
// TODO: add more tests
#[test]
fn basic1() {
    assert_eq!(	range.int({ stop: 3 }), [0, 1, 2]);
}

#[test]
fn basic2() {
    assert_eq!(	range.int({ start:1, stop: 3 }), [1, 2]);
}

#[test]
fn basic3() {
    assert_eq!(	range.int({ start:1, stop: 10, step: 2 }), [1, 3, 5, 7, 9]);
}