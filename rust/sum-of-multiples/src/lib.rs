fn is_mult(n: &u32, factors: &[u32]) -> bool {
    factors.iter().any(|x| *x != 0 && n % x == 0)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).fold(0, |acc, x| if is_mult(&x, factors) { acc + x } else { acc })
}
