pub struct Book {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: u32,
}

impl Book {
    // 2. Define the `new` associated function

    pub fn new(title: &str, author: &str, year: i32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
            likes: 0,
        }
    }
}

fn main() {
    let book = Book::new("ASOIAF", "George R. R. Martin", 1996);

    assert_eq!(book.title, "ASOIAF");
    assert_eq!(book.author, "George R. R. Martin");
    assert_eq!(book.year, 1996);
    assert_eq!(book.likes, 0);
}
