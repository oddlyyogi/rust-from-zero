# 14 - match, if let and let else

How does Rust decide what to do with a value? A sorting frame with chutes shows how match works: the first pattern that fits wins, and the compiler checks that everything has somewhere to go. We match numbers and ranges, pull data out of enums, use guards, tuples and the at sign, take structs apart, then shorten things with if let, let else, while let and matches.

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
    let weight = 12;
    match weight {
        0 => println!("Empty"),
        1..=10 => println!("Light"),
        11..=30 => println!("Medium"),
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    match weight {
        0 => println!("Empty"),
        1..=10 => println!("Light"),
        11..=30 => println!("Medium"),
        _ => println!("Heavy"),
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let lane = match weight {
        0 => "Empty",
        1..=10 => "Light",
        11..=30 => "Medium",
        _ => "Heavy",
    };
    println!("{lane}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let lane = match weight {
        _ => "Heavy",
        0 => "Empty",
        1..=10 => "Light",
        11..=30 => "Medium",
    };
    println!("{lane}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let lane = match weight {
        0 => "Empty",
        1..=10 => "Light",
        11..=30 => "Medium",
        _ => "Heavy",
    };
    println!("{lane}");
}
```

Compiles and runs:

```rust
enum Status {
    Packed,
    InTransit(u32),
    Delivered(String),
}

fn main() {
    println!("{}", describe(&Status::InTransit(7)));
    println!("{}", describe(&Status::Delivered(String::from("Sam"))));
    println!("{}", describe(&Status::Packed));
}

fn describe(status: &Status) -> String {
    match status {
        Status::Packed => String::from("Waiting at the depot"),
        Status::InTransit(van) => format!("On van {van}"),
        Status::Delivered(name) => format!("Signed by {name}"),
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 35;
    let fragile = true;
    let lane = match (weight, fragile) {
        (_, true) => String::from("Careful lane"),
        (w, false) if w > 30 => String::from("Heavy lane"),
        _ => String::from("Normal lane"),
    };
    println!("{lane}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 35;
    let fragile = false;
    let lane = match (weight, fragile) {
        (_, true) => String::from("Careful lane"),
        (w, false) if w > 30 => String::from("Heavy lane"),
        _ => String::from("Normal lane"),
    };
    println!("{lane}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    let fragile = false;
    let lane = match (weight, fragile) {
        (_, true) => String::from("Careful lane"),
        (w, false) if w > 30 => String::from("Heavy lane"),
        (w @ 11..=30, false) => format!("{w} kg, medium lane"),
        _ => String::from("Normal lane"),
    };
    println!("{lane}");
}
```

Compiles and runs:

```rust
struct Parcel {
    weight: u32,
    zone: u32,
}

fn main() {
    let parcel = Parcel { weight: 12, zone: 3 };
    let Parcel { weight, zone } = parcel;
    println!("{weight} {zone}");
    let text = match parcel {
        Parcel { zone: 1, .. } => String::from("Local"),
        Parcel { weight, .. } => format!("{weight} kg, long distance"),
    };
    println!("{text}");
}
```

Compiles and runs:

```rust
fn main() {
    let driver: Option<&str> = None;
    if let Some(name) = driver {
        println!("{name} is driving");
    } else {
        println!("No driver yet");
    }
}
```

Compiles and runs:

```rust
fn main() {
    ship(Some("Sam"));
    ship(None);
}

fn ship(driver: Option<&str>) {
    let Some(name) = driver else {
        println!("No driver, cannot ship");
        return;
    };
    println!("Shipping with {name}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    ship(Some("Sam"));
    ship(None);
}

fn ship(driver: Option<&str>) {
    let Some(name) = driver else {
        println!("No driver, cannot ship");
    };
    println!("Shipping with {name}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut belt = vec![12, 35, 8];
    while let Some(weight) = belt.pop() {
        println!("{weight}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let mut belt = vec![12, 35, 8];
    while let Some(weight) = belt.pop() {
        println!("{weight}");
    }
    let weight = 12;
    let light = matches!(weight, 1..=30);
    println!("{light}");
}
```

## Sources

- match and exhaustiveness: https://doc.rust-lang.org/stable/book/ch06-02-match.html
- if let and let else: https://doc.rust-lang.org/stable/book/ch06-03-if-let.html
- Pattern syntax, guards, @ bindings: https://doc.rust-lang.org/stable/book/ch19-03-pattern-syntax.html
