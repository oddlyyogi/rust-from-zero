# 10 - Mutable References and the Borrow Rules

Why can Rust let many parts of a program read a value, but only one change it? A worker repacking a parcel on a bench explains the rule. We write real mutable references, meet E0596, E0368, E0499 and E0502, see why a list refuses to grow while someone holds a reference to it, change items in a loop, and find out how the same rule stops data races.

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
    stamp(&mut label);
    println!("{label}");
}

fn stamp(label: &mut String) {
    label.push_str(", stamped");
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::from("Parcel");
    stamp(&mut label);
    println!("{label}");
}

fn stamp(label: &mut String) {
    label.push_str(", stamped");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut count = 0;
    add_parcel(&mut count);
    add_parcel(&mut count);
    println!("{count}");
}

fn add_parcel(count: &mut u32) {
    count += 1;
}
```

Compiles and runs:

```rust
fn main() {
    let mut count = 0;
    add_parcel(&mut count);
    add_parcel(&mut count);
    println!("{count}");
}

fn add_parcel(count: &mut u32) {
    *count += 1;
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut label = String::from("Parcel");
    let first = &mut label;
    let second = &mut label;
    first.push_str(" A");
    second.push_str(" B");
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::from("Parcel");
    let first = &mut label;
    first.push_str(" A");
    let second = &mut label;
    second.push_str(" B");
    println!("{label}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut label = String::from("Parcel");
    let reader = &label;
    label.push_str(" for Harbor Parcels on Quay Street");
    println!("{reader}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::from("Parcel");
    let reader = &label;
    println!("{reader}");
    label.push_str(" for Harbor Parcels on Quay Street");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut weights = vec![12, 35, 8];
    let first = &weights[0];
    weights.push(20);
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = vec![12, 35, 8];
    let first = weights[0];
    weights.push(20);
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = [12, 35, 8];
    for weight in &mut weights {
        *weight += 1;
    }
    for weight in &weights {
        println!("{weight}");
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut weights = vec![12, 35];
    for weight in &weights {
        weights.push(*weight);
    }
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = vec![12, 35];
    let mut extra = vec![];
    for weight in &weights {
        extra.push(*weight);
    }
    weights.extend(extra);
    println!("{weights:?}");
}
```

## Sources

- Mutable references and the rules of references: https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html
- Why a reference to a vector element blocks a push: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
- Data races and threads: https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html
