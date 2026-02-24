pub struct Logger;

impl Logger {
    pub fn log_message(message: &str) {
        print!("[LOG]: {message} \n");
    }
}

fn main() {
    Logger::log_message("This is a test log.");
}
