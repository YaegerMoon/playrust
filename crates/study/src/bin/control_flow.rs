pub fn main() {
    println!("{}", check_number_sign(32));
    println!("{}", check_number_sign(-33));
    println!("{}", check_number_sign(0));
}

pub fn check_number_sign(number: i32) -> String {
    return if number > 0 {
        String::from("positive")
    } else if number < 0 {
        String::from("negative")
    } else {
        String::from("zero")
    };
}
