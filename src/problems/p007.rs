// https://projecteuler.net/problem=7

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

use primes::{PrimeSet, Sieve};

const PRIME_NR: usize = 10001;

pub fn solver() -> u64 {
    let mut pset = Sieve::new();

    let mut res: u64 = 0;

    for n in pset.iter().take(PRIME_NR) {
        res = n;
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(7), 104743);
    }
}
