struct Countdown {
    left: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.left)
    }
}

fn main() {
    for n in (Countdown { left: 3 }) {
        println!("{n}");
    }
    let total: u32 = Countdown { left: 4 }.sum();
    println!("{total}");
}
