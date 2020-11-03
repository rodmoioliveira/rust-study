//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::add_one(3), 4);
/// assert_eq!(my_crate::add_one(4), 5);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_four() {
        assert_eq!(add_one(3), 4);
    }

    #[test]
    fn gets_five() {
        assert_eq!(add_one(4), 5);
    }
}
