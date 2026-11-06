use harbor_parcels::depot::lockers::locker_for;
use harbor_parcels::tracking::label;

fn main() {
    println!("{}", label("HP-1", 12));
    println!("{}", locker_for(12));
}
