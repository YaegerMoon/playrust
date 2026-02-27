use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

fn main() {
    let mut registry = HashMap::new();

    add_animal_to_section("Lion", "Savannah", &mut registry);
    assert_eq!(registry["Savannah"], vec!["Lion"]);

    add_animal_to_section("Elephant", "Savannah", &mut registry);
    assert_eq!(registry["Savannah"], vec!["Lion", "Elephant"]);

    add_animal_to_section("Penguin", "Arctic", &mut registry);
    assert_eq!(registry["Arctic"], vec!["Penguin"]);

    add_animal_to_section("Polar Bear", "Arctic", &mut registry);
    assert_eq!(registry["Arctic"], vec!["Penguin", "Polar Bear"]);

    add_animal_to_section("Monkey", "Jungle", &mut registry);
    assert_eq!(registry["Jungle"], vec!["Monkey"]);

    add_animal_to_section("Tiger", "Jungle", &mut registry);
    assert_eq!(registry["Jungle"], vec!["Monkey", "Tiger"]);

    add_animal_to_section("Shark", "Ocean", &mut registry);
    assert_eq!(registry["Ocean"], vec!["Shark"]);
}

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let animals = registry.entry(section.to_string()).or_default();
    if !animals.contains(&animal.to_string()) {
        animals.push(animal.to_string())
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let mut animals = registry.get(section).cloned().unwrap_or_default();
    animals.sort();
    animals
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut all_animals: Vec<String> = registry.values().flatten().cloned().collect();
    all_animals.sort();
    all_animals
}
