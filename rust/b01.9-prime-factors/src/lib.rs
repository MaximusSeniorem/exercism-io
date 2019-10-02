use primes::PrimeSet;

pub fn factors(n: u64) -> Vec<u64> {
   PrimeSet::new()
       .prime_factors(n)
}
