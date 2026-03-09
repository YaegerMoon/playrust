use leetcode::valid_palindrome::Solution;

#[test]
fn test_valid_palindrome() {
    assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!Solution::is_palindrome("race a car".to_string()));
    assert!(Solution::is_palindrome(" ".to_string()));
    assert!(Solution::is_palindrome("a.".to_string()));
    assert!(Solution::is_palindrome(".,".to_string()));
}
