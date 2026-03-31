/// Remove leading and trailing whitespace, then convert to lowercase.
pub fn clean_string(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Check if the text contains the given word (case-insensitive).
pub fn contains_word(text: &str, word: &str) -> bool {
    let p = word.to_lowercase();
    text.to_lowercase().contains(&p)
}

/// Replace all occurrences of `from` with `to`.
pub fn replace_word(text: &str, from: &str, to: &str) -> String {
    text.replace(from, to)
}

/// Split the string by the delimiter and trim each part.
pub fn split_and_trim(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(|s| s.trim().to_string()).collect()
}

/// Replace all sequences of whitespace with a single space, and trim.
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
