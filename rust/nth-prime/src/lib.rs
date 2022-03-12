fn is_prime(n: &u32, primes: &Vec<u32>) -> bool {
    let mut is_prime = true;
    let sqrt_n = (*n as f64).sqrt();

    for i in 0..primes.len() {
        if primes[i] as f64 <= sqrt_n && n % primes[i] == 0 {
            is_prime = false;
            break;
        }
    }
    is_prime
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let size = n as usize;
    let mut primes: Vec<u32> = Vec::with_capacity(size);
    let mut current: u32 = 3;
    primes.push(3);
    let mut i = primes.len();

    while i < size {
        current = current + 2;

        if is_prime(&current, &primes) {
            primes.push(current.clone());
            i = primes.len();
        }
    }
    current
}
