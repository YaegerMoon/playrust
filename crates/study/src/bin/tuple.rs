fn main() {
    let person: (&str, i32, u32) = ("yeager", 31, 176);

    print!(
        " name={} \n age={} \n height={} \n",
        person.0, person.1, person.2
    )
}
