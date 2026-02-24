fn main() {
    let a = "That’s not what it looks like,".to_string();

    let length = calculate_length(&a);

    println!("length = {}", length)
}

pub fn calculate_length(s: &String) -> usize {
    s.len()
}
