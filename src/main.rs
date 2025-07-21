// Import exercise modules
pub mod exercise_01;
pub mod exercise_02;
pub mod exercise_03;
pub mod exercise_04;
pub mod exercise_05;

fn main() {
    println!("Welcome to Rust Learning Sessions!");
    println!("This is a tutorial repository with exercises.");
    println!();
    println!("To run tests for specific exercises:");
    println!("  cargo test exercise_01    # Run exercise 01 tests");
    println!("  cargo test        # Run all tests");
    println!();
    println!("Current exercises:");
    println!("  Exercise 01: Basic Function Implementation");
    println!("    - Implement the add_numbers function");
    println!("    - Run: cargo test exercise_01");
    println!("  Exercise 02: Add Index to Each Number");
    println!("    - Implement the add_index_to_numbers function");
    println!("    - Run: cargo test exercise_02");
    println!("  Exercise 03: Add Index to Even Numbers");
    println!("    - Implement the add_index_to_even_numbers function");
    println!("    - Run: cargo test exercise_03");
    println!("  Exercise 04: Account Struct Implementation");
    println!("    - Implement Account struct with deposit and withdraw methods");
    println!("    - No tests - just implement the struct and methods");
    println!("  Exercise 05: Color Pattern Matching");
    println!("    - Implement Color enum with pattern matching functions");
    println!("    - Run: cargo test exercise_05");


let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let result: Vec<i32> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)  // get even numbers
    .map(|x| x * 2)           // double them
    .collect();
println!("{:?}", result); // [4, 8, 12, 16, 20]

}
