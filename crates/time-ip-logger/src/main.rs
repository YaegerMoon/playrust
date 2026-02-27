use chrono::Local;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(10));

    println!("Starting Time logger (every 10 seconds)...");

    loop {
        interval.tick().await;

        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Current Time: {}", now);
    }
}
