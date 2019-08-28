pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| {
            factors
                .iter()
                .filter(|&x| *x != 0)
                .map(|factor| num % factor)
                .any(|x| x == 0)
        })
        .fold(0, |sum, x| sum + x)
}
