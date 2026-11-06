# 02 - Cargo: Build, Check and Add Your First Crate

What does Cargo actually do for you? We picture a depot manager ordering a label printer and all the parts it needs, then use Cargo for real: cargo check, cargo add with the rand crate, generating tracking numbers for Harbor Parcels, reading Cargo.lock and cargo tree, cleaning the target folder, and keeping code healthy with cargo fmt and cargo clippy.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo check
cargo add rand
cargo run
rustc src/main.rs
cargo tree
cargo clean
cargo build
cargo fmt
cargo clippy
cargo doc
cargo remove rand
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
    let parcels = 3;
    println!("Parcels today: {parcels}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let parcels = 3;
    println!("Parcels today: {parcel}");
}
```

Compiles and runs:

```rust
fn main() {
    let parcels = 3;
    println!("Parcels today: {parcels}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let tracking = rand::random_range(1000..=9999);
    println!("Tracking number: HP-{tracking}");
}
```

Compiles and runs:

```rust
fn main() {
    let parcels = 3;
    let total = parcels + 0;
    println!("Total: {total}");
}
```

Compiles and runs:

```rust
fn main() {
    let parcels = 3;
    let total = parcels;
    println!("Total: {total}");
}
```

## Sources

- cargo check skips the final code generation step and is faster than cargo build; some errors only appear during code generation: https://doc.rust-lang.org/cargo/commands/cargo-check.html
- Cargo.lock is maintained by Cargo and should not be edited by hand: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
- How version requirements such as 0.10.3 are read: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
