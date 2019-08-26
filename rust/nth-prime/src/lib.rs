pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut len = 1;

    let is_prime = |num, primes: &Vec<u32>| {
        !primes
            .iter()
            .filter(|&prime| *prime as f64 <= (num as f64).sqrt())
            .map(|&prime| num % prime == 0)
            .any(|x| x == true)
    };
    for num in (3..).step_by(2) {
        if is_prime(num, &primes) {
            primes.push(num);
            len += 1;
        }
        // println!("{:?}", primes);
        if len >= n + 1 {
            if let Some(the_prime) = primes.get(n as usize) {
                return the_prime.clone();
            }
        }
    }
    unreachable!();
}
