pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Your code here...
    numbers.iter().find(|&n| n % 2 == 0).copied()
}

// Example usage
pub fn main() {
    let nums1 = vec![1, 3, 5, 8];
    let nums2 = vec![1, 3, 5];
    let nums3: Vec<i32> = vec![];

    println!("{:?}", find_first_even(&nums1)); // Output: Some(8)
    println!("{:?}", find_first_even(&nums2)); // Output: None
    println!("{:?}", find_first_even(&nums3)); // Output: None
}
