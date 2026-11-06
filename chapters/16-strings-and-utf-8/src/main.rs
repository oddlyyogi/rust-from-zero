fn main() {
    let owned = String::from("HP-1");
    println!("{}", is_ours(&owned));
    println!("{}", is_ours("XX-9"));
    println!("{}", label(&owned, "Oslo"));
}

fn is_ours(code: &str) -> bool {
    code.starts_with("HP-")
}

fn label(tracking: &str, city: &str) -> String {
    format!("{tracking} to {city}")
}
