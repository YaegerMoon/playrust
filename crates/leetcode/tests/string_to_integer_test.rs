use leetcode::string_to_integer::Solution;

#[test]
fn test_my_atoi_basic() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
}

#[test]
fn test_my_atoi_with_sign() {
    assert_eq!(Solution::my_atoi(" -42".to_string()), -42);
}

#[test]
fn test_my_atoi_with_words() {
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
}

#[test]
fn test_my_atoi_leading_whitespace() {
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
}

#[test]
fn test_my_atoi_out_of_range() {
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
}

#[test]
fn test_my_atoi_plus_sign() {
    assert_eq!(Solution::my_atoi("+1".to_string()), 1);
}

#[test]
fn test_my_atoi_no_digits() {
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
}
