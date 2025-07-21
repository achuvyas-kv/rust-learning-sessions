# Rust Learning Sessions

---

## Introduction

Welcome to Rust Learning Sessions! This is a hands-on tutorial where we'll explore Rust programming concepts through practical exercises.

### About This Session

I'm still learning Rust myself, so expect some mistakes

---

## What We'll Cover

Anything that helps to get started.

---

## Getting Started with Syntax

Let's cover enough syntax to get started (and compare it with TypeScript). I think most of you guys know TypeScript and the basic syntax is easily comparable.

---

## Hello World

### TypeScript
```typescript
console.log("Hello World");
```

### Rust
```rust
fn main() {
    println!("Hello World");
}
```
---

## Defining Variables & Constants

### TypeScript
```typescript
const foo = "hola";
let bar = "hello";
```

### Rust
```rust
let foo = "hola";
let mut bar = "Hello";

const PI: f64 = 3.14159;
let radius = 5.3;
let area = PI * radius * radius;
```

---

## Printing a Variable in Rust

You can print the value of a variable using the `println!` macro:

### TypeScript
```typescript
const favoriteColor = "blue";
console.log(`My favorite color is ${favoriteColor}.`); // Output: My favorite color is blue.
```

### Rust
```rust
let favorite_color = "blue";
println!("My favorite color is {}.", favorite_color); // Output: My favorite color is blue.
```

---


## Data Types

### TypeScript
```typescript
// Primitive types
let str: string = "hello";
let num: number = 42;
let bool: boolean = true;
let arr: number[] = [1, 2, 3];
let obj: { name: string; age: number } = { name: "John", age: 30 };
```




### Rust
```rust
// Primitive types
let num: i32 = 42;
let float: f32 = 4.5;
let bool: bool = true;

let str: &str = "hello";
let string: String = String::from("hello");
let arr: Vec<i32> = vec![1, 2, 3];

// Struct (similar to TypeScript object)
struct Person {
    name: String,
    age: u32,
}

let person = Person {
    name: String::from("John"),
    age: 30,
};
```

---

## String and `&str`

### What's the Difference?

```rust
let s: String = String::from("hello");
let r: &str = &s; // borrow as &str

let s2: &str = "world";
let s3: String = s2.to_string();

let text = String::from("Rust programming");
let word: &str = &text[0..4]; // slice the first word
```

---


### Value (Owned)
```rust
let mut s = String::from("hello"); // `s` owns the String

// ✅ READ - Can read owned data
println!("{}", s); // "hello"

// ✅ WRITE - Can modify owned data (if mutable)
s.push_str(" world");
println!("{}", s); // "hello world"
```

### Immutable Reference (&T)
```rust
let s = String::from("hello");
let r1 = &s; // Immutable borrow

println!("{}", r1); // ✅ allowed

// ❌ NOT ALLOWED - Can't modify through immutable reference
// r1.push_str(" world"); // This would cause a compile error
```

### Mutable Reference (&mut T)
```rust
let mut s = String::from("hello");
let r1 = &mut s; // Mutable borrow

r1.push_str(" world");
println!("{}", r1); // ✅ "hello world"
```

---

## Arithmetic Operations

### Type Safety in Rust

```rust
let a: f32 = 3.5;
let b: f32 = 2.0;
let c: i32 = 5;

let result1 = a + b; // ✅ OK: f32 + f32

let result2 = a + c; // ❌ Error: no implementation for `f32 + i32`

let result2 = a + c as f32; // ✅ OK after casting i32 to f32
```

---

## usize Type

### Platform-Dependent Integer

```rust
// usize: unsigned integer based on system architecture
// - 32-bit systems: 32 bits (0 to 4,294,967,295)
// - 64-bit systems: 64 bits (0 to 18,446,744,073,709,551,615)

let array = vec![1, 2, 3, 4, 5];
let length: usize = array.len(); // Array length is usize

let index: usize = 2;
let element = array[index]; // Array indexing uses usize

// Common use cases
for i in 0..array.len() { // usize for loop indices
    println!("Element {}: {}", i, array[i]);
}
```

---

## Functions

### TypeScript Functions
```typescript
function add(a: number, b: number) {
    return a + b;
}
```

### Rust Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  
}

//unit type (nothing)
fn print_hello() {
    println!("Hello!");
}
```

---

## Arrow Functions & Closures

### TypeScript Arrow Functions
```typescript
const add = (a: number, b: number) => a + b;

onst numbers = [1, 2, 3, 4, 5];
const doubled = numbers.map(x => x * 2);
```

### Rust Closures
```rust
let add = |a: i32, b: i32| a + b;

let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

---



## Conditional Statements

### TypeScript
```typescript
if (x > 5) {
    console.log("greater than 5");
} else {
    console.log("less than 5");
}
```

### Rust
```rust
if x > 5 {
    println!("greater than 5");
} else {
    println!("less than 5");
}
```

---

## Arrays

### TypeScript
```typescript
let array = [3, 4, 5];
```

### Rust
```rust
let array = vec![3, 4, 5];
```

---

## For Loops

### TypeScript
```typescript
let array = [1, 2, 3, 4, 5];

// Iterate over values
for (let item of array) {
    console.log(item);
}
```

### Rust
```rust
let array = vec![1, 2, 3, 4, 5];

// Iterate over values
for item in &array {
    println!("{}", item);
}
```

---


---

## Map Method

### TypeScript
```typescript
const numbers = [1, 2, 3, 4, 5];
const doubled = numbers.map(x => x * 2);
console.log(doubled); // [2, 4, 6, 8, 10]
```

### Rust
```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("{:?}", doubled); // [2, 4, 6, 8, 10]
```

---

## Iter and Collect Methods


### Rust
```rust
let numbers = vec![1, 2, 3, 4, 5];

// iter() creates an iterator over references
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("{:?}", doubled); // [2, 4, 6, 8, 10]

// into_iter() takes ownership
let doubled_owned: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
println!("{:?}", doubled_owned); // [2, 4, 6, 8, 10]


// collect consumes the iterator and builds the collectoin back

```

---

## Filter Method


//These methods are adapters to the iterator

### TypeScript
```typescript
const numbers = [1, 2, 3, 4, 5, 6];
const evens = numbers.filter(x => x % 2 === 0);
console.log(evens); // [2, 4, 6]
```

### Rust
```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
println!("{:?}", evens); // [2, 4, 6]
```

---

## Enumerate Method

### TypeScript
```typescript
const fruits = ["apple", "banana", "cherry"];
fruits.forEach((fruit, index) => {
    console.log(`${index}: ${fruit}`);
});
// 0: apple
// 1: banana
// 2: cherry
```

### Rust
```rust
let fruits = vec!["apple", "banana", "cherry"];
for (index, fruit) in fruits.iter().enumerate() {
    println!("{}: {}", index, fruit);
}
// 0: apple
// 1: banana
// 2: cherry
```

---

## Slicing

### Rust
```rust
let v = vec![10, 20, 30, 40, 50];

let slice: &[i32] = &v[1..4]; // gets elements at index 1, 2, 3
println!("{:?}", slice); // Output: [20, 30, 40]
```

---

## Combining Filter and Map

### TypeScript
```typescript
const numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const result = numbers
    .filter(x => x % 2 === 0)  // get even numbers
    .map(x => x * 2);          // double them
console.log(result); // [4, 8, 12, 16, 20]
```

### Rust
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let result: Vec<i32> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)  // get even numbers
    .map(|x| x * 2)           // double them
    .collect();
println!("{:?}", result); // [4, 8, 12, 16, 20]
```

---

## Struct Implementation

---

## Trait

---

## Enums

### Basic Enum Definition
```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    let my_color = Color::Red;
    println!("My color is: {:?}", my_color); // My color is: Red
}
```

### Pattern Matching with Enums
```rust
fn get_color_name(color: Color) -> &'static str {
    match color {
        Color::Red => "Red",
        Color::Green => "Green", 
        Color::Blue => "Blue",
        Color::Yellow => "Yellow",
    }
}

fn main() {
    let colors = vec![Color::Red, Color::Blue, Color::Green];
    
    for color in colors {
        println!("Color: {}", get_color_name(color));
    }
    // Output:
    // Color: Red
    // Color: Blue
    // Color: Green
}
```

### Enum with Data
```rust
enum Shape {
    Circle(f64),    // radius
    Rectangle(f64, f64), // width, height
    Square(f64),    // side
}

fn get_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}
```

---

## Options

### Option Enum Definition
```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Using Option
```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero!"),
    }
    
    // Using unwrap (dangerous!)
    let safe_result = divide(10.0, 2.0).unwrap(); // 5.0
    
    // Using unwrap_or for safe default
    let default_result = divide(10.0, 0.0).unwrap_or(0.0); // 0.0
}
```

---

## Result

### Result Enum Definition
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Using Result
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    
    // Using unwrap (dangerous!)
    let safe_result = divide(10.0, 2.0).unwrap(); // 5.0
    
    // Using unwrap_or_else for safe default
    let default_result = divide(10.0, 0.0).unwrap_or_else(|_| 0.0); // 0.0
    
    // Using ? operator (in functions that return Result)
    fn process_division() -> Result<f64, String> {
        let result = divide(10.0, 0.0)?; // Returns early if Err
        Ok(result * 2.0)
    }
}
```

---

## Borrow Checking Rules

---

*Happy coding! 🦀*














