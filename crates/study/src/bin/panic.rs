/**
* Your Task
Write a function, get_database_url, that retrieves the value of the DATABASE_URL environment variable and validates it. The function should behave as follows:

If the DATABASE_URL variable is set and starts with postgresql://, return its value as a String.
If the DATABASE_URL variable is not set, the function should terminate the program by calling panic!
with the exact message "DATABASE_URL environment variable is not set.".
If the DATABASE_URL variable is set but does not start with postgresql://, the function should terminate the program by calling panic!
with the message "DATABASE_URL must start with 'postgresql://'".

*/

pub fn get_database_url() -> String {
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.");
    if !db_url.starts_with("postgresql://") {
        panic!("DATABASE_URL must start with 'postgresql://'");
    }
    return db_url;
}

/// Example usage
pub fn main() {
    unsafe {
        std::env::set_var("DATABASE_URL", "postgresql://localhost");

        let db_url = get_database_url();
        println!("Database URL: {}", db_url);

        std::env::remove_var("DATABASE_URL"); // Missing variable scenario
        get_database_url();

        std::env::set_var("DATABASE_URL", "mysql://localhost"); // Invalid prefix scenario
        get_database_url();
    }
}
