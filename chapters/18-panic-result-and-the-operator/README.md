# 18 - panic, Result and the ? Operator

What should a program do when something goes wrong? A fire alarm and a failed delivery slip explain Rust's two answers: panic and Result. We read real panic output, parse text into numbers, handle errors with match, write functions that return Result, use unwrap, expect and unwrap_or, shorten everything with the ? operator, use ? in main and on Option, and handle a missing file.

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
    println!("Loading the van");
    panic!("conveyor jammed");
    println!("Van loaded");
}
```

Compiles and runs:

```rust
fn main() {
    let good = "12".parse::<u32>();
    let bad = "heavy".parse::<u32>();
    println!("{good:?}");
    println!("{bad:?}");
}
```

Compiles and runs:

```rust
fn main() {
    let good = "12".parse::<u32>();
    let bad = "heavy".parse::<u32>();
    println!("{good:?}");
    println!("{bad:?}");
    match bad {
        Ok(w) => println!("{w} kg"),
        Err(e) => println!("Not a number: {e}"),
    }
}
```

Compiles and runs:

```rust
fn main() {
    "heavy".parse::<u32>();
    println!("All fine");
}
```

Compiles and runs:

```rust
fn main() {
    for text in ["12", "500", "heavy"] {
        match parse_weight(text) {
            Ok(w) => println!("{w} kg"),
            Err(e) => println!("Problem: {e}"),
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
fn main() {
    let a = "12".parse::<u32>().unwrap();
    println!("{a}");
    let b = "heavy".parse::<u32>().unwrap_or(0);
    println!("{b}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let a = "12".parse::<u32>().unwrap();
    println!("{a}");
    let b = "heavy".parse::<u32>().unwrap_or(0);
    println!("{b}");
    let c = "heavy".parse::<u32>().expect("weight must be a number");
    println!("{c}");
}
```

Compiles and runs:

```rust
use std::num::ParseIntError;

fn main() {
    println!("{:?}", total("12", "35"));
    println!("{:?}", total("12", "heavy"));
}

fn total(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x = match a.parse::<u32>() { Ok(v) => v, Err(e) => return Err(e) };
    let y = match b.parse::<u32>() { Ok(v) => v, Err(e) => return Err(e) };
    Ok(x + y)
}
```

Compiles and runs:

```rust
use std::num::ParseIntError;

fn main() {
    println!("{:?}", total("12", "35"));
    println!("{:?}", total("12", "heavy"));
}

fn total(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x = a.parse::<u32>()?;
    let y = b.parse::<u32>()?;
    Ok(x + y)
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::num::ParseIntError;

fn main() {
    let w = "heavy".parse::<u32>()?;
    println!("{w}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let w = "heavy".parse::<u32>()?;
    println!("{w}");
    Ok(())
}
```

Compiles and runs:

```rust
fn main() {
    println!("{:?}", city("HP-1,Oslo"));
    println!("{:?}", city("HP-2"));
}

fn city(line: &str) -> Option<&str> {
    let (_code, city) = line.split_once(',')?;
    Some(city)
}
```

Compiles and runs:

```rust
use std::fs;
use std::io;

fn main() {
    match read_log("parcels.txt") {
        Ok(t) => print!("{t}"),
        Err(e) => println!("Could not read: {e}"),
    }
}

fn read_log(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}
```

Compiles and runs:

```rust
use std::fs;
use std::io;

fn main() {
    fs::write("parcels.txt", "HP-1 12\n").expect("could not write the file");
    match read_log("parcels.txt") {
        Ok(t) => print!("{t}"),
        Err(e) => println!("Could not read: {e}"),
    }
}

fn read_log(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}
```

## Sources

- Unrecoverable errors with panic: https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html
- Recoverable errors with Result, unwrap, expect and the ? operator: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html
- To panic or not to panic: https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html
- Result type and must_use: https://doc.rust-lang.org/stable/std/result/enum.Result.html
- Termination: main returning Err prints the error and exits with a failure code: https://doc.rust-lang.org/stable/std/process/trait.Termination.html
