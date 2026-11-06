fn main() {
    let shelves: [i32; 5] = [3, 1, 4, 1, 5];
    let index: usize = "9".parse().unwrap();
    println!("{}", shelves[index]);
}
