pub enum TrafficLight {
    // Define enum variants here
    Red,
    Yellow,
    Green,
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    assert_eq!(light_action(&red), "Stop");
    assert_eq!(light_action(&yellow), "Caution");
    assert_eq!(light_action(&green), "Go");
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}
