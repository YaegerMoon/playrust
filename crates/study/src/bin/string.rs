fn main() {
    let st_arr = "hello";
    let end = '!';
    let mut str = st_arr.to_string();
    let alt_str = String::from("world");

    str = format!("{} {}", str, alt_str);

    str.push_str("yaeger");

    str.push(end);

    println!("{}", str);

    // Slicing str
    let test_str = "yeager is man";

    let part_of_str = &test_str[0..6];
    let sliced: String = test_str[10..=12].into();

    for c in part_of_str.chars() {
        println!("{}", c)
    }

    println!("\n");

    for c in sliced.chars() {
        println!("{}", c)
    }
}
