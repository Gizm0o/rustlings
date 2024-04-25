// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}


// Writeup
// The match statement is attempting to destructure y, which is a reference to an Option<Point> type.
// The match statement is trying to match the Some(ref p) pattern, which is a reference to a Point type
// When 'y' matches the Some(ref p) pattern, it moves the "Point" struct out of the Option type, and binds it to the 'p' variable.
// Match arm then executes, ownership of 'Point' is moved to 'p', and 'y' is no longer usable because it has been moved.
//
//With ref p, instead of moving the 'Point' struct, we are creatin a reference to the 'Point' struct and binding it to 'p'.
// This way ownership of 'Point' remains with 'y' and 'y' is still usable after the match statement.