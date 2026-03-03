/**
* Your Task
You need to define a function that returns a Box<dyn Speakable> based on a condition.

TODO : Here is what you need to do:
1. Define the Speakable trait with a method speak that returns a String.
2. Define a struct Dog with two fields: name and breed, both of type String.
3. Implement the Speakable trait for Dog to return a string Woof.
4. Define a struct Robot with two fields: model and purpose, both of type String.
5. Implement the Speakable trait for Robot to return a string Beep boop.
6. Finish the function get_speaker that takes a &str parameter and returns either a Dog or a Robot based on the parameter.
The parameter can either be dog or robot.
*/

pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        "Beep boop".to_string()
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => {
            // Return a Dog instance here
            return Box::new(Dog {
                name: "Alro".to_string(),
                breed: "A".to_string(),
            });
        }
        "robot" => {
            // Return a Robot instance here
            return Box::new(Robot {
                model: "T1000".to_string(),
                purpose: "Genocide".to_string(),
            });
        }
        _ => panic!("Unknown speaker type"),
    }
}

// Example usage
pub fn main() {
    let dog_speaker = get_speaker("dog");
    println!("{}", dog_speaker.speak()); // Expected output: Woof

    let robot_speaker = get_speaker("robot");
    println!("{}", robot_speaker.speak()); // Expected output: Beep boop
}
