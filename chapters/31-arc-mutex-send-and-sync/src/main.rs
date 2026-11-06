use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let batches = vec![["Oslo", "Bergen"], ["Oslo", "Oslo"], ["Bergen", "Tromso"]];
    let mut handles = Vec::new();
    for batch in batches {
        let counts = Arc::clone(&counts);
        handles.push(thread::spawn(move || {
            for city in batch {
                let mut map = counts.lock().unwrap();
                *map.entry(city).or_insert(0) += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let map = counts.lock().unwrap();
    let mut rows: Vec<_> = map.iter().collect();
    rows.sort();
    println!("{rows:?}");
}
