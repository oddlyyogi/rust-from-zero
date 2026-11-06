# 06 - Decisions and Loops: if, loop, while and for

Nearly every program is built from two ideas: making a decision and doing something again. At the Harbor Parcels sorting belt we use if and else, see why conditions must be real booleans, use if as a value, keep trying with loop and break, count down with while, walk safely through weights with for, use ranges, and break out of two loops with a label.

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
    let weight = 35;
    if weight > 30 {
        println!("Heavy lane");
    } else {
        println!("Normal lane");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    if weight > 30 {
        println!("Heavy lane");
    } else if weight > 10 {
        println!("Medium lane");
    } else {
        println!("Normal lane");
    }
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let weight = 12;
    if weight {
        println!("Heavy lane");
    } else if weight > 10 {
        println!("Medium lane");
    } else {
        println!("Normal lane");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weight = 12;
    if weight != 0 {
        println!("Heavy lane");
    } else if weight > 10 {
        println!("Medium lane");
    } else {
        println!("Normal lane");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let fragile = true;
    let fee = if fragile { 5 } else { 2 };
    println!("Fee: {fee}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let fragile = true;
    let fee = if fragile { 5 } else { "two" };
    println!("Fee: {fee}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut attempts = 0;
    loop {
        attempts += 1;
        println!("Attempt {attempts}");
        if attempts == 3 {
            break;
        }
    }
}
```

Compiles and runs:

```rust
fn main() {
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        println!("Attempt {attempts}");
        if attempts == 3 {
            break attempts * 10;
        }
    };
    println!("Result: {result}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut left = 3;
    while left > 0 {
        println!("{left} left");
        left -= 1;
    }
    println!("Belt empty");
}
```

Compiles and runs:

```rust
fn main() {
    let weights = [12, 35, 8, 20];
    for weight in weights {
        println!("{weight}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    for door in 1..4 {
        println!("{door}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    for door in 1..=4 {
        println!("{door}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    for door in (1..=3).rev() {
        println!("{door}");
    }
}
```

Compiles and runs:

```rust
fn main() {
    let weights = [12, 35, 8, 20];
    let mut heavy = 0;
    for weight in weights {
        if weight > 30 {
            heavy += 1;
            continue;
        }
        println!("{weight}: normal lane");
    }
    println!("Heavy parcels: {heavy}");
}
```

Compiles and runs:

```rust
fn main() {
    let shelves = [[4, 9], [7, 2], [5, 1]];
    'search: for shelf in 0..3 {
        for code in shelves[shelf] {
            if code == 7 {
                println!("Found on shelf {shelf}");
                break 'search;
            }
            println!("Checked {code}");
        }
    }
}
```

## Sources

- if, else, loop, while, for, loop labels and break values: https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html
