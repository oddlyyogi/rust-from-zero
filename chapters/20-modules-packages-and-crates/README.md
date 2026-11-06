# 20 - Modules, Packages and Crates

How do you organize a Rust program as it grows? A depot divided into rooms with public doors explains modules. We hit the real privacy errors, keep helpers and struct fields private, move modules into their own files and folders, use crate paths and use, and finally sort out what the words module, crate and package mean by splitting our program into a library and a binary.

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
mod pricing {
    fn cost(weight: u32) -> u32 {
        weight * 2
    }
}

fn main() {
    println!("{}", pricing::cost(12));
}
```

Compiles and runs:

```rust
mod pricing {
    pub fn cost(weight: u32) -> u32 {
        weight * 2
    }
}

fn main() {
    println!("{}", pricing::cost(12));
}
```

Compiles and runs:

```rust
mod pricing {
    fn base_fee() -> u32 {
        5
    }

    pub fn cost(weight: u32) -> u32 {
        base_fee() + weight * 2
    }
}

fn main() {
    println!("{}", pricing::cost(12));
}
```

Does not compile, on purpose, so we can read the error:

```rust
mod pricing {
    fn base_fee() -> u32 {
        5
    }

    pub fn cost(weight: u32) -> u32 {
        base_fee() + weight * 2
    }
}

fn main() {
    println!("{}", pricing::cost(12));
    println!("{}", pricing::base_fee());
}
```

Does not compile, on purpose, so we can read the error:

```rust
mod depot {
    pub struct Parcel {
        pub code: String,
        weight: u32,
    }
}

fn main() {
    let p = depot::Parcel { code: String::from("HP-1"), weight: 12 };
}
```

Compiles and runs:

```rust
mod depot {
    pub struct Parcel {
        pub code: String,
        weight: u32,
    }

    impl Parcel {
        pub fn new(code: &str, weight: u32) -> Parcel {
            Parcel { code: code.to_string(), weight }
        }
        pub fn weight(&self) -> u32 {
            self.weight
        }
    }
}

fn main() {
    let p = depot::Parcel::new("HP-1", 12);
    println!("{} {}", p.code, p.weight());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    println!("{}", pricing::cost(12));
}
```

Compiles and runs:

```rust
mod pricing;

fn main() {
    println!("{}", pricing::cost(12));
}
```

Compiles and runs:

```rust
mod pricing;
mod tracking;
use pricing::cost;

fn main() {
    println!("{}", cost(12));
    println!("{}", tracking::label("HP-1", 12));
}
```

Compiles and runs:

```rust
mod depot;
mod pricing;
mod tracking;
use pricing::cost;

fn main() {
    println!("{}", cost(12));
    println!("{}", tracking::label("HP-1", 12));
    println!("{}", depot::lockers::locker_for(12));
}
```

Compiles and runs:

```rust
use harbor_parcels::depot::lockers::locker_for;
use harbor_parcels::tracking::label;

fn main() {
    println!("{}", label("HP-1", 12));
    println!("{}", locker_for(12));
}
```

Does not compile, on purpose, so we can read the error:

```rust
use harbor_parcels::depot::lockers::locker_for;
use harbor_parcels::tracking::label;

fn main() {
    println!("{}", label("HP-1", 12));
    println!("{}", locker_for(12));
    println!("{}", harbor_parcels::pricing::cost(12));
}
```

Compiles and runs:

```rust
use harbor_parcels::depot::lockers::locker_for;
use harbor_parcels::tracking::label;

fn main() {
    println!("{}", label("HP-1", 12));
    println!("{}", locker_for(12));
}
```

## Sources

- Packages, crates and modules: https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Packages and crates, at most one library crate, src/lib.rs and src/main.rs conventions: https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html
- Privacy, pub and paths: https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
- Struct fields are private by default: https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
- use keyword: https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
- Separating modules into files, older mod.rs style: https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html
