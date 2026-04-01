use std::{num::ParseIntError, str::FromStr};

/// Parse a string into an i32, returning a descriptive error message on failure.
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| "Not a valid integer".to_string())
}

/// Parse common boolean representations (case-insensitive).
/// Accepts: "true", "false", "1", "0", "yes", "no"
pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s.trim().to_lowercase().as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        "1" => Ok(true),
        "0" => Ok(false),
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err("Not a valid boolean".to_string()),
    }
}

/// Parse a "key=value" string into a tuple.
pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    s.split_once('=')
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .ok_or("Not a valid key=value pair".to_string())
}

/// A color represented by red, green, and blue components.
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .split(',')
            .map(|v| v.trim().parse::<u8>())
            .collect::<Result<Vec<u8>, ParseIntError>>()
            .map_err(|_| "Not a valid color".to_string())
            .and_then(|parts| {
                if parts.len() == 3 {
                    Ok(Color {
                        r: parts[0],
                        g: parts[1],
                        b: parts[2],
                    })
                } else {
                    Err("Not a valid color".to_string())
                }
            })
    }
}

/// Parse a delimited list of values into a Vec.
pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    s.trim()
        .split(delimiter)
        .map(|v| v.trim().parse::<T>())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|_| "Not a valid list".to_string())
}

pub fn main() {
    // Demonstrate parse_int
    println!("Parsing integers:");
    println!("  '42' -> {:?}", parse_int("42"));
    println!("  '-17' -> {:?}", parse_int("-17"));
    println!("  'abc' -> {:?}", parse_int("abc"));

    // Demonstrate parse_bool
    println!("\nParsing booleans:");
    println!("  'true' -> {:?}", parse_bool("true"));
    println!("  'YES' -> {:?}", parse_bool("YES"));
    println!("  '0' -> {:?}", parse_bool("0"));
    println!("  'maybe' -> {:?}", parse_bool("maybe"));

    // Demonstrate parse_key_value
    println!("\nParsing key=value pairs:");
    println!("  'name=Alice' -> {:?}", parse_key_value("name=Alice"));
    println!("  'count=42' -> {:?}", parse_key_value("count=42"));
    println!("  'invalid' -> {:?}", parse_key_value("invalid"));

    // Demonstrate Color parsing
    println!("\nParsing colors:");
    let color: Result<Color, _> = "255,128,0".parse();
    println!("  '255,128,0' -> {:?}", color);

    // Demonstrate parse_list
    println!("\nParsing lists:");
    println!("  '1,2,3' as i32 -> {:?}", parse_list::<i32>("1,2,3", ','));
}
