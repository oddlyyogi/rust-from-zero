# 19 - Custom Error Types

Why is a String a poor error type? A handwritten note and a printed form with tick boxes explain custom errors. We watch a real bug caused by searching error text, replace it with an enum the compiler can check, implement Display and the Error trait, use From so the ? operator converts errors for you, and compare that with the quick Box<dyn Error> approach.

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
    for text in ["12", "500", "heavy"] {
        match parse_weight(text) {
            Ok(w) => println!("{w} kg"),
            Err(e) if e.contains("heavy") => println!("Send by freight"),
            Err(e) => println!("Ask again: {e}"),
        }
    }
}

fn parse_weight(text: &str) -> Result<u32, String> {
    match text.parse::<u32>() {
        Ok(w) if w > 100 => Err(format!("{w} kg is too heavy")),
        Ok(w) => Ok(w),
        Err(_) => Err(format!("not a number: {text}")),
    }
}
```

Compiles and runs:

```rust
#[derive(Debug)]
enum WeightError {
    NotANumber(String),
    TooHeavy(u32),
}

fn main() {
    for text in ["12", "500", "heavy"] {
        match parse_weight(text) {
            Ok(w) => println!("{w} kg"),
            Err(WeightError::TooHeavy(w)) => println!("Send {w} kg by freight"),
            Err(WeightError::NotANumber(t)) => println!("Ask again: {t}"),
        }
    }
}

fn parse_weight(text: &str) -> Result<u32, WeightError> {
    match text.parse::<u32>() {
        Ok(w) if w > 100 => Err(WeightError::TooHeavy(w)),
        Ok(w) => Ok(w),
        Err(_) => Err(WeightError::NotANumber(text.to_string())),
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
#[derive(Debug)]
enum WeightError {
    NotANumber(String),
    TooHeavy(u32),
    Zero,
}

fn main() {
    for text in ["12", "500", "heavy"] {
        match parse_weight(text) {
            Ok(w) => println!("{w} kg"),
            Err(WeightError::TooHeavy(w)) => println!("Send {w} kg by freight"),
            Err(WeightError::NotANumber(t)) => println!("Ask again: {t}"),
        }
    }
}

fn parse_weight(text: &str) -> Result<u32, WeightError> {
    match text.parse::<u32>() {
        Ok(w) if w > 100 => Err(WeightError::TooHeavy(w)),
        Ok(w) => Ok(w),
        Err(_) => Err(WeightError::NotANumber(text.to_string())),
    }
}
```

Compiles and runs:

```rust
#[derive(Debug)]
enum WeightError {
    NotANumber(String),
    TooHeavy(u32),
    Zero,
}

fn main() {
    for text in ["12", "500", "heavy", "0"] {
        match parse_weight(text) {
            Ok(w) => println!("{w} kg"),
            Err(WeightError::TooHeavy(w)) => println!("Send {w} kg by freight"),
            Err(WeightError::NotANumber(t)) => println!("Ask again: {t}"),
            Err(WeightError::Zero) => println!("A parcel can't weigh nothing"),
        }
    }
}

fn parse_weight(text: &str) -> Result<u32, WeightError> {
    match text.parse::<u32>() {
        Ok(0) => Err(WeightError::Zero),
        Ok(w) if w > 100 => Err(WeightError::TooHeavy(w)),
        Ok(w) => Ok(w),
        Err(_) => Err(WeightError::NotANumber(text.to_string())),
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
#[derive(Debug)]
enum WeightError {
    NotANumber(String),
    TooHeavy(u32),
}

fn main() {
    let e = WeightError::TooHeavy(500);
    println!("{e}");
    let n = WeightError::NotANumber(String::from("heavy"));
    println!("{n}");
}
```

Compiles and runs:

```rust
use std::fmt;

#[derive(Debug)]
enum WeightError {
    NotANumber(String),
    TooHeavy(u32),
}

impl fmt::Display for WeightError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeightError::NotANumber(t) => write!(f, "not a number: {t}"),
            WeightError::TooHeavy(w) => write!(f, "{w} kg is too heavy"),
        }
    }
}

fn main() {
    let e = WeightError::TooHeavy(500);
    println!("{e}");
    let n = WeightError::NotANumber(String::from("heavy"));
    println!("{n}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    let _ = load("good.txt");
}

enum LoadError {
    Read(io::Error),
    BadWeight(ParseIntError),
}

fn load(path: &str) -> Result<u32, LoadError> {
    let text = fs::read_to_string(path)?;
    let w = text.trim().parse::<u32>()?;
    Ok(w)
}
```

Compiles and runs:

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    fs::write("good.txt", "12").unwrap();
    fs::write("bad.txt", "twelve").unwrap();
    for path in ["good.txt", "bad.txt", "missing.txt"] {
        match load(path) {
            Ok(w) => println!("{path}: {w} kg"),
            Err(LoadError::Read(e)) => println!("{path}: could not read: {e}"),
            Err(LoadError::BadWeight(e)) => println!("{path}: bad weight: {e}"),
        }
    }
}

enum LoadError {
    Read(io::Error),
    BadWeight(ParseIntError),
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::Read(e)
    }
}

impl From<ParseIntError> for LoadError {
    fn from(e: ParseIntError) -> Self {
        LoadError::BadWeight(e)
    }
}

fn load(path: &str) -> Result<u32, LoadError> {
    let text = fs::read_to_string(path)?;
    let w = text.trim().parse::<u32>()?;
    Ok(w)
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    fs::write("bad.txt", "twelve")?;
    println!("{}", load("bad.txt")?);
    Ok(())
}

fn load(path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let w = text.trim().parse::<u32>()?;
    Ok(w)
}
```

## Sources

- Recoverable errors and the ? operator calling From: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html
- Display trait and implementing fmt: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html
- Error trait requires Debug and Display: https://doc.rust-lang.org/stable/std/error/trait.Error.html
- From trait: https://doc.rust-lang.org/stable/std/convert/trait.From.html
- thiserror crate: https://docs.rs/thiserror
- anyhow crate: https://docs.rs/anyhow
