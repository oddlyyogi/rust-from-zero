use std::collections::BTreeMap;

fn main() {
    let mut weights = BTreeMap::new();
    weights.insert("HP-3", 8);
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    println!("{weights:?}");
}
