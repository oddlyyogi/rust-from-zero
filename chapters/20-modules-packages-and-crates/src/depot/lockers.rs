pub fn locker_for(weight: u32) -> u32 {
    weight % 30 + 1
}
