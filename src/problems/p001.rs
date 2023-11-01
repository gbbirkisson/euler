// https://projecteuler.net/problem=1

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23. Find the sum of all the multiples of 3 or 5 below 1000.

const THRESHOLD: usize = 1000;

pub fn solver() -> usize {
    let res: usize = (0..THRESHOLD)
        .map(|x| match (x % 3, x % 5) {
            (0, _) => x,
            (_, 0) => x,
            (_, _) => 0,
        })
        .sum();
    res
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(1), 233168);
    }
}
