use std::fmt::Display;

/**
* Your Task
Define a generic function compare_and_display that:

Takes two parameters of the same type.
Returns the greater of the two using the PartialOrd trait.
Use trait bounds to ensure the function works only with types that implement both Display and PartialOrd.
*/

// TODO: Define the generic function `compare_and_display` with appropriate trait bounds.
pub fn compare_and_display<T>(value1: T, value2: T) -> T
where
    T: Display + PartialOrd,
{
    if value1 > value2 { value1 } else { value2 }
} // Complete the function definition

// Example usage
pub fn main() {
    let greater = compare_and_display(10, 20);
    println!("Greater value: {}", greater);

    let greater = compare_and_display("Apple", "Orange");
    println!("Greater value: {}", greater);
}
