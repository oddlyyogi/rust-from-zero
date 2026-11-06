# 03 - Variables, Mutability and Shadowing

Why are Rust variables locked by default? We compare labels written in pen and pencil at the Harbor Parcels depot, then meet the real compiler errors for changing an immutable variable and for changing a type, use mut, cover an old label with shadowing, watch shadowing end with a block, and set company-wide facts with const.

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
    let parcels = 3;
    println!("{parcels}");
    parcels = 4;
    println!("{parcels}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut parcels = 3;
    println!("{parcels}");
    parcels = 4;
    println!("{parcels}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut parcels = 3;
    println!("{parcels}");
    parcels = 4;
    println!("{parcels}");
    let mut depot = 1;
    println!("{depot}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut parcels = 3;
    println!("{parcels}");
    parcels = 4;
    println!("{parcels}");
    let depot = 1;
    println!("{depot}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = "  parcel 7  ";
    let label = label.trim();
    let label = label.len();
    println!("{label}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("{spaces}");
}
```

Compiles and runs:

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 5;
    {
        let weight = weight * 2;
        println!("Inside: {weight}");
    }
    println!("After: {weight}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
const MAX_WEIGHT = 30;
fn main() {
}
```

Compiles and runs:

```rust
const MAX_WEIGHT: u32 = 30;
const SECONDS_PER_DAY: u32 = 60 * 60 * 24;
fn main() {
    println!("{MAX_WEIGHT}");
    println!("{SECONDS_PER_DAY}");
}
```

Compiles and runs:

```rust
fn main() {
    let parcelCount = 2;
    println!("{parcelCount}");
}
```

Compiles and runs:

```rust
fn main() {
    let parcel_count = 2;
    println!("{parcel_count}");
    let unused = 9;
}
```

## Sources

- Variables and mutability, shadowing, constants: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
- Naming conventions (snake case for variables, screaming snake case for constants): https://doc.rust-lang.org/stable/style-guide/advice.html
