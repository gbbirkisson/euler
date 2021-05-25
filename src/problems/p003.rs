const NUMBER: u64 = 600851475143;

pub fn solver() -> u64 {
    let factors = primes::factors_uniq(NUMBER);
    let res: u64 = *factors.last().unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::Solver;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(3), 6857);
    }
}
