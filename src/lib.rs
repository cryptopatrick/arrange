/// Subtracts one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = arrange::minus_just_one(arg);
///
/// assert_eq!(4, answer);
/// ```
pub fn minus_just_one(x: i32) -> i32 {
    x - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(minus_just_one(5), 4);
    }
}
