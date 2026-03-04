// 1. Define the struct
pub struct TextFinder<'a> {
    text: &'a str,
}

impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        return TextFinder { text };
    }

    pub fn find_first(&self, text: &str) -> Option<&str> {
        self.text.lines().find(|line| line.contains(text))
    }

    pub fn find_many(&self, text: &str) -> Vec<&str> {
        self.text
            .lines()
            .filter(|line| line.contains(text))
            .collect()
    }
}

// 2. Implement the struct and define the methods

// Example usage
pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    dbg!(first);
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
