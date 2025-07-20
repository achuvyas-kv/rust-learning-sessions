/// SOLUTION for Exercise 01: Basic Function Implementation
/// 
/// This file shows the correct implementation of the add_numbers function.
/// Use this as a reference after attempting the exercise yourself.

pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add_numbers(5, 3), 8);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add_numbers(-1, -2), -3);
    }

    #[test]
    fn test_add_mixed_numbers() {
        assert_eq!(add_numbers(-1, 1), 0);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(5, 0), 5);
        assert_eq!(add_numbers(0, 5), 5);
    }

    #[test]
    fn test_add_large_numbers() {
        assert_eq!(add_numbers(1000, 2000), 3000);
    }
} 