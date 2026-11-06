# 36 - Macros, unsafe and Shipping a Release: Final Project

Three last tools: a rubber stamp, a locked cage and a delivery van. We write macros with macro_rules, including any number of arguments, see what the unsafe keyword allows and what it doesn't, wrap unsafe code in a safe function, call C from Rust, measure debug against release, tune a release with LTO and strip, and finish a small command line tool with fmt, clippy, tests and a release build.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo run
cargo run --release
du -h target/debug/harbor_parcels target/release/harbor_parcels
cargo build --release
du -h target/release/harbor_parcels
cargo fmt
cargo clippy
cargo test
./target/release/harbor_parcels 1200 300 abc
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
macro_rules! log {
    ($msg:expr) => {
        println!("[depot] {}", $msg)
    };
}

fn main() {
    log!("Van loaded");
    log!(format!("{} parcels", 3));
}
```

Compiles and runs:

```rust
macro_rules! total {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}

fn main() {
    println!("{}", total!(1, 2, 3));
    println!("{}", total!(10));
    println!("{}", total!());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weight = 1200;
    let p = &raw const weight;
    println!("{}", *p);
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 1200;
    let p = &raw const weight;
    println!("{}", unsafe { *p });
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut weights = [100, 200, 300, 400];
    let a = &mut weights[0];
    let b = &mut weights[1];
    *a += 1;
    *b += 1;
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = [100, 200, 300, 400];
    let (left, right) = weights.split_at_mut(2);
    left[0] += 1;
    right[0] += 1;
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let n = unsafe { abs(-42) };
    println!("{n}");
}
```

Compiles and runs:

```rust
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut sum: u64 = 0;
    for i in 0..100_000_000u64 {
        sum = sum.wrapping_add(black_box(i));
    }
    println!("{sum}");
    println!("{} ms", start.elapsed().as_millis());
}
```

Compiles and runs:

```rust
use std::env;

fn price(grams: u32) -> u32 {
    5 + grams / 100
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("Usage: harbor_parcels GRAMS...");
        return;
    }
    for arg in args {
        match arg.parse() {
            Ok(grams) => println!("{grams} g costs {}", price(grams)),
            Err(_) => println!("{arg} isn't a number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_a_base_price() {
        assert_eq!(price(1200), 17);
    }
}
```

Compiles and runs:

```rust
use std::env;

fn price(grams: u32) -> u32 {
    5 + grams / 100
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: harbor_parcels GRAMS...");
        return;
    }
    for arg in args {
        match arg.parse() {
            Ok(grams) => println!("{grams} g costs {}", price(grams)),
            Err(_) => println!("{arg} isn't a number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_a_base_price() {
        assert_eq!(price(1200), 17);
    }
}
```

## Sources

- Macros in the Rust book: https://doc.rust-lang.org/stable/book/ch20-05-macros.html
- Unsafe Rust and its superpowers, abs extern example, split_at_mut: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html
- unsafe extern blocks in edition 2024: https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-extern.html
- Raw borrow operator &raw const: https://doc.rust-lang.org/reference/expressions/operator-expr.html
- std::hint::black_box: https://doc.rust-lang.org/std/hint/fn.black_box.html
- Cargo profiles, lto and strip: https://doc.rust-lang.org/cargo/reference/profiles.html
- Clippy len_zero: https://rust-lang.github.io/rust-clippy/master/index.html#len_zero
- cargo install and publish: https://doc.rust-lang.org/cargo/commands/cargo-install.html
