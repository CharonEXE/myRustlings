// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// IMPORTANT

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

    // my experiment here
    let maybe_value: Option<String> = Some(String::from("thing"));

    match maybe_value {
        Some(ref val) => {
            // Create a reference to the value inside Some
            println!("The value is: {}", val);
        }
        None => {
            println!("It's None");
        }
    }
    maybe_value;
}
