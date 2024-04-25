// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) -> String {
    data.to_uppercase()
}

// Writeup
// get_char() is originally taking ownership of the String data, but it should not. 
// We can fix this by passing a reference to the String data instead.
//
// string_uppercase() is originally taking a reference to the String data, but it should take ownership. 
// We can fix this by changing the function signature to take ownership of the String data. 