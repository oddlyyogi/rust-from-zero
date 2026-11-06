fn main() {
    println!("{:?}", find_weight("HP-1"));
    println!("{:?}", find_weight("HP-9"));
    let weights = [12, 35];
    println!("{:?}", weights.first());
    println!("{:?}", weights.get(9));
    match find_weight("HP-9") {
        Some(w) => println!("{w} kg"),
        None => println!("Not found"),
    }
}

fn find_weight(tracking: &str) -> Option<u32> {
    if tracking == "HP-1" { Some(12) } else { None }
}
