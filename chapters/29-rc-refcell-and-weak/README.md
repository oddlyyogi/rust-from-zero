# 29 - Rc, RefCell and Weak

What do you do when one owner isn't enough? A shared manifest with a tally counter explains Rc. We clone an Rc and watch the count change, see why shared data is read only, use a RefCell to move the borrowing check to run time and trigger its panic, combine Rc and RefCell, build a real memory leak with a reference cycle, and fix it with Weak.

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
    let manifest = String::from("HP-1, HP-2");
    let van_a = manifest;
    let van_b = manifest;
    println!("{van_a} {van_b}");
}
```

Compiles and runs:

```rust
use std::rc::Rc;

fn main() {
    let manifest = Rc::new(String::from("HP-1, HP-2"));
    let van_a = Rc::clone(&manifest);
    let van_b = Rc::clone(&manifest);
    println!("{van_a} {van_b}");
    println!("{}", Rc::strong_count(&manifest));
}
```

Compiles and runs:

```rust
use std::rc::Rc;

fn main() {
    let manifest = Rc::new(String::from("HP-1, HP-2"));
    let van_a = Rc::clone(&manifest);
    let van_b = Rc::clone(&manifest);
    println!("{van_a} {van_b}");
    println!("{}", Rc::strong_count(&manifest));
    {
        let van_c = Rc::clone(&manifest);
        println!("{}", Rc::strong_count(&van_c));
    }
    println!("{}", Rc::strong_count(&manifest));
}
```

Compiles and runs:

```rust
use std::rc::Rc;

fn main() {
    let manifest = Rc::new(String::from("HP-1, HP-2"));
    let van_a = Rc::clone(&manifest);
    let van_b = Rc::clone(&manifest);
    println!("{van_a} {van_b}");
    println!("{}", Rc::strong_count(&manifest));
    {
        let van_c = Rc::clone(&manifest);
        println!("{}", Rc::strong_count(&van_c));
    }
    println!("{}", Rc::strong_count(&manifest));
    println!("{}", Rc::ptr_eq(&van_a, &van_b));
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::rc::Rc;

fn main() {
    let manifest = Rc::new(String::from("HP-1"));
    manifest.push_str(", HP-2");
}
```

Compiles and runs:

```rust
use std::cell::RefCell;

fn main() {
    let list = RefCell::new(vec![12]);
    list.borrow_mut().push(35);
    println!("{:?}", list.borrow());
}
```

Does not compile, on purpose, so we can read the error:

```rust
use std::cell::RefCell;

fn main() {
    let list = RefCell::new(vec![12]);
    list.borrow_mut().push(35);
    println!("{:?}", list.borrow());
    let first = list.borrow_mut();
    let second = list.borrow_mut();
    println!("{} {}", first.len(), second.len());
}
```

Compiles and runs:

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let manifest = Rc::new(RefCell::new(vec!["HP-1"]));
    let van_a = Rc::clone(&manifest);
    let van_b = Rc::clone(&manifest);
    van_a.borrow_mut().push("HP-2");
    van_b.borrow_mut().push("HP-3");
    println!("{:?}", manifest.borrow());
}
```

Compiles and runs:

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    name: &'static str,
    next: RefCell<Option<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn node(name: &'static str) -> Rc<Node> {
    Rc::new(Node {
        name,
        next: RefCell::new(None),
    })
}

fn main() {
    let a = node("A");
    let b = node("B");
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.next.borrow_mut() = Some(Rc::clone(&a));
    println!("{} {}", Rc::strong_count(&a), Rc::strong_count(&b));
    println!("End of main");
}
```

Compiles and runs:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: &'static str,
    next: RefCell<Option<Weak<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn node(name: &'static str) -> Rc<Node> {
    Rc::new(Node {
        name,
        next: RefCell::new(None),
    })
}

fn main() {
    let a = node("A");
    let b = node("B");
    *a.next.borrow_mut() = Some(Rc::downgrade(&b));
    *b.next.borrow_mut() = Some(Rc::downgrade(&a));
    println!("{} {}", Rc::strong_count(&a), Rc::strong_count(&b));
    println!("End of main");
}
```

Compiles and runs:

```rust
use std::rc::Rc;

fn main() {
    let strong = Rc::new(5);
    let note = Rc::downgrade(&strong);
    println!("{:?}", note.upgrade());
    drop(strong);
    println!("{:?}", note.upgrade());
}
```

## Sources

- Rc, the reference counted smart pointer: https://doc.rust-lang.org/stable/book/ch15-04-rc.html
- RefCell and interior mutability: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html
- Reference cycles can leak memory, Weak: https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html
- std::rc: https://doc.rust-lang.org/stable/std/rc/index.html
- RefCell borrow_mut panics if already borrowed: https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.borrow_mut
- Rc is not Send or Sync: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
