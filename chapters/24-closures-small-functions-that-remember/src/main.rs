fn main() {
    let mut parcels = vec![("HP-1", 12), ("HP-2", 35), ("HP-3", 8)];
    parcels.sort_by_key(|p| p.1);
    println!("{parcels:?}");
    parcels.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{parcels:?}");
    let limit = 10;
    parcels.retain(|p| p.1 > limit);
    println!("{parcels:?}");
    let name: Option<&str> = None;
    println!("{}", name.unwrap_or_else(|| "nobody"));
}
