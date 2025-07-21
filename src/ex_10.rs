// Exercise: Shape Enum and Area Calculation
//
// Instructions:
// 1. Define an enum Shape with variants: Circle(f64), Rectangle(f64, f64), Square(f64).
// 2. Implement the function get_area that takes a Shape and returns its area as f64.
// 3. In main, create examples of each shape and print their areas.
// 4. Use the test to check your implementation.

enum Shape {
    Circle(f64),    // radius
    Rectangle(f64, f64), // width, height
    Square(f64),    // side
}

// TODO: Implement this function
fn get_area(shape: Shape) -> f64 {
    todo!("Implement area calculation for each shape variant")
}

fn main() {
    let c = Shape::Circle(2.0);
    let r = Shape::Rectangle(3.0, 4.0);
    let s = Shape::Square(5.0);
    println!("Circle area: {}", get_area(c));
    println!("Rectangle area: {}", get_area(r));
    println!("Square area: {}", get_area(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        let c = Shape::Circle(2.0);
        let r = Shape::Rectangle(3.0, 4.0);
        let s = Shape::Square(5.0);
        assert!((get_area(c) - 12.56636).abs() < 1e-5);
        assert_eq!(get_area(r), 12.0);
        assert_eq!(get_area(s), 25.0);
    }
} 