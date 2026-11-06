# 15 - Vec: A List That Can Grow

What do you use when you don't know how many items you will have? A row of shelf space that moves to a longer stretch when it fills up explains Vec. We watch real length and capacity grow, compare square brackets with get, see why inserting at the front is slow, hit the errors for moving out of a vector and looping without an ampersand, and work through a vector of parcels.

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
    let mut weights = Vec::new();
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = Vec::new();
    weights.push(12);
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = Vec::new();
    weights.push(12);
    weights.push(35);
    weights.push(8);
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = Vec::new();
    for i in 0..9 {
        weights.push(i);
        println!("{} {}", weights.len(), weights.capacity());
    }
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = Vec::with_capacity(10);
    for i in 0..9 {
        weights.push(i);
        println!("{} {}", weights.len(), weights.capacity());
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    println!("{}", weights[0]);
    println!("{:?}", weights.get(9));
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weights = vec![12, 35, 8];
    println!("{}", weights[0]);
    println!("{:?}", weights.get(9));
    println!("{}", weights[9]);
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = vec![12, 35, 8, 20];
    println!("{:?}", weights.pop());
    weights.insert(1, 99);
    weights.remove(0);
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = vec![12, 35, 8, 20];
    println!("{:?}", weights.pop());
    weights.insert(1, 99);
    weights.remove(0);
    println!("{weights:?}");
    weights.sort();
    println!("{}", weights.contains(&8));
    weights.retain(|w| *w > 10);
    println!("{weights:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut weights = vec![12, 35];
    for w in &weights { println!("{w}"); }
    for w in &mut weights { *w += 1; }
    for w in weights { println!("{w}"); }
    println!("{}", weights.len());
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = vec![12, 35];
    for w in &weights { println!("{w}"); }
    for w in &mut weights { *w += 1; }
    for w in &weights { println!("{w}"); }
    println!("{}", weights.len());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let labels = vec![String::from("HP-1"), String::from("HP-2")];
    let first = labels[0];
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let labels = vec![String::from("HP-1"), String::from("HP-2")];
    let first = &labels[0];
    println!("{first}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let items = vec![12, "fragile"];
}
```

Compiles and runs:

```rust
enum Item {
    Weight(u32),
    Note(String),
}

fn main() {
    let items = vec![Item::Weight(12), Item::Note(String::from("fragile"))];
    for item in &items {
        match item {
            Item::Weight(w) => println!("{w} kg"),
            Item::Note(n) => println!("{n}"),
        }
    }
}
```

Compiles and runs:

```rust
struct Parcel {
    tracking: String,
    weight: u32,
}

fn main() {
    let parcels = vec![
        Parcel { tracking: String::from("HP-1"), weight: 12 },
        Parcel { tracking: String::from("HP-2"), weight: 35 },
        Parcel { tracking: String::from("HP-3"), weight: 8 },
    ];
    let mut total = 0;
    for p in &parcels { total += p.weight; }
    println!("{total}");
    let mut heaviest = &parcels[0];
    for p in &parcels {
        if p.weight > heaviest.weight { heaviest = p; }
    }
    println!("{}", heaviest.tracking);
}
```

## Sources

- Storing lists with vectors: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
- Vec capacity and reallocation, growth strategy not guaranteed: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
