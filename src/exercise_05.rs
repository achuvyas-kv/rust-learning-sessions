// Problem: Color Pattern Matching
// 
// Create a Color enum and implement functions to work with colors:
// - Define a Color enum with Red, Green, Blue, Yellow, Purple, Orange
// - Implement a function get_color_name(color: Color) -> &'static str that returns the color name
// - Implement a function is_warm_color(color: Color) -> bool that returns true for warm colors
//   (Red, Orange, Yellow are warm colors; Green, Blue, Purple are cool colors)
// - Implement a function get_complementary_color(color: Color) -> Color that returns the complementary color
//   (Red ↔ Green, Blue ↔ Orange, Yellow ↔ Purple)

#[derive(Debug, PartialEq)]
pub enum Color {
    // TODO: Define the color variants
}

pub fn get_color_name(color: Color) -> &'static str {
    todo!("Return the name of the color as a string")
}

pub fn is_warm_color(color: Color) -> bool {
    todo!("Return true if the color is warm (Red, Orange, Yellow)")
}

pub fn get_complementary_color(color: Color) -> Color {
    todo!("Return the complementary color")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color_name() {
        assert_eq!(get_color_name(Color::Red), "Red");
        assert_eq!(get_color_name(Color::Blue), "Blue");
        assert_eq!(get_color_name(Color::Green), "Green");
    }

    #[test]
    fn test_is_warm_color() {
        assert_eq!(is_warm_color(Color::Red), true);
        assert_eq!(is_warm_color(Color::Orange), true);
        assert_eq!(is_warm_color(Color::Yellow), true);
        assert_eq!(is_warm_color(Color::Green), false);
        assert_eq!(is_warm_color(Color::Blue), false);
        assert_eq!(is_warm_color(Color::Purple), false);
    }

    #[test]
    fn test_get_complementary_color() {
        assert_eq!(get_complementary_color(Color::Red), Color::Green);
        assert_eq!(get_complementary_color(Color::Green), Color::Red);
        assert_eq!(get_complementary_color(Color::Blue), Color::Orange);
        assert_eq!(get_complementary_color(Color::Orange), Color::Blue);
        assert_eq!(get_complementary_color(Color::Yellow), Color::Purple);
        assert_eq!(get_complementary_color(Color::Purple), Color::Yellow);
    }
} 