// Exercise: Mutable Variables
//
// Instructions:
// 1. Create a mutable variable `counter` starting at 0
// 2. Increment it by 1
// 3. Increment it by 2
// 4. The final value should be 3
//
// Expected output:
// Counter: 0
// Counter: 1
// Counter: 3
//
// Fill in the code below:

fn main() {
    todo!("Fix the bug in the code below");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_final_value() {
        let  counter = 0;
        assert_eq!(counter, 3, "Counter should be 3, but got {}", counter);

    }
}
