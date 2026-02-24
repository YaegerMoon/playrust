fn main() {
    println!("3+4 = {}", addition(3, 4));
    println!("8-3 = {}", subtract(8, 3));
    println!("4*5 = {}", multiply(4, 5));
}

pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
