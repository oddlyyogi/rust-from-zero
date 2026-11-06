# 04 - Numbers, Characters, Tuples and Arrays

How big a number do you need? We start with a three-wheel mechanical counter that rolls back to zero, then meet Rust's integer types, a real overflow panic in a debug build and a real wrap in a release build, whole number division, casts with as, booleans and four-byte chars, tuples, arrays, and the runtime check that stops you reading past the end of an array.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo run
cargo run --release
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
    let weight: u32 = 12;
    let temperature: i32 = -5;
    let parcels = 3;
    let small: u8 = 255;
    println!("{weight} {temperature} {parcels} {small}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weight: u32 = 12;
    let temperature: i32 = -5;
    let parcels = 3;
    let small: u8 = 256;
    println!("{weight} {temperature} {parcels} {small}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let count: u8 = "255".parse().unwrap();
    let next = count + 1;
    println!("{next}");
}
```

Compiles and runs:

```rust
fn main() {
    let price = 4.5;
    let half = 7 / 2;
    println!("{half}");
    println!("{price}");
}
```

Compiles and runs:

```rust
fn main() {
    let price = 4.5;
    let half = 7.0 / 2.0;
    println!("{half}");
    println!("{price}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let price = 4.5;
    let half = 7.0 / 2.0;
    println!("{half}");
    println!("{price}");
    let parcels = 5;
    let total = price * parcels;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let price = 4.5;
    let half = 7.0 / 2.0;
    println!("{half}");
    println!("{price}");
    let parcels = 5;
    let total = price * parcels as f64;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let fragile = true;
    let grade = 'A';
    let parcel_icon = '📦';
    println!("{fragile} {grade} {parcel_icon}");
    println!("{}", std::mem::size_of::<char>());
}
```

Compiles and runs:

```rust
fn main() {
    let parcel = (12, 4.5, 'A');
    println!("{} {}", parcel.0, parcel.2);
}
```

Compiles and runs:

```rust
fn main() {
    let parcel = (12, 4.5, 'A');
    println!("{} {}", parcel.0, parcel.2);
    let (weight, price, grade) = parcel;
    println!("{weight} {price} {grade}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let parcel = (12, 4.5, 'A');
    println!("{} {}", parcel.0, parcel.2);
    let (weight, price, grade) = parcel;
    println!("{weight} {price} {grade}");
    println!("{}", parcel.3);
}
```

Compiles and runs:

```rust
fn main() {
    let shelves = [3, 1, 4, 1, 5];
    println!("{}", shelves[0]);
    println!("{}", shelves.len());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let shelves: [i32; 5] = [3, 1, 4, 1, 5];
    let index: usize = "9".parse().unwrap();
    println!("{}", shelves[index]);
}
```

## Sources

- Scalar and compound types, overflow in debug and release, invalid array access: https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
- Casting between number types with as: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
