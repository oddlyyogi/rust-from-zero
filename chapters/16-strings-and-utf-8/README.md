# 16 - Strings and UTF-8

Why can't you write word[0] in Rust? A roll of label tape divided into tiny squares explains UTF-8: some letters fill one square, others two, three or four. We compare String and &str, join text with + and format!, measure bytes against characters, hit the real errors for indexing and the panic for cutting through a letter, then use get, chars, bytes, trim, split and replace on a line of depot data.

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
    let name = "Harbor";
    name.push_str(" Parcels");
    println!("{name}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut name = String::from("Harbor");
    name.push(' ');
    name.push_str("Parcels");
    println!("{name}");
}
```

Compiles and runs:

```rust
fn main() {
    let first = String::from("Harbor");
    let second = String::from("Parcels");
    let both = first + " " + &second;
    println!("{both}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let first = String::from("Harbor");
    let second = String::from("Parcels");
    let both = first + " " + &second;
    println!("{both}");
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let first = String::from("Harbor");
    let second = String::from("Parcels");
    let both = format!("{first} {second}");
    println!("{both}");
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let word = "Café";
    println!("{} {}", word.len(), word.chars().count());
}
```

Compiles and runs:

```rust
fn main() {
    let word = "Café";
    println!("{} {}", word.len(), word.chars().count());
    println!("{} {}", "東京".len(), "東京".chars().count());
    println!("{} {}", "📦".len(), "📦".chars().count());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let word = String::from("Harbor");
    let first = word[0];
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let word = String::from("Harbor");
    let first = &word[0..3];
    println!("{first}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let word = String::from("Café");
    let first = &word[0..4];
    println!("{first}");
}
```

Compiles and runs:

```rust
fn main() {
    let word = "Café";
    println!("{:?}", word.get(0..4));
    println!("{:?}", word.get(0..3));
}
```

Compiles and runs:

```rust
fn main() {
    let word = "Café";
    println!("{:?}", word.get(0..4));
    println!("{:?}", word.get(0..3));
    for c in word.chars() { print!("{c} "); }
    println!();
    for b in word.bytes() { print!("{b} "); }
    println!();
}
```

Compiles and runs:

```rust
fn main() {
    let line = "  HP-1,Oslo,12  ";
    let clean = line.trim();
    println!("[{clean}]");
    println!("{}", clean.to_uppercase());
    println!("{}", clean.contains("Oslo"));
    println!("{}", clean.replace("Oslo", "Bergen"));
    for part in clean.split(',') {
        println!("{part}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let owned = String::from("HP-1");
    println!("{}", is_ours(&owned));
    println!("{}", is_ours("XX-9"));
}

fn is_ours(code: &str) -> bool {
    code.starts_with("HP-")
}
```

Compiles and runs:

```rust
fn main() {
    let owned = String::from("HP-1");
    println!("{}", is_ours(&owned));
    println!("{}", is_ours("XX-9"));
    println!("{}", label(&owned, "Oslo"));
}

fn is_ours(code: &str) -> bool {
    code.starts_with("HP-")
}

fn label(tracking: &str, city: &str) -> String {
    format!("{tracking} to {city}")
}
```

## Sources

- Storing UTF-8 text with strings: https://doc.rust-lang.org/stable/book/ch08-02-strings.html
- String type, wraps Vec<u8>, UTF-8 validity, len in bytes: https://doc.rust-lang.org/stable/std/string/struct.String.html
- str methods get, chars, bytes, trim, split, slicing panics off a char boundary: https://doc.rust-lang.org/stable/std/primitive.str.html
- UTF-8 uses one to four bytes per code point: https://www.rfc-editor.org/rfc/rfc3629
