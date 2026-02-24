pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here

    for &idx in indices.iter() {
        if let Some(e) = slice.get_mut(idx) {
            *e = value
        }
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    update_slice(&mut data, &[1, 3, 4], 7);
    assert_eq!(data, vec![1, 7, 3, 7, 7]);

    let mut data = vec![10, 20, 30];
    // Index 5 is out of bounds
    update_slice(&mut data, &[2, 5], 100);
    assert_eq!(data, vec![10, 20, 100]);
}
