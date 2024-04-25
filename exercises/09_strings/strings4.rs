// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

// Writeup
// "blue" is a &str because it is a string literal
// "red".to_string() is a String because it is a method call on a string literal that returns a String
// String::from("hi") is a String because it is a method call on the String struct that returns a String
// "rust is fun!".to_owned() is a String because it is a method call on a string literal that returns a String
// format!("Interpolation {}", "Station") is a String because it is a macro that returns a String
// &String::from("abc")[0..1] is a &str because it is a reference to a slice of a String that returns a &str
// "  hello there ".trim() is a &str because it is a method call on a string literal that returns a &str
// "Happy Monday!".to_string().replace("Mon", "Tues") is a String because it is a method call on a string literal that returns a String
// "mY sHiFt KeY iS sTiCkY".to_lowercase() is a String because it is a method call on a string literal that returns a String
