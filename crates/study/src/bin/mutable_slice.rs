pub fn transform_even_odd(slice: &mut [i32]) {
    for num in slice.iter_mut() {
        if *num % 2 == 0 {
            *num *= 2;
        } else {
            *num -= 1;
        }
    }
}

fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    transform_even_odd(&mut numbers);

    // Odd numbers reduced by 1
    // Even numbers doubled
    assert_eq!(numbers, [0, 4, 2, 8, 4]);

    let mut numbers = [10, 15, 20];
    transform_even_odd(&mut numbers);

    // 10 -> 20, 15 -> 14, 20 -> 40
    assert_eq!(numbers, [20, 14, 40]);
}
