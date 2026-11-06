use std::fs;
use std::io;

fn main() {
    fs::write("parcels.txt", "HP-1 12\n").expect("could not write the file");
    match read_log("parcels.txt") {
        Ok(t) => print!("{t}"),
        Err(e) => println!("Could not read: {e}"),
    }
}

fn read_log(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}
