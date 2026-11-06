# 17 - HashMap: Find It by Name

How do you find one parcel among thousands without checking them all? A wall of lockers and a rule that turns a tracking code into a locker number explains HashMap. We hit the missing import error, watch the order change between runs, look up keys with get, see a map take ownership of Strings, count things with the entry API, sort keys for a stable report, and switch to BTreeMap when you want order.

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
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    println!("{:?}", weights.get("HP-1"));
    println!("{:?}", weights.get("HP-9"));
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    println!("{:?}", weights.get("HP-1"));
    println!("{:?}", weights.get("HP-9"));
    match weights.get("HP-2") {
        Some(w) => println!("{w} kg"),
        None => println!("not found"),
    }
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    println!("{:?}", weights.get("HP-1"));
    println!("{:?}", weights.get("HP-9"));
    match weights.get("HP-2") {
        Some(w) => println!("{w} kg"),
        None => println!("not found"),
    }
    println!("{}", weights.contains_key("HP-9"));
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::collections::HashMap;

fn main() {
    let code = String::from("HP-1");
    let mut weights = HashMap::new();
    weights.insert(code, 12);
    println!("{code}");
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let code = String::from("HP-1");
    let mut weights = HashMap::new();
    weights.insert(code.clone(), 12);
    println!("{code}");
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let code = String::from("HP-1");
    let mut weights = HashMap::new();
    weights.insert(code.clone(), 12);
    println!("{code}");
    println!("{:?}", weights.get("HP-1"));
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    println!("{:?}", weights.insert("HP-1", 15));
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    println!("{:?}", weights.insert("HP-1", 15));
    println!("{weights:?}");
    let cities = ["Oslo", "Bergen", "Oslo", "Oslo"];
    let mut counts = HashMap::new();
    for city in cities {
        *counts.entry(city).or_insert(0) += 1;
    }
    println!("{:?}", counts.get("Oslo"));
    println!("{:?}", counts.get("Bergen"));
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    for (code, weight) in &weights {
        println!("{code} {weight}");
    }
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    for (code, weight) in &weights {
        println!("{code} {weight}");
    }
    let mut codes: Vec<_> = weights.keys().collect();
    codes.sort();
    for code in codes {
        println!("sorted {code} {}", weights[code]);
    }
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    weights.insert("HP-3", 8);
    println!("{:?}", weights.remove("HP-1"));
    for w in weights.values_mut() { *w += 1; }
    let mut total = 0;
    for w in weights.values() { total += w; }
    println!("{total}");
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

struct Parcel {
    city: String,
    weight: u32,
}

fn main() {
    let mut parcels = HashMap::new();
    parcels.insert(String::from("HP-1"), Parcel { city: String::from("Oslo"), weight: 12 });
    find(&parcels, "HP-1");
    find(&parcels, "HP-9");
}

fn find(parcels: &HashMap<String, Parcel>, code: &str) {
    match parcels.get(code) {
        Some(p) => println!("{} kg to {}", p.weight, p.city),
        None => println!("no such parcel"),
    }
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

struct Parcel {
    city: String,
    weight: u32,
}

fn main() {
    let mut parcels = HashMap::new();
    parcels.insert(String::from("HP-1"), Parcel { city: String::from("Oslo"), weight: 12 });
    find(&parcels, "HP-1");
    find(&parcels, "HP-9");
    if let Some(p) = parcels.get_mut("HP-1") {
        p.weight = 13;
    }
    find(&parcels, "HP-1");
}

fn find(parcels: &HashMap<String, Parcel>, code: &str) {
    match parcels.get(code) {
        Some(p) => println!("{} kg to {}", p.weight, p.city),
        None => println!("no such parcel"),
    }
}
```

Compiles and runs:

```rust
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::new();
    weights.insert("HP-3", 8);
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    println!("{weights:?}");
}
```

Compiles and runs:

```rust
use std::collections::BTreeMap;

fn main() {
    let mut weights = BTreeMap::new();
    weights.insert("HP-3", 8);
    weights.insert("HP-1", 12);
    weights.insert("HP-2", 35);
    println!("{weights:?}");
}
```

## Sources

- Storing keys with associated values in hash maps: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html
- HashMap type, random seed, no iteration order, Index panics on a missing key, insert returns the old value: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
- Entry API and or_insert: https://doc.rust-lang.org/stable/std/collections/hash_map/enum.Entry.html
- Collections performance table, BTreeMap versus HashMap lookups: https://doc.rust-lang.org/stable/std/collections/index.html
- BTreeMap keeps keys sorted: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
