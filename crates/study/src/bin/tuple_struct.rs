pub struct Rectangle(f32, f32); // 1. Finish the struct

fn main() {
    let rectangle = Rectangle(33.3, 23.1);

    println!("Area = {}", area(&rectangle))
}

pub fn area(rect: &Rectangle) -> f32 {
    // 2. Implement the area function
    rect.0 * rect.1
}
