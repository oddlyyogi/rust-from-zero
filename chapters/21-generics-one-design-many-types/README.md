# 21 - Generics: One Design, Many Types

Why write the same function twice? A single drawing of a storage bin, with a blank where the contents go, explains generics. We replace duplicate functions with one generic function, meet trait bounds through a real compiler error, build generic structs and methods, see that Option, Result and Vec are generics too, use the turbofish, and find out why generic code costs nothing at run time.

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
    println!("{}", largest_u32(&[12, 35, 8]));
    println!("{}", largest_f64(&[1.5, 2.5, 0.5]));
}

fn largest_u32(list: &[u32]) -> &u32 {
    let mut big = &list[0];
    for item in list {
        if item > big {
            big = item;
        }
    }
    big
}

fn largest_f64(list: &[f64]) -> &f64 {
    let mut big = &list[0];
    for item in list {
        if item > big {
            big = item;
        }
    }
    big
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    println!("{}", largest(&[12, 35, 8]));
    println!("{}", largest(&[1.5, 2.5, 0.5]));
    println!("{}", largest(&['a', 'z', 'm']));
}

fn largest<T>(list: &[T]) -> &T {
    let mut big = &list[0];
    for item in list {
        if item > big {
            big = item;
        }
    }
    big
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", largest(&[12, 35, 8]));
    println!("{}", largest(&[1.5, 2.5, 0.5]));
    println!("{}", largest(&['a', 'z', 'm']));
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut big = &list[0];
    for item in list {
        if item > big {
            big = item;
        }
    }
    big
}
```

Compiles and runs:

```rust
struct Bin<T> {
    item: T,
}

fn main() {
    let a = Bin { item: 12 };
    let b = Bin { item: String::from("HP-1") };
    println!("{} {}", a.item, b.item);
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Bin<T> {
    item: T,
}

fn main() {
    let mut a = Bin { item: 12 };
    a.item = String::from("tape");
    let b = Bin { item: String::from("HP-1") };
    println!("{} {}", a.item, b.item);
}
```

Compiles and runs:

```rust
struct Bin<T> {
    item: T,
}

struct Pair<A, B> {
    first: A,
    second: B,
}

fn main() {
    let a = Bin { item: 12 };
    let b = Bin { item: String::from("HP-1") };
    println!("{} {}", a.item, b.item);
    let p = Pair { first: 12, second: "fragile" };
    println!("{} {}", p.first, p.second);
}
```

Compiles and runs:

```rust
struct Bin<T> {
    item: T,
}

impl<T> Bin<T> {
    fn new(item: T) -> Bin<T> {
        Bin { item }
    }
    fn peek(&self) -> &T {
        &self.item
    }
    fn take(self) -> T {
        self.item
    }
}

fn main() {
    let a = Bin::new(12);
    println!("{}", a.peek());
    let b = Bin::new(String::from("fragile"));
    let label = b.take();
    println!("{label}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Bin<T> {
    item: T,
}

impl<T> Bin<T> {
    fn new(item: T) -> Bin<T> {
        Bin { item }
    }
    fn peek(&self) -> &T {
        &self.item
    }
    fn take(self) -> T {
        self.item
    }
}
impl Bin<String> {
    fn shout(&self) -> String {
        self.item.to_uppercase()
    }
}


fn main() {
    let a = Bin::new(12);
    println!("{}", a.peek());
    let b = Bin::new(String::from("fragile"));
    println!("{}", b.shout());
    println!("{}", a.shout());
    let label = b.take();
    println!("{label}");
}
```

Compiles and runs:

```rust
struct Bin<T> {
    item: T,
}

impl<T> Bin<T> {
    fn new(item: T) -> Bin<T> {
        Bin { item }
    }
    fn peek(&self) -> &T {
        &self.item
    }
    fn take(self) -> T {
        self.item
    }
}
impl Bin<String> {
    fn shout(&self) -> String {
        self.item.to_uppercase()
    }
}


fn main() {
    let a = Bin::new(12);
    println!("{}", a.peek());
    let b = Bin::new(String::from("fragile"));
    println!("{}", b.shout());
    let label = b.take();
    println!("{label}");
}
```

Compiles and runs:

```rust
enum Slot<T> {
    Empty,
    Holding(T),
}

fn main() {
    let slots = [Slot::Holding(12), Slot::Empty, Slot::Holding(35)];
    for s in &slots {
        match s {
            Slot::Holding(w) => println!("{w} kg"),
            Slot::Empty => println!("empty"),
        }
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let list = Vec::new();
    println!("{}", list.len());
}
```

Compiles and runs:

```rust
fn main() {
    let list = Vec::<u32>::new();
    println!("{}", list.len());
}
```

Compiles and runs:

```rust
fn main() {
    show(12);
    show(2.5);
    show("HP-1");
    show(String::from("HP-2"));
}

fn show<T>(_value: T) {
    println!("{}", std::any::type_name::<T>());
}
```

## Sources

- Generic data types, the largest example, monomorphization and performance: https://doc.rust-lang.org/stable/book/ch10-01-syntax.html
- Trait bounds: https://doc.rust-lang.org/stable/book/ch10-02-traits.html
- PartialOrd: https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html
- Turbofish naming, method call syntax: https://doc.rust-lang.org/stable/book/appendix-02-operators.html
- std::any::type_name, output not guaranteed stable: https://doc.rust-lang.org/stable/std/any/fn.type_name.html
- Integer and float defaults i32 and f64: https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
