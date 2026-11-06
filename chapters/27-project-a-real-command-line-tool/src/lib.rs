use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};

pub struct Parcel {
    pub code: String,
    pub city: String,
    pub weight: u32,
}

impl Parcel {
    fn to_line(&self) -> String {
        format!("{},{},{}", self.code, self.city, self.weight)
    }

    pub fn from_line(line: &str) -> Result<Parcel, String> {
        let parts: Vec<&str> = line.split(',').collect();
        match parts[..] {
            [code, city, weight] => {
                let weight = weight.parse().map_err(|_| format!("bad line: {line}"))?;
                Ok(Parcel {
                    code: code.to_string(),
                    city: city.to_string(),
                    weight,
                })
            }
            _ => Err(format!("bad line: {line}")),
        }
    }
}

pub enum Command {
    Add(Parcel),
    List,
    Total,
}

pub fn parse_args(args: &[String]) -> Result<Command, String> {
    match args {
        [cmd] if cmd == "list" => Ok(Command::List),
        [cmd] if cmd == "total" => Ok(Command::Total),
        [cmd, code, city, weight] if cmd == "add" => {
            let weight = weight
                .parse()
                .map_err(|_| format!("bad weight: {weight}"))?;
            Ok(Command::Add(Parcel {
                code: code.clone(),
                city: city.clone(),
                weight,
            }))
        }
        _ => Err(String::from("usage: add CODE CITY KG | list | total")),
    }
}

fn load(path: &str) -> Result<Vec<Parcel>, Box<dyn Error>> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let parcels = text
        .lines()
        .map(Parcel::from_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(parcels)
}

fn append(path: &str, p: &Parcel) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", p.to_line())?;
    Ok(())
}

pub fn run(args: &[String], path: &str) -> Result<(), Box<dyn Error>> {
    match parse_args(args)? {
        Command::Add(p) => {
            append(path, &p)?;
            println!("Added {}", p.code);
        }
        Command::List => {
            for p in load(path)? {
                println!("{} to {}, {} kg", p.code, p.city, p.weight);
            }
        }
        Command::Total => {
            let total: u32 = load(path)?.iter().map(|p| p.weight).sum();
            println!("{total} kg");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn bad_weight_is_rejected() {
        assert!(parse_args(&words(&["add", "HP-1", "Oslo", "heavy"])).is_err());
    }

    #[test]
    fn line_round_trip() {
        let line = "HP-1,Oslo,12";
        let p = Parcel::from_line(line).unwrap();
        assert_eq!(p.to_line(), line);
    }

    #[test]
    fn broken_line_is_rejected() {
        assert!(Parcel::from_line("oops").is_err());
    }
}
