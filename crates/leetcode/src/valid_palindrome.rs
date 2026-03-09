/// LeetCode #125 - Valid Palindrome
/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters 
/// and removing all non-alphanumeric characters, it reads the same forward and backward. 
/// Alphanumeric characters include letters and numbers.
///
/// Example 1:
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        if chars.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}
