// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")] // Add this attribute
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
        
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")] // Add this attribute

    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}

// Writeup
// The correct_width_and_height test should check if the rectangle is the size that we pass into its constructor.
// To do this we use the assert_eq! macro to check if the width and height of the rectangle are equal to the values we 
// passed into the constructor.

// The negative_width and negative_height test should check if the program panics when we try to create a rectangle with a negative width. 
// To do this we use the #[should_panic(expected = "Rectangle width and height cannot be negative!")] attribute to check if
// the program panics with the expected message when we try to create a rectangle with a negative width.

