// 1. Finish the struct definition
pub struct MutableTextFinder<'a> {
    text: &'a mut String,
}

// 2. Implement the methods for the struct
impl<'a> MutableTextFinder<'a> {
    pub fn new(s: &'a mut String) -> Self {
        Self { text: s }
    }

    pub fn find_first(&self, input_text: &str) -> Option<&str> {
        self.text.lines().find(|line| line.contains(input_text))
    }

    pub fn replace_lines(&mut self, target: &str, replace: &str) {
        *self.text = self
            .text
            .lines()
            .map(|line| if line.contains(target) { replace } else { line })
            .collect::<Vec<_>>()
            .join("\n");
    }

    pub fn get_text(&self) -> &str {
        self.text
    }
}

// Example usage
pub fn main() {
    let mut text = String::from("Rust is awesome\nLearning Rust\nFun with Rustaceans");
    let mut finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is awesome")

    finder.replace_lines("Rust", "Programming in Rust");
    println!("{}", finder.get_text()); // Should print the modified text
}
