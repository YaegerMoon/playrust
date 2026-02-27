// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

use std::fmt;
impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => write!(f, "Invalid input"),
            ParsePercentageError::OutOfRange => write!(f, "Value out of range (0-100)"),
        }
    }
}

// 2. Implement the `Error` trait
use std::error;
impl error::Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<u8>() {
        Ok(v) if v <= 100 => Ok(v),
        Ok(_) => Err(ParsePercentageError::OutOfRange),
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
