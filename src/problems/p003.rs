// https://projecteuler.net/problem=3

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

const NUMBER: u64 = 600851475143;

pub fn solver() -> u64 {
    let factors = primes::factors_uniq(NUMBER);
    let res: u64 = *factors.last().unwrap();
    res
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(3), 6857);
    }
}
