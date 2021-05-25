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
