fn main() {
    let s1 = String::from("yaeger");

    say_hello(&s1);

    println!("Goodbye {}", s1)
}

fn say_hello(name: &String) {
    println!("Hello {}", name)
}
