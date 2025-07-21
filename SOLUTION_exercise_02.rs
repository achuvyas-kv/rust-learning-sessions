// Solution for Exercise 02: Add Index to Each Number

pub fn add_index_to_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .enumerate()
        .map(|(i, x)| i as i32 + x)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_index_to_numbers() {
        let input = vec![10, 20, 30, 40];
        let expected = vec![10, 21, 32, 43];
        assert_eq!(add_index_to_numbers(&input), expected);
    }

    #[test]
    fn test_empty_vector() {
        let input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(add_index_to_numbers(&input), expected);
    }

    #[test]
    fn test_single_element() {
        let input = vec![5];
        let expected = vec![5];
        assert_eq!(add_index_to_numbers(&input), expected);
    }

    #[test]
    fn test_negative_numbers() {
        let input = vec![-5, -3, -1];
        let expected = vec![-5, -2, 1];
        assert_eq!(add_index_to_numbers(&input), expected);
    }
} 