use primes::{PrimeSet, Sieve};

const THRESHOLD: usize = 10001;

pub fn solver() -> u64 {
    let mut pset = Sieve::new();

    let mut res: u64 = 0;

    for n in pset.iter().take(THRESHOLD) {
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
