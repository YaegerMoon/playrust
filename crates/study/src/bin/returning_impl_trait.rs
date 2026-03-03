/**
* Write a function named filter_starts_with that:

Accepts two arguments:
A slice of String
A &str keyword
Returns an iterator that filters and yields only the strings starting with the given keyword.
*/

// Finish the function
pub fn filter_starts_with<'a>(
    slice: &'a [String],
    keyword: &'a str,
) -> impl Iterator<Item = &'a String> + 'a {
    slice.iter().filter(move |st| st.starts_with(keyword))
}

// Example usage
pub fn main() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let filtered: Vec<&String> = filter_starts_with(&input, "ap").collect();
    println!("{:?}", filtered); // Expected output: ["apple", "apricot"]
}
