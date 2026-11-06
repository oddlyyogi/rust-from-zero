# 12 - Structs and Methods

How do you turn loose values into something with a name? The printed parcel form at Harbor Parcels explains structs. We define one, see why a half filled form cannot exist, change fields, print with derive Debug, attach methods with an impl block, compare the three kinds of self, write a constructor with Self, and use tuple structs so a weight can never be swapped with a price.

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
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let parcel = Parcel {
        tracking: String::from("HP-1"),
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
    parcel.weight = 14;
    println!("{}", parcel.weight);
}
```

Compiles and runs:

```rust
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let mut parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
    parcel.weight = 14;
    println!("{}", parcel.weight);
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let mut parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
    parcel.weight = 14;
    println!("{}", parcel.weight);
    let second = Parcel {
        tracking: String::from("HP-2"),
        ..parcel
    };
    println!("{}", second);
}

fn make_parcel(tracking: String, weight: u32) -> Parcel {
    Parcel {
        tracking,
        weight,
        fragile: false,
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let mut parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
    parcel.weight = 14;
    println!("{}", parcel.weight);
    let second = Parcel {
        tracking: String::from("HP-2"),
        ..parcel
    };
    println!("{:?}", second);
}

fn make_parcel(tracking: String, weight: u32) -> Parcel {
    Parcel {
        tracking,
        weight,
        fragile: false,
    }
}
```

Compiles and runs:

```rust
#[derive(Debug)]
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let mut parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
    parcel.weight = 14;
    println!("{}", parcel.weight);
    let second = Parcel {
        tracking: String::from("HP-2"),
        ..parcel
    };
    println!("{:?}", second);
}

fn make_parcel(tracking: String, weight: u32) -> Parcel {
    Parcel {
        tracking,
        weight,
        fragile: false,
    }
}
```

Compiles and runs:

```rust
#[derive(Debug)]
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

fn main() {
    let mut parcel = Parcel {
        tracking: String::from("HP-1"),
        weight: 12,
        fragile: true,
    };
    println!("{} {}", parcel.tracking, parcel.weight);
    parcel.weight = 14;
    println!("{}", parcel.weight);
    let second = Parcel {
        tracking: String::from("HP-2"),
        ..parcel
    };
    println!("{:#?}", second);
}

fn make_parcel(tracking: String, weight: u32) -> Parcel {
    Parcel {
        tracking,
        weight,
        fragile: false,
    }
}
```

Compiles and runs:

```rust
#[derive(Debug)]
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

impl Parcel {
    fn price(&self) -> u32 {
        self.weight * 2
    }
}

fn main() {
    let parcel = Parcel { tracking: String::from("HP-1"), weight: 14, fragile: false };
    println!("{}", parcel.price());
}
```

Does not compile, on purpose, so we can read the error:

```rust
#[derive(Debug)]
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

impl Parcel {
    fn price(&self) -> u32 {
        self.weight * 2
    }
    fn mark_fragile(&mut self) {
        self.fragile = true;
    }
    fn deliver(self) -> String {
        self.tracking
    }
}

fn main() {
    let mut parcel = Parcel { tracking: String::from("HP-1"), weight: 14, fragile: false };
    println!("{}", parcel.price());
    parcel.mark_fragile();
    let tracking = parcel.deliver();
    println!("{tracking}");
    println!("{}", parcel.weight);
}
```

Compiles and runs:

```rust
#[derive(Debug)]
struct Parcel {
    tracking: String,
    weight: u32,
    fragile: bool,
}

impl Parcel {
    fn price(&self) -> u32 {
        self.weight * 2
    }
    fn mark_fragile(&mut self) {
        self.fragile = true;
    }
    fn deliver(self) -> String {
        self.tracking
    }
    fn new(tracking: &str, weight: u32) -> Self {
        Self {
            tracking: tracking.to_string(),
            weight,
            fragile: false,
        }
    }
}

fn main() {
    let mut parcel = Parcel { tracking: String::from("HP-1"), weight: 14, fragile: false };
    println!("{}", parcel.price());
    parcel.mark_fragile();
    let tracking = parcel.deliver();
    println!("{tracking}");
    let third = Parcel::new("HP-3", 20);
    println!("{}", third.price());
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Kilos(u32);
struct Pence(u32);

fn charge(weight: Kilos, rate: Pence) -> u32 {
    weight.0 * rate.0
}

fn main() {
    println!("{}", charge(Pence(50), Kilos(12)));
}
```

Compiles and runs:

```rust
struct Kilos(u32);
struct Pence(u32);

fn charge(weight: Kilos, rate: Pence) -> u32 {
    weight.0 * rate.0
}

fn main() {
    println!("{}", charge(Kilos(12), Pence(50)));
}
```

## Sources

- Defining structs, field shorthand, update syntax, tuple structs: https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html
- Printing with derive Debug: https://doc.rust-lang.org/stable/book/ch05-02-example-structs.html
- Methods, kinds of self, associated functions: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html
