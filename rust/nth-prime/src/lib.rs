pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut num = 3;
    let mut len = 1;

    loop {
        let mut is_prime = true;
        for prime in primes.iter() {
            if num % *prime == 0 {
                is_prime = false;
                break;
            }
            if *prime as f64 > (num as f64).sqrt() {
                break;
            }
        }
        if is_prime {
            primes.push(num);
            len += 1;
        }
        num += 1;
        // println!("{:?}", primes);
        if len >= n + 1 {
            match primes.get(n as usize) {
                Some(the_prime) => {
                    return the_prime.clone();
                }
                None => panic!("Lol"),
            };
        }
    }
}
