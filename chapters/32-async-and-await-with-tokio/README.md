# 32 - async and await with Tokio

How can one thread handle thousands of waiting jobs? A clerk juggling phones on hold explains async. We measure the slow way, add Tokio, see that a future does nothing until awaited, run three jobs in the time of one with join, start ten thousand tasks with spawn, set limits with timeout and select, and fix the classic mistake of blocking inside async code.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo run
cargo add tokio --features full
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
use std::thread;
use std::time::{Duration, Instant};

fn check(name: &str) -> String {
    thread::sleep(Duration::from_millis(100));
    format!("{name} is open")
}

fn main() {
    let start = Instant::now();
    println!("{}", check("Oslo"));
    println!("{}", check("Bergen"));
    println!("{}", check("Tromso"));
    println!("{} ms", start.elapsed().as_millis());
}
```

Compiles and runs:

```rust
use std::time::{Duration, Instant};

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    println!("{}", check("Oslo").await);
    println!("{}", check("Bergen").await);
    println!("{}", check("Tromso").await);
    println!("{} ms", start.elapsed().as_millis());
}
```

Compiles and runs:

```rust
use std::time::Duration;

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    check("Oslo");
    println!("Created it");
}
```

Compiles and runs:

```rust
use std::time::Duration;

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let job = check("Oslo");
    println!("Created it");
    println!("{}", job.await);
}
```

Compiles and runs:

```rust
use std::time::{Duration, Instant};

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(check("Oslo"), check("Bergen"), check("Tromso"));
    println!("{a}, {b}, {c}");
    println!("{} ms", start.elapsed().as_millis());
}
```

Compiles and runs:

```rust
use std::time::{Duration, Instant};

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut handles = Vec::new();
    for i in 0..10_000 {
        handles.push(tokio::spawn(async move { check(&format!("Depot {i}")).await }));
    }
    let mut count = 0;
    for h in handles {
        h.await.unwrap();
        count += 1;
    }
    println!("{count} checks in {} ms", start.elapsed().as_millis());
}
```

Does not compile, on purpose, so we can read the error:

```rust
#[tokio::main]
async fn main() {
    let name = String::from("Oslo");
    let handle = tokio::spawn(async { println!("{name}") });
    handle.await.unwrap();
}
```

Compiles and runs:

```rust
#[tokio::main]
async fn main() {
    let name = String::from("Oslo");
    let handle = tokio::spawn(async move { println!("{name}") });
    handle.await.unwrap();
}
```

Compiles and runs:

```rust
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    for (name, delay) in [("Oslo", 30), ("Bergen", 10), ("Tromso", 20)] {
        let tx = tx.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            tx.send(name).await.unwrap();
        });
    }
    drop(tx);
    while let Some(name) = rx.recv().await {
        println!("{name} answered");
    }
}
```

Compiles and runs:

```rust
use std::time::Duration;

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let result = tokio::time::timeout(Duration::from_millis(50), check("Oslo")).await;
    match result {
        Ok(message) => println!("{message}"),
        Err(_) => println!("Too slow"),
    }
}
```

Compiles and runs:

```rust
use std::time::Duration;

async fn check(name: &str) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let result = tokio::time::timeout(Duration::from_millis(50), check("Oslo")).await;
    match result {
        Ok(message) => println!("{message}"),
        Err(_) => println!("Too slow"),
    }
    tokio::select! {
        a = check("Oslo") => println!("{a}"),
        _ = tokio::time::sleep(Duration::from_millis(10)) => println!("The short sleep won"),
    }
}
```

Compiles and runs:

```rust
use std::thread;
use std::time::{Duration, Instant};

async fn bad_check(name: &str) -> String {
    thread::sleep(Duration::from_millis(100));
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    tokio::join!(bad_check("Oslo"), bad_check("Bergen"), bad_check("Tromso"));
    println!("{} ms", start.elapsed().as_millis());
}
```

Compiles and runs:

```rust
use std::thread;
use std::time::{Duration, Instant};

fn bad_check(name: &str) -> String {
    thread::sleep(Duration::from_millis(100));
    format!("{name} is open")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        tokio::task::spawn_blocking(|| bad_check("Oslo")),
        tokio::task::spawn_blocking(|| bad_check("Bergen")),
        tokio::task::spawn_blocking(|| bad_check("Tromso")),
    );
    println!("{}, {}, {}", a.unwrap(), b.unwrap(), c.unwrap());
    println!("{} ms", start.elapsed().as_millis());
}
```

## Sources

- Async and await in the Rust book: https://doc.rust-lang.org/stable/book/ch17-00-async-await.html
- Futures are lazy: https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html
- Tokio tutorial, spawning tasks, select: https://tokio.rs/tokio/tutorial
- tokio::time::timeout: https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
- tokio::task::spawn_blocking and not blocking the runtime: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
- When not to use Tokio, CPU-bound work: https://tokio.rs/tokio/tutorial
