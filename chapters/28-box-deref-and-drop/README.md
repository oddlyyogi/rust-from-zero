# 28 - Box, Deref and Drop

What is a Box, and when do you actually need one? A claim ticket for a pallet too big for the counter explains it. We measure real sizes, build a recursive delivery route and fix the infinite size error, write our own Deref, see deref coercion turn &Ticket<String> into &str, and use Drop to watch cleanup happen in reverse order, including why you call drop(a) and never a.drop().

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
    let w = Box::new(12);
    println!("{w}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let w = Box::new(12);
    println!("{w}");
    let total = w + 1;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let w = Box::new(12);
    println!("{w}");
    let total = *w + 1;
    println!("{total}");
}
```

Compiles and runs:

```rust
fn main() {
    let w = Box::new(12);
    println!("{w}");
    let total = *w + 1;
    println!("{total}");
    println!("{}", std::mem::size_of::<[u8; 1000]>());
    println!("{}", std::mem::size_of::<Box<[u8; 1000]>>());
}
```

Compiles and runs:

```rust
fn main() {
    let big = Box::new([0u8; 1000]);
    println!("{big:p}");
    let moved = big;
    println!("{moved:p}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
enum Route {
    Stop(String, Route),
    End,
}

fn main() {
}
```

Compiles and runs:

```rust
use Route::*;

enum Route {
    Stop(String, Box<Route>),
    End,
}

fn main() {
    let route = Stop(String::from("Oslo"), Box::new(Stop(String::from("Bergen"), Box::new(End))));
    show(&route);
}

fn show(route: &Route) {
    match route {
        Stop(city, next) => {
            println!("{city}");
            show(next);
        }
        End => println!("end of the line"),
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Ticket<T>(T);

fn main() {
    let t = Ticket(12);
    println!("{}", *t);
}
```

Compiles and runs:

```rust
use std::ops::Deref;

struct Ticket<T>(T);

impl<T> Deref for Ticket<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let t = Ticket(12);
    println!("{}", *t);
}
```

Compiles and runs:

```rust
use std::ops::Deref;

struct Ticket<T>(T);

impl<T> Deref for Ticket<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let t = Ticket(String::from("HP-1"));
    greet(&t);
}

fn greet(code: &str) {
    println!("Sending {code}");
}
```

Compiles and runs:

```rust
struct Locker {
    name: &'static str,
}

impl Drop for Locker {
    fn drop(&mut self) {
        println!("Releasing {}", self.name);
    }
}

fn main() {
    let a = Locker { name: "A" };
    let b = Locker { name: "B" };
    println!("Using {} and {}", a.name, b.name);
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Locker {
    name: &'static str,
}

impl Drop for Locker {
    fn drop(&mut self) {
        println!("Releasing {}", self.name);
    }
}

fn main() {
    let a = Locker { name: "A" };
    let b = Locker { name: "B" };
    a.drop();
    println!("Using {}", b.name);
}
```

Compiles and runs:

```rust
struct Locker {
    name: &'static str,
}

impl Drop for Locker {
    fn drop(&mut self) {
        println!("Releasing {}", self.name);
    }
}

fn main() {
    let a = Locker { name: "A" };
    let b = Locker { name: "B" };
    drop(a);
    println!("Using {}", b.name);
}
```

Compiles and runs:

```rust
struct Locker {
    name: &'static str,
}

impl Drop for Locker {
    fn drop(&mut self) {
        println!("Releasing {}", self.name);
    }
}

fn keep(l: Locker) {
    println!("Keeping {}", l.name);
}

fn main() {
    let a = Locker { name: "A" };
    {
        let b = Locker { name: "B" };
        println!("Inside the block, holding {}", b.name);
    }
    keep(a);
    println!("Back in main");
}
```

## Sources

- Smart pointers: https://doc.rust-lang.org/stable/book/ch15-00-smart-pointers.html
- Box and recursive types, cons list: https://doc.rust-lang.org/stable/book/ch15-01-box.html
- Deref trait and deref coercion: https://doc.rust-lang.org/stable/book/ch15-02-deref.html
- Drop trait, reverse order, std::mem::drop: https://doc.rust-lang.org/stable/book/ch15-03-drop.html
- std::mem::drop implementation: https://doc.rust-lang.org/stable/std/mem/fn.drop.html
- Destructors and drop order: https://doc.rust-lang.org/stable/reference/destructors.html
