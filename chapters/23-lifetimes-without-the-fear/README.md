# 23 - Lifetimes Without the Fear

What is a lifetime, and why does Rust keep asking for one? A note saying your parcel is in locker 22 explains it. We catch a dangling reference, answer the compiler's question with 'a, see on a timeline that it means the overlap of two lifetimes, watch the check work, learn why you rarely need to write them, put references in structs, and find out what 'static really promises.

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
    let label;
    {
        let code = String::from("HP-1");
        label = &code;
    }
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    let code = String::from("HP-1");
    let label = &code;
    println!("{label}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    println!("{}", longer("Oslo", "Bergen"));
}

fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", longer("Oslo", "Bergen"));
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Compiles and runs:

```rust
fn main() {
    let a = String::from("Oslo");
    let result;
    {
        let b = String::from("Bergen");
        result = longer(&a, &b);
        println!("{result}");
    }
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let a = String::from("Oslo");
    let result;
    {
        let b = String::from("Bergen");
        result = longer(&a, &b);
    }
    println!("{result}");
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Compiles and runs:

```rust
fn main() {
    let line = String::from("HP-1,Oslo");
    let result;
    {
        let prefix = String::from("Checking");
        result = tracking(&line, &prefix);
    }
    println!("{result}");
}

fn tracking<'a>(line: &'a str, prefix: &str) -> &'a str {
    println!("{prefix}");
    line
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    println!("{}", make_label());
}

fn make_label<'a>() -> &'a str {
    let s = String::from("HP-1");
    &s
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", make_label());
}

fn make_label() -> String {
    let s = String::from("HP-1");
    s
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", first_part("HP-1,Oslo"));
}

fn first_part(line: &str) -> &str {
    match line.split_once(',') {
        Some((first, _)) => first,
        None => line,
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Label {
    text: &str,
}

fn main() {
}
```

Compiles and runs:

```rust
struct Label<'a> {
    text: &'a str,
}

fn main() {
    let line = String::from("HP-1");
    let label = Label { text: &line };
    println!("{}", label.text);
}
```

Does not compile, on purpose, so we can read the error:

```rust
struct Label<'a> {
    text: &'a str,
}

fn main() {
    let line = String::from("HP-1");
    let label = Label { text: &line };
    drop(line);
    println!("{}", label.text);
}
```

Compiles and runs:

```rust
fn main() {
    println!("{}", depot_name());
}

fn depot_name() -> &'static str {
    "Harbor Parcels"
}
```

## Sources

- Validating references with lifetimes, dangling references, longest example, elision rules, structs, 'static: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html
- Lifetime elision reference: https://doc.rust-lang.org/stable/reference/lifetime-elision.html
- E0597: https://doc.rust-lang.org/stable/error_codes/E0597.html
- E0106: https://doc.rust-lang.org/stable/error_codes/E0106.html
- E0515: https://doc.rust-lang.org/stable/error_codes/E0515.html
