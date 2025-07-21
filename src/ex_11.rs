// Function to find the first even number in a slice (not implemented)
pub fn find_first_even(arr: &[i32]) -> Option<i32> {
    unimplemented!("Implement this function as your task");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_even() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
    }

    #[test]
    fn test_multiple_evens() {
        assert_eq!(find_first_even(&[2, 4, 6, 8]), Some(2));
    }

    #[test]
    fn test_no_even() {
        assert_eq!(find_first_even(&[1, 3, 5, 7]), None);
    }

    #[test]
    fn test_empty() {
        assert_eq!(find_first_even(&[]), None);
    }

    #[test]
    fn test_even_at_end() {
        assert_eq!(find_first_even(&[1, 3, 5, 8]), Some(8));
    }
}
