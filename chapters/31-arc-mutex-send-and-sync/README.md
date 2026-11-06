# 31 - Arc, Mutex, Send and Sync

How do threads safely change the same data? A ledger in a locked room with one key explains Mutex. We see why Rc can't cross threads, use Arc, hit the compile error that stops a data race, lock a Mutex and watch the guard unlock it, add up from ten threads, poison a lock on purpose, count with atomics, and find out how Send and Sync make thread safety part of the type.

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
use std::rc::Rc;
use std::thread;

fn main() {
    let shared = Rc::new(12);
    let handle = thread::spawn(move || println!("{shared}"));
    handle.join().unwrap();
}
```

Compiles and runs:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let shared = Arc::new(12);
    let handle = thread::spawn(move || println!("{shared}"));
    handle.join().unwrap();
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let count = Arc::new(0);
    let c = Arc::clone(&count);
    thread::spawn(move || *c += 1);
}
```

Compiles and runs:

```rust
use std::sync::Mutex;

fn main() {
    let ledger = Mutex::new(0);
    {
        let mut n = ledger.lock().unwrap();
        *n += 1;
    }
    println!("{ledger:?}");
}
```

Compiles and runs:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let ledger = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let ledger = Arc::clone(&ledger);
        let handle = thread::spawn(move || {
            let mut n = ledger.lock().unwrap();
            *n += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *ledger.lock().unwrap());
}
```

Compiles and runs:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let ledger = Arc::new(Mutex::new(0));
    let c = Arc::clone(&ledger);
    let h = thread::spawn(move || {
        let _key = c.lock().unwrap();
        panic!("dropped the ledger");
    });
    let _ = h.join();
    println!("{}", ledger.lock().is_err());
}
```

Compiles and runs:

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

fn main() {
    let count = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let c = Arc::clone(&count);
        handles.push(thread::spawn(move || {
            c.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", count.load(Ordering::Relaxed));
}
```

Compiles and runs:

```rust
use std::sync::Arc;

fn needs_send<T: Send>(_value: T) {}

struct Parcel {
    code: String,
}

fn main() {
    let parcel = Parcel { code: String::from("HP-1") };
    println!("{}", parcel.code);
    needs_send(parcel);
    needs_send(Arc::new(5));
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::rc::Rc;
use std::sync::Arc;

fn needs_send<T: Send>(_value: T) {}

struct Parcel {
    code: String,
}

fn main() {
    let parcel = Parcel { code: String::from("HP-1") };
    println!("{}", parcel.code);
    needs_send(parcel);
    needs_send(Arc::new(5));
    needs_send(Rc::new(5));
}
```

Compiles and runs:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let batches = vec![["Oslo", "Bergen"], ["Oslo", "Oslo"], ["Bergen", "Tromso"]];
    let mut handles = Vec::new();
    for batch in batches {
        let counts = Arc::clone(&counts);
        handles.push(thread::spawn(move || {
            for city in batch {
                let mut map = counts.lock().unwrap();
                *map.entry(city).or_insert(0) += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let map = counts.lock().unwrap();
    let mut rows: Vec<_> = map.iter().collect();
    rows.sort();
    println!("{rows:?}");
}
```

## Sources

- Shared-state concurrency, Mutex and Arc: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html
- Send and Sync: https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html
- Mutex poisoning: https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#poisoning
- RwLock: https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html
- Atomics and fetch_add: https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicU32.html
- Arc: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
- Data races are not possible in safe Rust: https://doc.rust-lang.org/nomicon/races.html
