# 11 - Slices: Borrowing Part of Something

How do you talk about part of a list without copying it? A clerk's note saying start at box four and take three explains slices. We see why a plain position goes stale, write string slices, watch the compiler reject the same bug once a slice is tied to the text, hit a real char boundary panic in a word with an accent, and use array and mutable slices in functions.

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
fn first_word_end(text: &str) -> usize {
    for (i, b) in text.bytes().enumerate() {
        if b == b' ' {
            return i;
        }
    }
    text.len()
}

fn main() {
    let mut label = String::from("Parcel 7");
    let end = first_word_end(&label);
    label.clear();
    println!("{end}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel 7");
    let word = &label[0..6];
    let number = &label[7..8];
    println!("{word} {number}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel 7");
    let word = &label[..6];
    let number = &label[7..];
    println!("{word} {number}");
}
```

Compiles and runs:

```rust
fn main() {
    let label = String::from("Parcel 7");
    let word = &label[..6];
    let number = &label[7..];
    println!("{word} {number}");
    println!("{}", std::mem::size_of::<&str>());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut label = String::from("Parcel 7");
    let word = first_word(&label);
    label.clear();
    println!("{word}");
}

fn first_word(text: &str) -> &str {
    for (i, b) in text.bytes().enumerate() {
        if b == b' ' {
            return &text[..i];
        }
    }
    text
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::from("Parcel 7");
    let word = first_word(&label);
    println!("{word}");
    label.clear();
}

fn first_word(text: &str) -> &str {
    for (i, b) in text.bytes().enumerate() {
        if b == b' ' {
            return &text[..i];
        }
    }
    text
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::from("Parcel 7");
    let word = first_word(&label);
    println!("{word}");
    label.clear();
    println!("{}", first_word("Parcel 9"));
}

fn first_word(text: &str) -> &str {
    for (i, b) in text.bytes().enumerate() {
        if b == b' ' {
            return &text[..i];
        }
    }
    text
}
```

Compiles and runs:

```rust
fn main() {
    let label = "  fragile parcel seven  ";
    println!("[{}]", label.trim());
    for word in label.split_whitespace() {
        println!("{word}");
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let name = "café";
    println!("{}", &name[0..4]);
}
```

Compiles and runs:

```rust
fn main() {
    let name = "café";
    println!("{}", &name[0..5]);
    println!("{}", name.chars().count());
}
```

Compiles and runs:

```rust
fn main() {
    let weights = [12, 35, 8, 20];
    let middle = &weights[1..3];
    println!("{middle:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let weights = [12, 35, 8, 20];
    let middle = &weights[1..3];
    println!("{middle:?}");
    println!("{}", total(&weights));
    println!("{}", total(middle));
    let list = vec![30, 20];
    println!("{}", total(&list));
}

fn total(weights: &[i32]) -> i32 {
    let mut sum = 0;
    for weight in weights {
        sum += weight;
    }
    sum
}
```

Compiles and runs:

```rust
fn main() {
    let mut weights = [12, 35, 8, 20];
    add_packaging(&mut weights[1..3]);
    println!("{weights:?}");
}

fn add_packaging(weights: &mut [i32]) {
    for weight in weights {
        *weight += 1;
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut weights = [12, 35, 8, 20];
    add_packaging(&mut weights[1..9]);
    println!("{weights:?}");
}

fn add_packaging(weights: &mut [i32]) {
    for weight in weights {
        *weight += 1;
    }
}
```

## Sources

- Slices, string slices, char boundaries, other slices: https://doc.rust-lang.org/stable/book/ch04-03-slices.html
- Slicing a str at a non char boundary panics: https://doc.rust-lang.org/stable/std/primitive.str.html
