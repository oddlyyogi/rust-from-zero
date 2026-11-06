use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    fs::write("bad.txt", "twelve")?;
    println!("{}", load("bad.txt")?);
    Ok(())
}

fn load(path: &str) -> Result<u32, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let w = text.trim().parse::<u32>()?;
    Ok(w)
}
