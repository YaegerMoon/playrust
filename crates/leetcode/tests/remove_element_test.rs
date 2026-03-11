use leetcode::remove_element::Solution;

#[test]
fn test_remove_element_example_1() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let k = Solution::remove_element(&mut nums, val);
    assert_eq!(k, 2);
    // The first k elements should be non-val
    nums[0..k as usize].sort();
    assert_eq!(&nums[0..k as usize], &[2, 2]);
}

#[test]
fn test_remove_element_example_2() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let k = Solution::remove_element(&mut nums, val);
    assert_eq!(k, 5);
    // The first k elements should be non-val
    let mut actual = nums[0..k as usize].to_vec();
    actual.sort();
    let mut expected = vec![0, 0, 1, 3, 4];
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn test_remove_element_no_matches() {
    let mut nums = vec![1, 2, 3, 4, 5];
    let val = 6;
    let k = Solution::remove_element(&mut nums, val);
    assert_eq!(k, 5);
    let mut actual = nums[0..k as usize].to_vec();
    actual.sort();
    assert_eq!(actual, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_remove_element_all_matches() {
    let mut nums = vec![2, 2, 2];
    let val = 2;
    // Note: The current implementation might have issues with underflow or logic if not careful
    // but I am just providing the tests as requested.
    let k = Solution::remove_element(&mut nums, val);
    assert_eq!(k, 0);
}
