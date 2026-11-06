trait Deliver {
    fn deliver(&self, code: &str) -> String;
}

struct Van {
    plate: String,
}

impl Deliver for Van {
    fn deliver(&self, code: &str) -> String {
        format!("Van {} delivered {code}", self.plate)
    }
}

struct Bike {
    rider: String,
}

impl Deliver for Bike {
    fn deliver(&self, code: &str) -> String {
        format!("{} cycled {code} over", self.rider)
    }
}

fn main() {
    let van = Van {
        plate: String::from("V7"),
    };
    let bike = Bike {
        rider: String::from("Sam"),
    };
    let fleet: Vec<Box<dyn Deliver>> = vec![Box::new(van), Box::new(bike)];
    for v in &fleet {
        println!("{}", v.deliver("HP-9"));
    }
}
