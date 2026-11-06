use std::thread;

fn main() {
    let weights = vec![12, 35, 8];
    thread::scope(|s| {
        s.spawn(|| println!("{}", weights.iter().sum::<i32>()));
        s.spawn(|| println!("{:?}", weights.iter().max()));
    });
    println!("{weights:?}");
}
