fn main() {
    // Rust: Ownership and Borrow Checker
    let mut numbers = vec![0, 10, 20, 30, 40]; // Owned by main

    // Borrow mutably
    modify_numbers(&mut numbers);

    // Borrow immutably
    print_numbers(&numbers);

    // Can still use numbers after borrowing
    println!("After modification: {:?}", numbers);

    // Ownership transfer
    let moved_numbers = numbers; // numbers is moved, can't use it anymore
    println!("Moved: {:?}", moved_numbers);

    println!("{:?}", numbers);
}

fn print_numbers(nums: &Vec<i32>) {
    // Immutable borrow - can read but not modify
    for (i, &num) in nums.iter().enumerate() {
        println!("numbers[{}] = {}", i, num);
    }
}

fn modify_numbers(nums: &mut Vec<i32>) {
    // Mutable borrow - can modify
    for num in nums.iter_mut() {
        *num *= 2;
    }
}

// // // This would cause a compile error if we tried to use both borrows;
// let immutable_numbers = &numbers;
// let mutable_numbers = &mut numbers;
//
// println!("Immutable: {:?}", immutable_numbers); // Error!
// println!("Mutable: {:?}", mutable_numbers);
//
//
//

// Ownership
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
//
//

// References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
