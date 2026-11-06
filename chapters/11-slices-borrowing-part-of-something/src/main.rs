fn main() {
    let mut weights = [12, 35, 8, 20];
    add_packaging(&mut weights[1..3]);
    println!("{weights:?}");
}

fn add_packaging(weights: &mut [i32]) {
    for weight in weights {
        *weight += 1;
    }
}
