// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

// Writeup
// In Rust, a function has to return a value, here there are two ways to return a value from a function:
// 1. Using the return keyword
// 2. Without using the return keyword, the last expression in the function will be returned. Using a semicolon at the end of the expression will make it a statement and it will not be returned.