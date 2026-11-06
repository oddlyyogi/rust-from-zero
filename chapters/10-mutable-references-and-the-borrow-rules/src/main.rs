fn main() {
    let mut weights = vec![12, 35];
    let mut extra = vec![];
    for weight in &weights {
        extra.push(*weight);
    }
    weights.extend(extra);
    println!("{weights:?}");
}
