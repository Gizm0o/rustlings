// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}


// Writeup 
// To call the macro with an argument, we need to define a new rule in the macro definition.
// The rule should have a pattern that matches the argument, and the pattern should be followed by a block that uses the argument.
// In this case, we added a new rule that matches an expression, and we used the expression in the block to print the value.