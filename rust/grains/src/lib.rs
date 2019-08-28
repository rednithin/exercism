pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    (2 as u64).pow(s - 1)
}

pub fn total() -> u64 {
    (1..64 + 1)
        .map(|x| square(x))
        .fold(0 as u64, |acc, x| acc + x)
}
