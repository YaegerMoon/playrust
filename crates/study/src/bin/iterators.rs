/**
* Your Task
Write the following two functions:

filter_even_numbers:
Takes an iterator of integers and filters out even numbers, returning a vector of the remaining odd numbers.

uppercase_strings:
Takes an iterator of string slices and converts each string to uppercase, returning a vector of the transformed strings.

Read the main() function carefully to understand how the functions will be used and what is expected from them.
*/

pub fn filter_even_numbers(iter: impl Iterator<Item = i32>) -> Vec<i32> {
    // 1. Finish the function
    iter.filter(|num| *num % 2 != 0).collect::<Vec<_>>()
}

// 2. Finish the function here
pub fn uppercase_strings<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<String> {
    iter.map(|s| s.to_uppercase()).collect::<Vec<String>>()
}

// Example usage
pub fn main() {
    // Filtering even numbers
    let numbers = vec![1, 2, 3, 4, 5];
    let odd_numbers = filter_even_numbers(numbers.into_iter());
    println!("{:?}", odd_numbers); // Should print: [1, 3, 5]

    // Converting strings to uppercase
    let words = vec!["hello", "world"];
    let uppercase_words = uppercase_strings(words.into_iter());
    println!("{:?}", uppercase_words); // Should print: ["HELLO", "WORLD"]
}
