use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = harbor_parcels::run(&args, "parcels.txt") {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
