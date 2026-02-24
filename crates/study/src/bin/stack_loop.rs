fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];

    while let Some(value) = stack.pop() {
        println!("value = {}", value)
    }
}
