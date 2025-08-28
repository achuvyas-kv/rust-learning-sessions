/// Exercise 05: Calculator Struct Implementation
/// 
/// Your task is to implement a Calculator struct with methods for basic operations.
/// You need to:
/// 1. Complete the Calculator struct definition
/// 2. Implement the required methods 
/// 3. Make sure the tests pass
/// 
/// The Calculator should track a running total and provide methods to:
/// - add a number to the total
/// - subtract a number from the total  
/// - multiply the total by a number
/// - get the current total
/// - reset the total to 0

// TODO: Complete the struct definition
struct Calculator {
    // Add a field to store the running total
}




fn main() {
    todo!("Add operations to the calculator");
    // let mut calc = Calculator::new();
    // calc.add(10);
    // calc.subtract(3);
    // calc.multiply(2);
    // println!("Final total: {}", calc.get_total());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_new_calculator() {
        let calc = Calculator::new();
        assert_eq!(calc.get_total(), 0);
    }

    #[test]
    #[ignore]
    fn test_add_operation() {
        let mut calc = Calculator::new();
        calc.add(5);
        assert_eq!(calc.get_total(), 5);
        calc.add(3);
        assert_eq!(calc.get_total(), 8);
    }

    #[test]
    #[ignore]
    fn test_subtract_operation() {
        let mut calc = Calculator::new();
        calc.add(10);
        calc.subtract(3);
        assert_eq!(calc.get_total(), 7);
    }

    #[test]
    #[ignore]
    fn test_multiply_operation() {
        let mut calc = Calculator::new();
        calc.add(5);
        calc.multiply(3);
        assert_eq!(calc.get_total(), 15);
    }

    #[test]
    #[ignore]
    fn test_reset_operation() {
        let mut calc = Calculator::new();
        calc.add(100);
        calc.subtract(20);
        calc.reset();
        assert_eq!(calc.get_total(), 0);
    }

    #[test]
    #[ignore]
    fn test_complex_calculation() {
        let mut calc = Calculator::new();
        calc.add(10);
        calc.subtract(2);
        calc.multiply(3);
        calc.add(5);
        assert_eq!(calc.get_total(), 29);
    }
} 