# 25 - Iterators: The Conveyor Belt

What really happens when you loop in Rust? A conveyor belt that only moves when the packer at the end asks for the next parcel explains iterators. We call next by hand, watch laziness in a real compiler warning, use map, filter and collect, untangle the double reference in filter, try sum, find, any and position, compare iter, iter_mut and into_iter, chain a pipeline, and build our own iterator.

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

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let mut belt = weights.iter();
    println!("{:?}", belt.next());
    println!("{:?}", belt.next());
    println!("{:?}", belt.next());
    println!("{:?}", belt.next());
}
```

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    weights.iter().map(|w| println!("{w}"));
}
```

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let doubled: Vec<u32> = weights.iter().map(|w| w * 2).collect();
    println!("{doubled:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let big: Vec<&u32> = weights.iter().filter(|w| w > 10).collect();
    println!("{big:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let big: Vec<&u32> = weights.iter().filter(|w| **w > 10).collect();
    println!("{big:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let big: Vec<u32> = weights.iter().copied().filter(|w| *w > 10).collect();
    println!("{big:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let total = weights.iter().sum();
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    let total: u32 = weights.iter().sum();
    println!("{total}");
    println!("{}", weights.iter().count());
    println!("{:?}", weights.iter().max());
    println!("{:?}", weights.iter().find(|w| **w > 20));
    println!("{}", weights.iter().any(|w| *w == 8));
    println!("{:?}", weights.iter().position(|w| *w == 35));
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut labels = vec![String::from("hp-1"), String::from("hp-2")];
    for l in labels.iter() {
        println!("{l}");
    }
    for l in labels.iter_mut() {
        l.push_str("!");
    }
    let upper: Vec<String> = labels.into_iter().map(|l| l.to_uppercase()).collect();
    println!("{upper:?}");
    println!("{labels:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut labels = vec![String::from("hp-1"), String::from("hp-2")];
    for l in labels.iter() {
        println!("{l}");
    }
    for l in labels.iter_mut() {
        l.push_str("!");
    }
    let upper: Vec<String> = labels.into_iter().map(|l| l.to_uppercase()).collect();
    println!("{upper:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let codes = ["HP-1", "HP-2", "HP-3"];
    let weights = [12, 35, 8];
    for (i, c) in codes.iter().enumerate() {
        println!("{i} {c}");
    }
    for (c, w) in codes.iter().zip(weights.iter()) {
        println!("{c} {w}");
    }
    for w in weights.iter().rev().take(2) {
        println!("{w}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let parcels = vec![("HP-1", 12), ("HP-2", 35), ("HP-3", 8)];
    let report: Vec<String> = parcels
        .iter()
        .filter(|p| p.1 > 10)
        .map(|p| format!("{} {} kg", p.0, p.1))
        .collect();
    println!("{report:?}");
}
```

Compiles and runs:

```rust
struct Countdown {
    left: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.left)
    }
}

fn main() {
    for n in (Countdown { left: 3 }) {
        println!("{n}");
    }
    let total: u32 = Countdown { left: 4 }.sum();
    println!("{total}");
}
```

## Sources

- Processing a series of items with iterators, lazy, map, filter, collect: https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
- Comparing performance: loops vs iterators, zero-cost abstraction: https://doc.rust-lang.org/stable/book/ch13-04-performance.html
- Iterator trait, provided methods: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
- filter passes a reference to the item: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter
- std::iter module, iter, iter_mut, into_iter: https://doc.rust-lang.org/stable/std/iter/index.html
