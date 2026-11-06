fn main() {
    show("Parcel 7");
    let label = String::from("Parcel");
    show(&label);
}

fn show(label: &str) {
    println!("{label}");
}
