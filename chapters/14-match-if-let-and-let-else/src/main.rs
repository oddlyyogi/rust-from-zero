fn main() {
    let mut belt = vec![12, 35, 8];
    while let Some(weight) = belt.pop() {
        println!("{weight}");
    }
    let weight = 12;
    let light = matches!(weight, 1..=30);
    println!("{light}");
}
