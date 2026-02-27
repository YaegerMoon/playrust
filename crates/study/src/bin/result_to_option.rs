use std::{fs::File, io::Read};

pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let mut file = File::open(file_path).ok()?;
    let mut content = String::new();
    file.read_to_string(&mut content).ok()?;
    Some(content)
}

// Example usage
pub fn main() {
    let file_path = "./number.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
