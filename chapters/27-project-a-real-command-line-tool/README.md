# 27 - Project: A Real Command Line Tool

Let's build a real command line tool from scratch. A depot front desk with slips and a ledger explains the design. We read command line arguments, match on the shape of a slice, store parcels in a text file, handle missing files and bad input without a panic, keep main thin and the logic in a tested library, and produce a single fast release binary.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo run -q -- list
cargo run -q -- add HP-1 Oslo 12
cargo run -q -- add HP-2 Bergen 35
cargo run -q -- total
cargo run -q -- add HP-3 Oslo heavy
cargo run -q
cargo test -q
cargo build --release
./target/release/harbor_parcels total
```

## Every version of `src/main.rs` from the video

Compiles and runs:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}
```

Compiles and runs:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{args:?}");
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = harbor_parcels::run(&args, "parcels.txt") {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
```

## Sources

- An I/O project, command line programs: https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
- std::env::args: https://doc.rust-lang.org/stable/std/env/fn.args.html
- Slice patterns: https://doc.rust-lang.org/stable/reference/patterns.html#slice-patterns
- Collecting into a Result stops at the first Err: https://doc.rust-lang.org/stable/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E
- OpenOptions: https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html
- eprintln and standard error: https://doc.rust-lang.org/stable/std/macro.eprintln.html
- Release profile: https://doc.rust-lang.org/stable/cargo/reference/profiles.html
