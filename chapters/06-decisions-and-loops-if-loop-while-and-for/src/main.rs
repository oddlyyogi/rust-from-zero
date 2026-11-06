fn main() {
    let shelves = [[4, 9], [7, 2], [5, 1]];
    'search: for shelf in 0..3 {
        for code in shelves[shelf] {
            if code == 7 {
                println!("Found on shelf {shelf}");
                break 'search;
            }
            println!("Checked {code}");
        }
    }
}
