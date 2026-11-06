# 13 - Enums and Option

How do you say a parcel is exactly one of packed, in transit or delivered? The tick boxes on a Harbor Parcels form explain enums. We see a real typo slip past plain text, define an enum, give variants their own data, get the compiler to list every case we forgot in a match, and replace the billion dollar mistake of null with Option, Some and None.

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
    let status = "in transit";
    if status == "in trnasit" {
        println!("On a van");
    }
}
```

Compiles and runs:

```rust
#[derive(Debug)]
enum Status {
    Packed,
    InTransit,
    Delivered,
}

fn main() {
    let status = Status::InTransit;
    println!("{status:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
#[derive(Debug)]
enum Status {
    Packed,
    InTransit,
    Delivered,
}

fn main() {
    let status = Status::InTrnasit;
    println!("{status:?}");
}
```

Compiles and runs:

```rust
#[derive(Debug)]
enum Status {
    Packed,
    InTransit(u32),
    Delivered(String),
}

fn main() {
    let a = Status::Packed;
    let b = Status::InTransit(7);
    let c = Status::Delivered(String::from("Sam"));
    println!("{a:?} {b:?} {c:?}");
}
```

Compiles and runs:

```rust
#[derive(Debug)]
enum Status {
    Packed,
    InTransit(u32),
    Delivered(String),
    Lost { last_seen: String },
}

fn main() {
    let a = Status::Packed;
    let b = Status::InTransit(7);
    let c = Status::Delivered(String::from("Sam"));
    println!("{a:?} {b:?} {c:?}");
    let d = Status::Lost { last_seen: String::from("the depot") };
    println!("{d:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
#[derive(Debug)]
enum Status {
    Packed,
    InTransit(u32),
    Delivered(String),
    Lost { last_seen: String },
}

impl Status {
    fn describe(&self) -> String {
        match self {
            Status::Packed => String::from("Waiting at the depot"),
            Status::InTransit(van) => format!("On van {van}"),
        }
    }
}

fn main() {
    let a = Status::Packed;
    let b = Status::InTransit(7);
    let c = Status::Delivered(String::from("Sam"));
    println!("{a:?} {b:?} {c:?}");
    let d = Status::Lost { last_seen: String::from("the depot") };
    println!("{d:?}");
}
```

Compiles and runs:

```rust
#[derive(Debug)]
enum Status {
    Packed,
    InTransit(u32),
    Delivered(String),
    Lost { last_seen: String },
}

impl Status {
    fn describe(&self) -> String {
        match self {
            Status::Packed => String::from("Waiting at the depot"),
            Status::InTransit(van) => format!("On van {van}"),
            Status::Delivered(name) => format!("Signed by {name}"),
            Status::Lost { last_seen } => format!("Last seen at {last_seen}"),
        }
    }
}

fn main() {
    let a = Status::Packed;
    let b = Status::InTransit(7);
    let c = Status::Delivered(String::from("Sam"));
    println!("{}", a.describe());
    println!("{}", b.describe());
    println!("{}", c.describe());
    let d = Status::Lost { last_seen: String::from("the depot") };
    println!("{}", d.describe());
}
```

Compiles and runs:

```rust
fn main() {
    let driver = Some(String::from("Sam"));
    let spare: Option<String> = None;
    println!("{driver:?} {spare:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weight = Some(12);
    let total = weight + 1;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight = Some(12);
    let total = weight.unwrap_or(0) + 1;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let weight: Option<i32> = None;
    let total = weight.unwrap_or(0) + 1;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    println!("{:?}", find_weight("HP-1"));
    println!("{:?}", find_weight("HP-9"));
}

fn find_weight(tracking: &str) -> Option<u32> {
    if tracking == "HP-1" { Some(12) } else { None }
}
```

Compiles and runs:

```rust
fn main() {
    println!("{:?}", find_weight("HP-1"));
    println!("{:?}", find_weight("HP-9"));
    let weights = [12, 35];
    println!("{:?}", weights.first());
    println!("{:?}", weights.get(9));
}

fn find_weight(tracking: &str) -> Option<u32> {
    if tracking == "HP-1" { Some(12) } else { None }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    println!("{:?}", find_weight("HP-1"));
    println!("{:?}", find_weight("HP-9"));
    let weights = [12, 35];
    println!("{:?}", weights.first());
    println!("{:?}", weights.get(9));
    println!("{}", find_weight("HP-9").unwrap());
}

fn find_weight(tracking: &str) -> Option<u32> {
    if tracking == "HP-1" { Some(12) } else { None }
}
```

Compiles and runs:

```rust
fn main() {
    println!("{:?}", find_weight("HP-1"));
    println!("{:?}", find_weight("HP-9"));
    let weights = [12, 35];
    println!("{:?}", weights.first());
    println!("{:?}", weights.get(9));
    match find_weight("HP-9") {
        Some(w) => println!("{w} kg"),
        None => println!("Not found"),
    }
}

fn find_weight(tracking: &str) -> Option<u32> {
    if tracking == "HP-1" { Some(12) } else { None }
}
```

## Sources

- Enums, Option, and Tony Hoare on null: https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html
- match must cover every case: https://doc.rust-lang.org/stable/book/ch06-02-match.html
