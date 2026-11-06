fn main() {
    show(12);
    show(2.5);
    show("HP-1");
    show(String::from("HP-2"));
}

fn show<T>(_value: T) {
    println!("{}", std::any::type_name::<T>());
}
