pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut prime_numbers = vec![2 as u64];
    let mut number_factors = Vec::new();
    let mut divisor: u64 = 3;
    while num % 2 == 0 {
        num /= 2;
        number_factors.push(2 as u64);
    }
    while num != 1 {
        let sqrt_divisor = (divisor as f64).sqrt() as u64;
        if !prime_numbers
            .iter()
            .take_while(|&prime_number| *prime_number <= sqrt_divisor)
            .any(|prime_number| divisor % prime_number == 0)
        {
            prime_numbers.push(divisor);
            while num % divisor == 0 {
                num /= divisor;
                number_factors.push(divisor);
            }
        }
        divisor += 2;
    }
    number_factors
}
