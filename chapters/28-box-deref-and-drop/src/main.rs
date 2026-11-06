struct Locker {
    name: &'static str,
}

impl Drop for Locker {
    fn drop(&mut self) {
        println!("Releasing {}", self.name);
    }
}

fn keep(l: Locker) {
    println!("Keeping {}", l.name);
}

fn main() {
    let a = Locker { name: "A" };
    {
        let b = Locker { name: "B" };
        println!("Inside the block, holding {}", b.name);
    }
    keep(a);
    println!("Back in main");
}
