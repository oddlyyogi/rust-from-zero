struct Parcel {
    tracking: String,
    weight: u32,
}

fn main() {
    let parcels = vec![
        Parcel {
            tracking: String::from("HP-1"),
            weight: 12,
        },
        Parcel {
            tracking: String::from("HP-2"),
            weight: 35,
        },
        Parcel {
            tracking: String::from("HP-3"),
            weight: 8,
        },
    ];
    let mut total = 0;
    for p in &parcels {
        total += p.weight;
    }
    println!("{total}");
    let mut heaviest = &parcels[0];
    for p in &parcels {
        if p.weight > heaviest.weight {
            heaviest = p;
        }
    }
    println!("{}", heaviest.tracking);
}
