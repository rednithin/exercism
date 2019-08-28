fn main() {
    println!(
        "{}",
        5.to_string()
            .chars()
            .map(|x| (x as u32 - 48).pow(2))
            .fold(0, |sum, x| sum + x)
    );
}
