use std::collections::HashMap;

pub struct Student {
    // 1. Define the fields
    name: String,
    grade: Vec<u8>,
}

pub struct StudentGrades {
    // 2. Define the fields
    students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        // Implement here
        self.students.insert(
            String::from(name),
            Student {
                name: String::from(name),
                grade: Vec::new(),
            },
        );
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        // Implement here
        if let Some(student) = self.students.get_mut(&String::from(name)) {
            student.grade.push(grade);
        }
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
        &self.students.get(name).unwrap().grade
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
