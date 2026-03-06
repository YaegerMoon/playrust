use std::collections::HashSet;

// 1. Finish the function
// pub fn unique_items ...
fn unique_items<I, T>(items: I) -> Vec<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut result = items
        .filter_map(|s| {
            let trimed = s.as_ref().trim();
            if trimed.is_empty() {
                return None;
            } else {
                Some(trimed.to_string())
            }
        })
        .collect::<HashSet<_>>() // 중복 제거
        .into_iter()
        .collect::<Vec<String>>();
    result.sort();
    result
}
/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
