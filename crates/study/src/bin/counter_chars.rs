fn main() {
    let len = count_characters("hello world");

    println!("len={}", len)
}

pub fn count_characters(s: &str) -> u32 {
    // Count the number of characters in the string
    s.chars().count() as u32
}
