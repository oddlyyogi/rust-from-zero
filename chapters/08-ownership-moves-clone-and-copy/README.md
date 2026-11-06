# 08 - Ownership: Moves, Clone and Copy

Why does Rust stop you using a value after you hand it over? Two people holding tickets for one warehouse shelf show what a double free is. We then read the three ownership rules, see a real move and the E0382 error, make real copies with clone, find out why numbers are Copy, pass values to functions and get them back, and drop values early.

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
    {
        let label = String::from("Parcel");
        println!("{label}");
    }
    println!("{label}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel");
    let driver = label;
    println!("{label}");
    println!("{driver}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    let driver = label;
    println!("{driver}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    let driver = label.clone();
    println!("{label}");
    println!("{driver}");
    println!("{:p}", label.as_ptr());
    println!("{:p}", driver.as_ptr());
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let other = weight;
    println!("{weight} {other}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel");
    ship(label);
    println!("{label}");
}

fn ship(label: String) {
    println!("Shipping {label}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    ship(label);
    let kilos = 12;
    weigh(kilos);
    weigh(kilos);
}

fn ship(label: String) {
    println!("Shipping {label}");
}

fn weigh(kilos: u32) {
    println!("{kilos} kg");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel");
    let label = stamp(label);
    println!("{label}");
}

fn stamp(mut label: String) -> String {
    label.push_str(", stamped");
    label
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel");
    drop(label);
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut name = String::from("first");
    name = String::from("second");
    println!("{name}");
}
```

## Sources

- Ownership rules, moves, clone, Copy, ownership and functions: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
- std::mem::drop: https://doc.rust-lang.org/stable/std/mem/fn.drop.html
