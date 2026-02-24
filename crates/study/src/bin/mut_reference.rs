fn main() {
    let mut age = 31;

    print!("before={} \n", age);

    add_age(&mut age);

    print!("after={} \n", age);
}

fn add_age(age: &mut i32) {
    *age += 1
}
