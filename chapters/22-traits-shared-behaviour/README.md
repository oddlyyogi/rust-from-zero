# 22 - Traits: Shared Behaviour

What do a van and a cargo bike have in common? Not much, except that both can deliver a parcel. A delivery badge explains Rust traits. We define a trait, implement it for two unrelated types, see the compiler enforce every requirement, add default methods, write functions that accept anything holding the badge, derive standard traits, meet the orphan rule, and build a mixed fleet with Box<dyn Trait>.

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
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    println!("{}", van.deliver("HP-1"));
    println!("{}", bike.deliver("HP-2"));
}
```

Does not compile, on purpose, so we can read the error:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
    fn max_kg(&self) -> u32;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    println!("{}", van.deliver("HP-1"));
    println!("{}", bike.deliver("HP-2"));
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
    fn max_kg(&self) -> u32;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
    fn max_kg(&self) -> u32 {
        800
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
    fn max_kg(&self) -> u32 {
        30
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    println!("{}", van.deliver("HP-1"));
    println!("{}", bike.deliver("HP-2"));
    println!("{}", van.max_kg());
    println!("{}", bike.max_kg());
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
    fn max_kg(&self) -> u32;
    fn describe(&self) -> String {
        format!("Carries up to {} kg", self.max_kg())
    }
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
    fn max_kg(&self) -> u32 {
        800
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
    fn max_kg(&self) -> u32 {
        30
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    println!("{}", van.deliver("HP-1"));
    println!("{}", bike.deliver("HP-2"));
    println!("{}", van.max_kg());
    println!("{}", bike.max_kg());
    println!("{}", van.describe());
    println!("{}", bike.describe());
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
    fn max_kg(&self) -> u32;
    fn describe(&self) -> String {
        format!("Carries up to {} kg", self.max_kg())
    }
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
    fn max_kg(&self) -> u32 {
        800
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
    fn max_kg(&self) -> u32 {
        30
    }
    fn describe(&self) -> String {
        String::from("Light parcels only")
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    println!("{}", van.deliver("HP-1"));
    println!("{}", bike.deliver("HP-2"));
    println!("{}", van.max_kg());
    println!("{}", bike.max_kg());
    println!("{}", van.describe());
    println!("{}", bike.describe());
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn dispatch(v: &impl Deliver, code: &str) {
    println!("{}", v.deliver(code));
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    dispatch(&van, "HP-1");
    dispatch(&bike, "HP-2");
}
```

Does not compile, on purpose, so we can read the error:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

struct Trolley;

fn dispatch(v: &impl Deliver, code: &str) {
    println!("{}", v.deliver(code));
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    dispatch(&van, "HP-1");
    dispatch(&bike, "HP-2");
    dispatch(&Trolley, "HP-3");
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn dispatch<T: Deliver>(v: &T, code: &str) {
    println!("{}", v.deliver(code));
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    dispatch(&van, "HP-1");
    dispatch(&bike, "HP-2");
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Parcel {
    code: String,
    weight: u32,
}

fn main() {
    let a = Parcel { code: String::from("HP-1"), weight: 12 };
    let b = Parcel { code: String::from("HP-1"), weight: 12 };
    println!("{}", a == b);
}
```

Compiles and runs:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Parcel {
    code: String,
    weight: u32,
}

fn main() {
    let a = Parcel { code: String::from("HP-1"), weight: 12 };
    let b = Parcel { code: String::from("HP-1"), weight: 12 };
    println!("{}", a == b);
    let c = a.clone();
    println!("{c:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::fmt;

impl fmt::Display for Vec<u32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a list")
    }
}

fn main() {
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

fn pick() -> impl Deliver {
    Van { plate: String::from("V7") }
}

fn main() {
    let v = pick();
    println!("{}", v.deliver("HP-4"));
}
```

Does not compile, on purpose, so we can read the error:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn pick(heavy: bool) -> impl Deliver {
    if heavy {
        Van { plate: String::from("V7") }
    } else {
        Bike { rider: String::from("Sam") }
    }
}

fn main() {
    let v = pick(true);
    println!("{}", v.deliver("HP-4"));
}
```

Does not compile, on purpose, so we can read the error:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    let fleet = vec![van, bike];
}
```

Compiles and runs:

```rust
trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn main() {
    let van = Van { plate: String::from("V7") };
    let bike = Bike { rider: String::from("Sam") };
    let fleet: Vec<Box<dyn Deliver>> = vec![Box::new(van), Box::new(bike)];
    for v in &fleet {
        println!("{}", v.deliver("HP-9"));
    }
}
```

## Sources

- Traits, defining and implementing, default methods, impl Trait and bounds, where clauses: https://doc.rust-lang.org/stable/book/ch10-02-traits.html
- Orphan rule: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type
- Derivable traits: https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html
- Trait objects and dynamic dispatch: https://doc.rust-lang.org/stable/book/ch18-02-trait-objects.html
- Box: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
