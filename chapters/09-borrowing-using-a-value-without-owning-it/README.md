# 09 - Borrowing: Using a Value Without Owning It

How can a function use a value without taking it? A customer reading a parcel label shows what borrowing is. We write real references with the ampersand, measure one, have three readers at once, and meet the errors that protect you: changing through a shared reference, moving a value while it is borrowed, a reference that outlives its owner, and returning a reference to something that is about to disappear.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo run
```

## Every version of `src/main.rs` from the video

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel");
    show(label);
    println!("{label}");
}

fn show(label: String) {
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    show(&label);
    println!("{label}");
}

fn show(label: &String) {
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    show(&label);
    println!("{label}");
    println!("{}", count(&label));
}

fn show(label: &String) {
    println!("{label}");
}

fn count(label: &String) -> usize {
    label.len()
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    show(&label);
    println!("{label}");
    println!("{}", count(&label));
    println!("{}", std::mem::size_of::<&String>());
}

fn show(label: &String) {
    println!("{label}");
}

fn count(label: &String) -> usize {
    label.len()
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    let first = &label;
    let second = &label;
    let third = &label;
    println!("{first}, {second}, {third}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel");
    stamp(&label);
}

fn stamp(label: &String) {
    label.push_str(", stamped");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel");
    let reader = &label;
    let driver = label;
    println!("{reader} {driver}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    let reader = &label;
    println!("{reader}");
    let driver = label;
    println!("{driver}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let reader = &weight;
    let doubled = *reader * 2;
    println!("{doubled}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weight = 12;
    let reader = &weight;
    let doubled = *reader * 2;
    println!("{doubled}");
    if reader == 12 {
        println!("Heavy enough");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let reader = &weight;
    let doubled = *reader * 2;
    println!("{doubled}");
    if *reader == 12 {
        println!("Heavy enough");
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let reader;
    {
        let label = String::from("Parcel");
        reader = &label;
    }
    println!("{reader}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    println!("{}", make_label());
}

fn make_label() -> &String {
    let label = String::from("Parcel");
    &label
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", make_label());
}

fn make_label() -> String {
    let label = String::from("Parcel");
    label
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    show("Parcel 7");
}

fn show(label: &String) {
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    show("Parcel 7");
}

fn show(label: &str) {
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    show("Parcel 7");
    let label = String::from("Parcel");
    show(&label);
}

fn show(label: &str) {
    println!("{label}");
}
```

## Sources

- References and borrowing, dangling references: https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html
