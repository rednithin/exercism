pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut prime_numbers = vec![2];
    let mut number_factors = Vec::new();
    let mut divisor = 3;
    while num % 2 == 0 {
        num /= 2;
        number_factors.push(2);
    }
    while num != 1 {
        if !prime_numbers
            .iter()
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
