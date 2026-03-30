/// Convert a string slice to an owned String.
pub fn to_owned_string(s: &str) -> String {
    // TODO: Convert the string slice to an owned String
    s.to_string()
}

/// Count the number of Unicode characters in a string.
pub fn count_chars(s: &str) -> usize {
    // TODO: Count the number of characters
    s.chars().count()
}

/// Count the number of bytes in a string.
pub fn count_bytes(s: &str) -> usize {
    // TODO: Count the number of bytes
    s.bytes().count()
}

/// Check if a string contains only ASCII characters.
pub fn is_ascii_only(s: &str) -> bool {
    // TODO: Check if the string contains only ASCII characters
    s.is_ascii()
}

/// Return the first character of a string, or None if the string is empty.
pub fn first_char(s: &str) -> Option<char> {
    // TODO: Return the first character or None if empty
    s.chars().next()
}
