# Rust Learning Sessions

A tutorial repository with hands-on exercises to learn Rust programming.

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/achuvyas-kv/rust-learning-sessions.git
   cd rust-learning-sessions
   ```

2. **Run the project:**
   ```bash
   cargo run
   ```

## Exercises

### Exercise 01: Basic Function Implementation

**File:** `src/exercise_01.rs`

**Task:** Implement the `add_numbers` function that takes two integers and returns their sum.

**Solution:** `SOLUTION_exercise_01.rs` (for reference after attempting)

**To solve:**
1. Open `src/exercise_01.rs`
2. Find the `add_numbers` function
3. Replace the `todo!()` macro with your implementation
4. Run the tests to check your solution

**Run tests:**
```bash
# Run all tests
cargo test

# Run only exercise 01 tests
cargo test exercise_01

# Run tests with output (even for passing tests)
cargo test -- --nocapture
```

## Exercise Structure

Each exercise follows this pattern:
- **File:** `src/exercise_XX.rs` (where XX is the exercise number)
- **Function:** Contains a `todo!()` macro that you need to replace
- **Tests:** Comprehensive test cases to verify your solution
- **Documentation:** Clear instructions and examples

## Testing Your Solutions

The repository uses Rust's built-in testing framework. Each exercise includes:
- Multiple test cases covering different scenarios
- Clear error messages when tests fail
- Examples of expected behavior

### Test Commands

```bash
# Run all tests
cargo test

# Run specific exercise tests (using test name pattern)
cargo test exercise_01    # Exercise 01
cargo test exercise_02    # Exercise 02 (when available)

# Run tests with verbose output
cargo test -- --nocapture

# Run tests and show output even for passing tests
cargo test -- --nocapture --test-threads=1
```

## Adding New Exercises

To add a new exercise:

1. Create `src/exercise_XX.rs` with your exercise
2. Add the module to `src/main.rs`
3. Update this README with exercise details
4. Ensure tests can be run with `cargo test exercise_XX`

## Learning Path

1. **Start with Exercise 01** - Basic function implementation
2. **Read the documentation** in each exercise file
3. **Try to solve** without looking at solutions
4. **Run tests** to verify your solution
5. **Move to the next exercise** when all tests pass

## Tips

- Use `cargo check` to check for compilation errors without running tests
- Use `cargo fmt` to format your code
- Use `cargo clippy` for additional linting suggestions
- Don't be afraid to experiment and make mistakes!

## Contributing

Feel free to add more exercises or improve existing ones. Make sure to:
- Include comprehensive tests
- Provide clear documentation
- Follow Rust coding conventions
- Update the README with new exercises

Happy coding! 🦀 