extern crate arrange;

use arrange::IntRange;

fn main() {
    let ir = IntRange::new(1, 10, 1).range();
    assert_eq!(ir, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
