extern crate arrange;

use arrange::IntRange;

// These tests only cover arrays populated with integers.
// More tests, covering other types need to be added.
//
// TODO: add more tests

#[test]
fn basic1() {
    assert_eq!(
        IntRange::new(0, 10, 1).range(),
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}

#[test]
fn basic2() {
    assert_eq!(IntRange::new(1, 10, 1).range(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    /* start: 1
        stop: 10
    }) == [1, 2, 3, 4, 5, 6, 7, 8, 9]assert_eq!(	range.int({ start:1, stop: 10 }), [1, 2, 3, 4, 5, 6, 7, 8, 9]); */
}

#[test]
fn basic3() {
    assert_eq!(IntRange::new(1, 10, 2).range(), [1, 3, 5, 7, 9]);
    /* start: 1
        stop: 10
        step: 2
    }) == [1, 3, 5, 7, 9] */
}
