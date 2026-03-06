pub struct Animal {
    pub name: String,
    pub age: u8,
}

impl Animal {
    pub fn new(name: &str, age: u8) -> Box<Self> {
        Box::new(Self {
            name: name.into(),
            age,
        })
    }
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    // Your code here
    Animal::new(name, age)
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    // Your code here
    let animal = *animal;
    (animal.name, animal.age)
}

// Example usage
pub fn main() {
    let animal = create_animal("Leo", 5);
    let (name, age) = access_animal(animal);
    println!("Animal's name: {}, age: {}", name, age);
}
