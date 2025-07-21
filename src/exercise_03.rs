// Problem: Add Index to Even Numbers
// 
// Given a vector of numbers, create a new vector containing only the even numbers
// with their index in the original vector added to them.
//
// Example:
// Input: vec![10, 21, 30, 41, 50]
// Output: vec![10, 32, 54] (10+0, 30+2, 50+4)

pub fn add_index_to_even_numbers(numbers: &[i32]) -> Vec<i32> {
    // todo!("Implement this function to add index only to even numbers")
    return numbers
        .iter()
        .enumerate()
        .filter(|(_, x)| **x % 2 == 0)
        .map(|(i, x)| *x + i as i32)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_index_to_even_numbers() {
        let input = vec![10, 21, 30, 41, 50];
        let expected = vec![10, 32, 54];
        assert_eq!(add_index_to_even_numbers(&input), expected);
    }

    #[test]
    fn test_all_odd_numbers() {
        let input = vec![1, 3, 5, 7];
        let expected: Vec<i32> = vec![];
        assert_eq!(add_index_to_even_numbers(&input), expected);
    }

    #[test]
    fn test_all_even_numbers() {
        let input = vec![2, 4, 6, 8];
        let expected = vec![2, 5, 8, 11];
        assert_eq!(add_index_to_even_numbers(&input), expected);
    }

    #[test]
    fn test_empty_vector() {
        let input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(add_index_to_even_numbers(&input), expected);
    }

    #[test]
    fn test_negative_numbers() {
        let input = vec![-4, -3, -2, -1, 0];
        let expected = vec![-4, 0, 4];
        assert_eq!(add_index_to_even_numbers(&input), expected);
    }
}

