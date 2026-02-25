#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    debug_example();
}

// Example function
pub fn debug_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{:?}", person);

    let point = Point { x: 5.0, y: -3.2 };
    println!("{:?}", point);

    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", rectangle);
}
