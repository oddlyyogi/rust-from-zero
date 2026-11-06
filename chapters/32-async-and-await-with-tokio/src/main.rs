use std::thread;
use std::time::{Duration, Instant};

fn bad_check(name: &str) -> String {
    thread::sleep(Duration::from_millis(100));
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        tokio::task::spawn_blocking(|| bad_check("Oslo")),
        tokio::task::spawn_blocking(|| bad_check("Bergen")),
        tokio::task::spawn_blocking(|| bad_check("Tromso")),
    );
    println!("{}, {}, {}", a.unwrap(), b.unwrap(), c.unwrap());
    println!("{} ms", start.elapsed().as_millis());
}
