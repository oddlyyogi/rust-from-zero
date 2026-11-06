# 01 - Why Rust, and Your First Program

Why do so many security bugs come down to memory, and how does Rust stop them before a program ever runs? We start with a stale locker ticket at a parcel depot, then install Rust, create the Harbor Parcels project with Cargo, read Hello World one line at a time, write our own program, and read two real compiler errors, including your first look at ownership.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
rustc --version
cargo --version
cargo new harbor_parcels
cargo run
cargo build
./target/debug/harbor_parcels
cargo build --release
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
fn main() {
    println!("Hello, world!");
}
```

Compiles and runs:

```rust
fn main() {
    let parcels = 3;
    println!("Harbor Parcels depot is open");
    println!("Parcels today: {parcels}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let parcels = 3;
    println!("Harbor Parcels depot is open");
    println!("Parcels today: {parcel}");
}
```

Compiles and runs:

```rust
fn main() {
    let parcels = 3;
    println!("Harbor Parcels depot is open");
    println!("Parcels today: {parcels}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let label = String::from("Parcel 1");
    let driver = label;
    println!("{label}");
    println!("{driver}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel 1");
    let driver = label;
    println!("{driver}");
}
```

## Sources

- Chromium: "Around 70% of our high severity security bugs are memory unsafety problems" https://www.chromium.org/Home/chromium-security/memory-safety/
- Android: memory safety vulnerabilities below 20% of total for the first time, Nov 13 2025 https://blog.google/security/rust-in-android-move-fast-fix-things/
- Ownership: "None of the features of ownership will slow down your program while it's running." https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
- Install commands and the Windows route: https://doc.rust-lang.org/stable/book/ch01-01-installation.html
