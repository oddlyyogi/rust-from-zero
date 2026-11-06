# 24 - Closures: Small Functions That Remember

What is a closure, and why does Rust have them? A supervisor's job card, with today's limit clipped to it, explains. We write closures with and without types, capture values that a named function can't reach, see the three ways of capturing with real errors, meet Fn, FnMut and FnOnce, pass closures to functions, return them with move, and use them with sort_by_key, retain and unwrap_or_else.

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
    let double = |w| w * 2;
    println!("{}", double(12));
}
```

Compiles and runs:

```rust
fn main() {
    let double = |w| w * 2;
    println!("{}", double(12));
    let describe = |w: u32| -> String {
        format!("{w} kg")
    };
    println!("{}", describe(12));
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let double = |w| w * 2;
    println!("{}", double(12));
    let describe = |w: u32| -> String {
        format!("{w} kg")
    };
    println!("{}", describe(12));
    println!("{}", double(1.5));
}
```

Compiles and runs:

```rust
fn main() {
    let limit = 30;
    let heavy = |w: u32| w > limit;
    println!("{}", heavy(35));
    println!("{}", heavy(12));
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let limit = 30;
    let heavy = |w: u32| w > limit;
    println!("{}", heavy(35));
    println!("{}", heavy(12));
    fn too_heavy(w: u32) -> bool {
        w > limit
    }
}
```

Compiles and runs:

```rust
fn main() {
    let code = String::from("HP-1");
    let show = || println!("{code}");
    show();
    println!("{code}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let code = String::from("HP-1");
    let show = || println!("{code}");
    show();
    println!("{code}");
    let mut count = 0;
    let add = || count += 1;
    add();
    add();
    println!("{count}");
}
```

Compiles and runs:

```rust
fn main() {
    let code = String::from("HP-1");
    let show = || println!("{code}");
    show();
    println!("{code}");
    let mut count = 0;
    let mut add = || count += 1;
    add();
    add();
    println!("{count}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let code = String::from("HP-1");
    let show = || println!("{code}");
    show();
    println!("{code}");
    let mut count = 0;
    let mut add = || count += 1;
    add();
    println!("{count}");
    add();
    println!("{count}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let code = String::from("HP-1");
    let show = move || println!("{code}");
    show();
    println!("{code}");
    let mut count = 0;
    let mut add = || count += 1;
    add();
    add();
    println!("{count}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let code = String::from("HP-1");
    let send = || drop(code);
    send();
    send();
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", apply(|w| w + 1, 12));
}

fn apply<F: Fn(u32) -> u32>(f: F, value: u32) -> u32 {
    f(value)
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", apply(|w| w + 1, 12));
    println!("{}", apply(double, 12));
}

fn apply<F: Fn(u32) -> u32>(f: F, value: u32) -> u32 {
    f(value)
}

fn double(w: u32) -> u32 {
    w * 2
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", apply(|w| w + 1, 12));
    println!("{}", apply(double, 12));
    let mut count = 0;
    twice(|| count += 1);
    println!("{count}");
}

fn apply<F: Fn(u32) -> u32>(f: F, value: u32) -> u32 {
    f(value)
}

fn double(w: u32) -> u32 {
    w * 2
}

fn twice<F: FnMut()>(mut f: F) {
    f();
    f();
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let heavy = heavier_than(30);
    println!("{}", heavy(35));
}

fn heavier_than(limit: u32) -> impl Fn(u32) -> bool {
    |w| w > limit
}
```

Compiles and runs:

```rust
fn main() {
    let heavy = heavier_than(30);
    println!("{}", heavy(35));
}

fn heavier_than(limit: u32) -> impl Fn(u32) -> bool {
    move |w| w > limit
}
```

Compiles and runs:

```rust
fn main() {
    let mut parcels = vec![("HP-1", 12), ("HP-2", 35), ("HP-3", 8)];
    parcels.sort_by_key(|p| p.1);
    println!("{parcels:?}");
    parcels.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{parcels:?}");
    let limit = 10;
    parcels.retain(|p| p.1 > limit);
    println!("{parcels:?}");
    let name: Option<&str> = None;
    println!("{}", name.unwrap_or_else(|| "nobody"));
}
```

## Sources

- Closures, capturing, move, Fn traits, sort_by_key: https://doc.rust-lang.org/stable/book/ch13-01-closures.html
- Advanced functions and closures, returning closures: https://doc.rust-lang.org/stable/book/ch20-04-advanced-functions-and-closures.html
- Fn, FnMut, FnOnce: https://doc.rust-lang.org/stable/std/ops/trait.Fn.html
- Vec retain, slice sort_by_key: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
- Option unwrap_or_else: https://doc.rust-lang.org/stable/std/option/enum.Option.html
