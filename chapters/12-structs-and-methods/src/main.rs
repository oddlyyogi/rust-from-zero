struct Kilos(u32);
struct Pence(u32);

fn charge(weight: Kilos, rate: Pence) -> u32 {
    weight.0 * rate.0
}

fn main() {
    println!("{}", charge(Kilos(12), Pence(50)));
}
