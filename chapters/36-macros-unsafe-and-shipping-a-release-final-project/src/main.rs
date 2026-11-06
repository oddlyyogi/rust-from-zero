use std::env;

fn price(grams: u32) -> u32 {
    5 + grams / 100
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: harbor_parcels GRAMS...");
        return;
    }
    for arg in args {
        match arg.parse() {
            Ok(grams) => println!("{grams} g costs {}", price(grams)),
            Err(_) => println!("{arg} isn't a number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_a_base_price() {
        assert_eq!(price(1200), 17);
    }
}
