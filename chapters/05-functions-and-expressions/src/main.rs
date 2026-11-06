fn main() {
    let (fee, tax) = split(100);
    println!("Fee: {fee}, tax: {tax}");
}

/// Splits a total into a fee and a tax.
fn split(total: u32) -> (u32, u32) {
    // Whole number division, so the tax rounds down.
    let tax = total / 10;
    (total - tax, tax)
}
