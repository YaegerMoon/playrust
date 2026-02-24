fn main() {
    let mut stack = vec![1, 2, 3, 4];

    loop {
        let val = stack.pop();
        match val {
            Some(v) => println!("value = {}", v),
            None => break,
        }
    }
}
