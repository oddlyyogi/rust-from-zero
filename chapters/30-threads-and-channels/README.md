# 30 - Threads and Channels

How do you do several things at once in Rust? Packing benches and a message tube explain threads and channels. We spawn threads and see main end before the worker, wait with join, fix the real move errors, collect results, send messages through an mpsc channel, connect many senders to one receiver and learn why you need drop(tx), and borrow data safely with scoped threads.

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
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=5 {
            println!("worker {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });
    for i in 1..=2 {
        println!("main {i}");
        thread::sleep(Duration::from_millis(10));
    }
}
```

Compiles and runs:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("worker {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });
    for i in 1..=2 {
        println!("main {i}");
        thread::sleep(Duration::from_millis(10));
    }
    handle.join().unwrap();
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::thread;

fn main() {
    let codes = vec!["HP-1", "HP-2"];
    let handle = thread::spawn(|| println!("{codes:?}"));
    handle.join().unwrap();
}
```

Compiles and runs:

```rust
use std::thread;

fn main() {
    let codes = vec!["HP-1", "HP-2"];
    let handle = thread::spawn(move || println!("{codes:?}"));
    handle.join().unwrap();
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::thread;

fn main() {
    let codes = vec!["HP-1", "HP-2"];
    let handle = thread::spawn(move || println!("{codes:?}"));
    handle.join().unwrap();
    println!("{codes:?}");
}
```

Compiles and runs:

```rust
use std::thread;

fn main() {
    let handles: Vec<_> = (0..3)
        .map(|n| thread::spawn(move || n * 10))
        .collect();
    for h in handles {
        println!("{}", h.join().unwrap());
    }
}
```

Compiles and runs:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let slip = String::from("HP-1 packed");
        tx.send(slip).unwrap();
    });
    let got = rx.recv().unwrap();
    println!("{got}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let slip = String::from("HP-1 packed");
        tx.send(slip).unwrap();
        println!("{slip}");
    });
    let got = rx.recv().unwrap();
    println!("{got}");
}
```

Compiles and runs:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for bench in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("Bench {bench} done")).unwrap();
        });
    }
    drop(tx);
    for slip in rx {
        println!("{slip}");
    }
}
```

Compiles and runs:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let parcels = vec![("HP-1", 12), ("HP-2", 35), ("HP-3", 8)];
    for (code, weight) in parcels {
        let tx = tx.clone();
        thread::spawn(move || {
            let cost = 5 + weight * 2;
            tx.send((code, cost)).unwrap();
        });
    }
    drop(tx);
    let mut total = 0;
    for (code, cost) in rx {
        println!("{code} costs {cost}");
        total += cost;
    }
    println!("Total {total}");
}
```

Compiles and runs:

```rust
use std::thread;

fn main() {
    let weights = vec![12, 35, 8];
    thread::scope(|s| {
        s.spawn(|| println!("{}", weights.iter().sum::<i32>()));
        s.spawn(|| println!("{:?}", weights.iter().max()));
    });
    println!("{weights:?}");
}
```

## Sources

- Using threads to run code simultaneously, spawn, join, move: https://doc.rust-lang.org/stable/book/ch16-01-threads.html
- Message passing with channels, mpsc, send takes ownership, cloning the sender: https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html
- std::sync::mpsc, iteration ends when all senders are dropped: https://doc.rust-lang.org/stable/std/sync/mpsc/index.html
- JoinHandle::join returns Err if the thread panicked: https://doc.rust-lang.org/stable/std/thread/struct.JoinHandle.html#method.join
- Scoped threads: https://doc.rust-lang.org/stable/std/thread/fn.scope.html
