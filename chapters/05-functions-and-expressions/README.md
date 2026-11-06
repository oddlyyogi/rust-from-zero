# 05 - Functions and Expressions

What is a function really doing? We picture a service counter where you hand over a slip and get an answer back, then write Rust functions with parameters and return values, read real errors for a missing type and a wrong number of arguments, fall into the famous semicolon trap, see why blocks are expressions, and learn how clippy treats the return keyword.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo run
cargo clippy
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
    open_depot();
}

fn open_depot() {
    println!("Depot is open");
}
```

Compiles and runs:

```rust
fn main() {
    show_parcel(12);
}

fn show_parcel(weight: u32) {
    println!("Weight: {weight}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    show_parcel(12);
}

fn show_parcel(weight) {
    println!("Weight: {weight}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    show_parcel(12, 30);
}

fn show_parcel(weight: u32) {
    println!("Weight: {weight}");
}
```

Compiles and runs:

```rust
fn main() {
    let cost = price(12);
    println!("Cost: {cost}");
}

fn price(weight: u32) -> u32 {
    weight * 2
}
```

Compiles and runs:

```rust
fn main() {
    let cost = price(12);
    println!("Cost: {cost}");
    println!("Cost: {}", price(5));
    println!("Cost: {}", price(30));
}

fn price(weight: u32) -> u32 {
    weight * 2
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let cost = price(12);
    println!("Cost: {cost}");
    println!("Cost: {}", price(5));
    println!("Cost: {}", price(30));
}

fn price(weight: u32) -> u32 {
    weight * 2;
}
```

Compiles and runs:

```rust
fn main() {
    let total = {
        let base = 5;
        let extra = 2;
        base + extra
    };
    println!("Total: {total}");
}
```

Compiles and runs:

```rust
fn main() {
    let cost = price(12);
    println!("Cost: {cost}");
}

fn price(weight: u32) -> u32 {
    return weight * 2;
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let cost = price(12);
    println!("Cost: {cost}");
    println!("{rate}");
}

fn price(weight: u32) -> u32 {
    let rate = 2;
    weight * rate
}
```

Compiles and runs:

```rust
fn main() {
    let (fee, tax) = split(100);
    println!("Fee: {fee}, tax: {tax}");
}

fn split(total: u32) -> (u32, u32) {
    let tax = total / 10;
    (total - tax, tax)
}
```

Compiles and runs:

```rust
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
```

## Sources

- Functions, parameters, statements and expressions, return values: https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html
