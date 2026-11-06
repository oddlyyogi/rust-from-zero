# 07 - The Stack, the Heap and String

Where does your data actually live? A clerk's desk with a pile of trays and a warehouse out the back explain the stack and the heap. We measure real sizes, watch the order of function calls, overflow the stack on purpose, then see String ask for shelves, outgrow one and move, and find out why text in double quotes cannot grow.

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
    println!("{}", std::mem::size_of::<u8>());
    println!("{}", std::mem::size_of::<i32>());
    println!("{}", std::mem::size_of::<f64>());
    println!("{}", std::mem::size_of::<[i32; 5]>());
}
```

Compiles and runs:

```rust
fn main() {
    println!("main starts");
    load_van();
    println!("main ends");
}

fn load_van() {
    println!("load_van starts");
    weigh();
    println!("load_van ends");
}

fn weigh() {
    println!("weigh runs");
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn dive(depth: u32) {
    let tray = [0u8; 1024];
    if depth % 2000 == 0 {
        println!("Depth {depth}, tray has {} bytes", tray.len());
    }
    dive(depth + 1);
}

fn main() {
    dive(1);
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::new();
    println!("{} {}", label.len(), label.capacity());
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::new();
    println!("{} {}", label.len(), label.capacity());
    label.push_str("Parcel");
    println!("{} {}", label.len(), label.capacity());
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::new();
    println!("{} {}", label.len(), label.capacity());
    label.push_str("Parcel");
    println!("{} {}", label.len(), label.capacity());
    println!("{:p}", label.as_ptr());
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::new();
    println!("{} {}", label.len(), label.capacity());
    label.push_str("Parcel");
    println!("{} {}", label.len(), label.capacity());
    println!("{:p}", label.as_ptr());
    label.push_str(" for the Harbor Parcels depot on Quay Street");
    println!("{} {}", label.len(), label.capacity());
    println!("{:p}", label.as_ptr());
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::new();
    println!("{} {}", label.len(), label.capacity());
    label.push_str("Parcel");
    println!("{} {}", label.len(), label.capacity());
    println!("{:p}", label.as_ptr());
    label.push_str(" for the Harbor Parcels depot on Quay Street");
    println!("{} {}", label.len(), label.capacity());
    println!("{:p}", label.as_ptr());
    let ready = String::with_capacity(100);
    println!("{} {}", ready.len(), ready.capacity());
    println!("{}", std::mem::size_of::<String>());
}
```

Does not compile, on purpose, so we can read the error:

```rust
fn main() {
    let mut label = "Parcel";
    label.push_str(" 7");
    println!("{label}");
}
```

Compiles and runs:

```rust
fn main() {
    let mut label = String::from("Parcel");
    label.push_str(" 7");
    println!("{label}");
}
```

## Sources

- The stack and the heap: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
- String is a pointer, a length and a capacity, and its buffer is on the heap: https://doc.rust-lang.org/stable/std/string/struct.String.html
