// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}


// Writeup
// The Book struct is holding references to the author and title fields.
// We need to add a lifetime annotation to the Book struct to ensure that the references are valid for the lifetime of the struct.
// We can do this by adding a lifetime parameter to the Book struct definition.
// The lifetime parameter 'a is then used to annotate the author and title fields.
// This tells the compiler that the references in the author and title fields have the same lifetime as the Book struct itself.
// This ensures that the references are valid for as long as the Book struct is in scope.