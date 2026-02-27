use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let file = File::open(file_path)?;
    let mut sum = 0i32;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line?.parse::<i32>() {
            Ok(num) => {
                sum += num;
            }
            Err(_) => return Err(io::Error::from(ErrorKind::InvalidData)),
        }
    }

    return Result::Ok(sum);
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
