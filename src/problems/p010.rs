// https://projecteuler.net/problem=10

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17. Find the sum of all the primes below two
// million.

use primes::{PrimeSet, Sieve};

const THRESHOLD: u64 = 2000000;

pub fn solver() -> u64 {
    let mut pset = Sieve::new();
    pset.iter().take_while(|x| x < &THRESHOLD).sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(10), 142913828922);
    }
}
