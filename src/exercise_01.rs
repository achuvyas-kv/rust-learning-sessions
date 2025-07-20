/// Exercise 01: Basic Function Implementation
/// 
/// Your task is to implement the `add_numbers` function that takes two integers
/// and returns their sum.
/// 
/// Example:
/// add_numbers(5, 3) should return 8
/// add_numbers(-1, 1) should return 0
/// add_numbers(0, 0) should return 0

// TODO: Implement this function
pub fn add_numbers(a: i32, b: i32) -> i32 {
    // Your implementation goes here
    // Replace this line with your solution
    return a + b;
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